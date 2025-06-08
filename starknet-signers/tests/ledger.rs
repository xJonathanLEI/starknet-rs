#![cfg(not(target_arch = "wasm32"))]

use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use coins_ledger::{transports::LedgerAsync, APDUAnswer, APDUCommand, LedgerError};
use semver::Version;
use speculos_client::{AutomationAction, AutomationRule, Button, DeviceModel, SpeculosClient};
use starknet_core::types::Felt;
use starknet_signers::ledger::LedgerStarknetApp;

const TEST_PATH: &str = "m/2645'/1195502025'/1470455285'/0'/0'/0";
const APP_PATH: &str = "./test-data/ledger-app/nanox_2.4.2_2.3.1_sdk_v22.10.0";

#[derive(Debug)]
struct SpeculosTransport(Arc<SpeculosClient>);

#[async_trait]
impl LedgerAsync for SpeculosTransport {
    async fn init() -> Result<Self, LedgerError> {
        Ok(Self(Arc::new(
            SpeculosClient::new(DeviceModel::Nanox, 5001, APP_PATH).unwrap(),
        )))
    }

    async fn exchange(&self, packet: &APDUCommand) -> Result<APDUAnswer, LedgerError> {
        let raw_asnwer = self.0.apdu(&packet.serialize()).await.unwrap();
        Ok(APDUAnswer::from_answer(raw_asnwer).unwrap())
    }

    fn close(self) {}
}

fn setup_app(port: u16) -> (Arc<SpeculosClient>, LedgerStarknetApp<SpeculosTransport>) {
    let client = Arc::new(SpeculosClient::new(DeviceModel::Nanox, port, APP_PATH).unwrap());
    let app = LedgerStarknetApp::from_transport(SpeculosTransport(client.clone()));
    (client, app)
}

#[tokio::test]
#[ignore = "requires Speculos installation"]
async fn test_get_app_version() {
    let (_, app) = setup_app(5001);
    let version = app.get_version().await.unwrap();

    assert_eq!(version, Version::new(2, 3, 1));
}

#[tokio::test]
#[ignore = "requires Speculos installation"]
async fn test_get_public_key_headless() {
    let (_, app) = setup_app(5002);
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
    let (client, app) = setup_app(5003);

    tokio::time::sleep(Duration::from_secs(1)).await;

    // Automatically approve
    client
        .automation(&[AutomationRule {
            text: Some("Confirm Public Key".into()),
            regexp: None,
            x: None,
            y: None,
            conditions: &[],
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
        .await
        .unwrap();

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
