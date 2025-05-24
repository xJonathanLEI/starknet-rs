#![cfg(not(target_arch = "wasm32"))]

use std::{
    process::{Child, Command},
    time::Duration,
};

use async_trait::async_trait;
use coins_ledger::{transports::LedgerAsync, APDUAnswer, APDUCommand, LedgerError};
use reqwest::{Client, ClientBuilder};
use semver::Version;
use serde::{Deserialize, Serialize};
use starknet_core::types::Felt;
use starknet_signers::ledger::LedgerStarknetApp;

const TEST_PATH: &str = "m/2645'/1195502025'/1470455285'/0'/0'/0";

#[derive(Debug)]
struct SpeculosTransport {
    process: Child,
    port: u16,
    client: Client,
}

#[derive(Serialize, Deserialize)]
struct ApduData {
    #[serde(with = "hex")]
    data: Vec<u8>,
}

impl SpeculosTransport {
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

    async fn send(&self, packet: &APDUCommand) -> Result<APDUAnswer, LedgerError> {
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
}

#[async_trait]
impl LedgerAsync for SpeculosTransport {
    async fn init() -> Result<Self, LedgerError> {
        Ok(Self::new(5001).await)
    }

    async fn exchange(&self, packet: &APDUCommand) -> Result<APDUAnswer, LedgerError> {
        self.send(packet).await
    }

    fn close(self) {}
}

impl Drop for SpeculosTransport {
    fn drop(&mut self) {
        let _ = self.process.kill();
    }
}

#[tokio::test]
#[ignore = "requires Speculos installation"]
async fn test_get_app_version() {
    let app = LedgerStarknetApp::from_transport(SpeculosTransport::new(5001).await);
    let version = app.get_version().await.unwrap();

    assert_eq!(version, Version::new(2, 3, 1));
}

#[tokio::test]
#[ignore = "requires Speculos installation"]
async fn test_get_public_key_headless() {
    let app = LedgerStarknetApp::from_transport(SpeculosTransport::new(5002).await);
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
