#![cfg(not(target_arch = "wasm32"))]

use std::{
    process::{Child, Command},
    sync::Arc,
    time::Duration,
};

use async_trait::async_trait;
use coins_ledger::{transports::LedgerAsync, APDUAnswer, APDUCommand, LedgerError};
use reqwest::{Client, ClientBuilder};
use semver::Version;
use serde::{ser::SerializeSeq, Deserialize, Serialize};
use starknet_core::types::Felt;
use starknet_signers::ledger::LedgerStarknetApp;

const TEST_PATH: &str = "m/2645'/1195502025'/1470455285'/0'/0'/0";

#[derive(Debug)]
struct SpeculosTransport(Arc<SpeculosClient>);

#[derive(Debug)]
struct SpeculosClient {
    process: Child,
    port: u16,
    client: Client,
}

#[derive(Serialize, Deserialize)]
struct ApduData {
    #[serde(with = "hex")]
    data: Vec<u8>,
}

#[derive(Debug, Serialize)]
struct AutomationRequest {
    version: u32,
    rules: &'static [AutomationRule],
}

#[derive(Debug, Serialize)]
struct AutomationRule {
    text: &'static str,
    actions: &'static [AutomationAction],
}

#[derive(Debug)]
enum AutomationAction {
    Button { button: Button, pressed: bool },
}

#[derive(Debug)]
enum Button {
    Left,
    Right,
}

impl SpeculosClient {
    async fn new(port: u16) -> Self {
        let mut cmd = Command::new("speculos");
        let process = cmd
            .args([
                "--api-port",
                &port.to_string(),
                "--apdu-port",
                "0",
                "-m",
                "nanox",
                "--display",
                "headless",
                "./test-data/ledger-app/nanox_2.4.2_2.3.1_sdk_v22.10.0",
            ])
            .spawn()
            .expect("Unable to spawn speculos process");

        // Wait for process to be ready (flaky)
        tokio::time::sleep(Duration::from_secs(1)).await;

        Self {
            process,
            port,
            client: ClientBuilder::new()
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap(),
        }
    }

    async fn apdu(&self, packet: &APDUCommand) -> Result<APDUAnswer, LedgerError> {
        let response = self
            .client
            .post(format!("http://localhost:{}/apdu", self.port))
            .json(&ApduData {
                data: packet.serialize(),
            })
            .send()
            .await
            .unwrap();

        let body = response.json::<ApduData>().await.unwrap();
        APDUAnswer::from_answer(body.data)
    }

    async fn automation(&self, rules: &'static [AutomationRule]) {
        let response = self
            .client
            .post(format!("http://localhost:{}/automation", self.port))
            .json(&AutomationRequest { version: 1, rules })
            .send()
            .await
            .unwrap();

        if !response.status().is_success() {
            panic!("Response status code: {}", response.status());
        }
    }
}

#[async_trait]
impl LedgerAsync for SpeculosTransport {
    async fn init() -> Result<Self, LedgerError> {
        Ok(Self(Arc::new(SpeculosClient::new(5001).await)))
    }

    async fn exchange(&self, packet: &APDUCommand) -> Result<APDUAnswer, LedgerError> {
        self.0.apdu(packet).await
    }

    fn close(self) {}
}

impl Drop for SpeculosClient {
    fn drop(&mut self) {
        let _ = self.process.kill();
    }
}

impl Serialize for AutomationAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Button { button, pressed } => {
                let mut seq = serializer.serialize_seq(Some(3))?;

                seq.serialize_element("button")?;
                seq.serialize_element(&match button {
                    Button::Left => 1,
                    Button::Right => 2,
                })?;
                seq.serialize_element(pressed)?;

                seq.end()
            }
        }
    }
}

#[tokio::test]
#[ignore = "requires Speculos installation"]
async fn test_get_app_version() {
    let client = Arc::new(SpeculosClient::new(5001).await);
    let app = LedgerStarknetApp::from_transport(SpeculosTransport(client.clone()));
    let version = app.get_version().await.unwrap();

    assert_eq!(version, Version::new(2, 3, 1));
}

#[tokio::test]
#[ignore = "requires Speculos installation"]
async fn test_get_public_key_headless() {
    let client = Arc::new(SpeculosClient::new(5002).await);
    let app = LedgerStarknetApp::from_transport(SpeculosTransport(client.clone()));
    let public_key = app
        .get_public_key(TEST_PATH.parse().unwrap(), false)
        .await
        .unwrap();

    assert_eq!(
        public_key.scalar(),
        Felt::from_hex_unchecked(
            "0x07427aa749c4fc98a5bf76f037eb3c61e7b4793b576a72d45a4b52c5ded997f2"
        )
    );
}

#[tokio::test]
#[ignore = "requires Speculos installation"]
async fn test_get_public_key_with_confirmation() {
    let client = Arc::new(SpeculosClient::new(5003).await);

    // Automatically approve
    client
        .automation(&[AutomationRule {
            text: "Confirm Public Key",
            actions: &[
                // Press right
                AutomationAction::Button {
                    button: Button::Right,
                    pressed: true,
                },
                AutomationAction::Button {
                    button: Button::Right,
                    pressed: false,
                },
                // Press right
                AutomationAction::Button {
                    button: Button::Right,
                    pressed: true,
                },
                AutomationAction::Button {
                    button: Button::Right,
                    pressed: false,
                },
                // Press right
                AutomationAction::Button {
                    button: Button::Right,
                    pressed: true,
                },
                AutomationAction::Button {
                    button: Button::Right,
                    pressed: false,
                },
                // Press both
                AutomationAction::Button {
                    button: Button::Left,
                    pressed: true,
                },
                AutomationAction::Button {
                    button: Button::Right,
                    pressed: true,
                },
                AutomationAction::Button {
                    button: Button::Left,
                    pressed: false,
                },
                AutomationAction::Button {
                    button: Button::Right,
                    pressed: false,
                },
            ],
        }])
        .await;

    let app = LedgerStarknetApp::from_transport(SpeculosTransport(client.clone()));
    let public_key = app
        .get_public_key(TEST_PATH.parse().unwrap(), true)
        .await
        .unwrap();

    assert_eq!(
        public_key.scalar(),
        Felt::from_hex_unchecked(
            "0x07427aa749c4fc98a5bf76f037eb3c61e7b4793b576a72d45a4b52c5ded997f2"
        )
    );
}
