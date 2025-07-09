#![cfg(not(target_arch = "wasm32"))]

use std::{borrow::Cow, sync::Arc};

use async_trait::async_trait;
use coins_ledger::{transports::LedgerAsync, APDUAnswer, APDUCommand, LedgerError};
use speculos_client::{
    AutomationAction, AutomationCondition, AutomationRule, Button, DeviceModel, SpeculosClient,
};
use starknet_core::types::Felt;
use starknet_providers::jsonrpc::{HttpTransport, JsonRpcClient};
use starknet_signers::{ledger::LedgerStarknetApp, LedgerSigner};

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

fn create_jsonrpc_client() -> JsonRpcClient<HttpTransport> {
    let rpc_url = std::env::var("STARKNET_RPC")
        .unwrap_or_else(|_| "https://pathfinder.rpc.sepolia.starknet.rs/rpc/v0_9".into());
    JsonRpcClient::new(HttpTransport::new(url::Url::parse(&rpc_url).unwrap()))
}

/// Module for easy test filtering.
mod ledger {
    use starknet_accounts::{Account, ExecutionEncoding, SingleOwnerAccount};
    use starknet_core::{types::Call, utils::get_selector_from_name};

    use super::*;

    #[tokio::test]
    #[ignore = "requires Speculos installation"]
    async fn test_invoke_v3() {
        let (client, app) = setup_app(6001);
        client
            .automation(&[
                automation::ENABLE_BLIND_SIGN,
                automation::APPROVE_BLIND_SIGN_HASH,
            ])
            .await
            .unwrap();

        let signer = LedgerSigner::new_with_app(TEST_PATH.parse().unwrap(), app).unwrap();
        let provider = create_jsonrpc_client();

        let account = SingleOwnerAccount::new(
            provider,
            signer,
            Felt::from_hex_unchecked(
                "0x01b0f8a1ab14f84573d8ed9eec0852a2099ff76ffb601686ffb14fac352b78b3",
            ),
            starknet_core::chain_id::SEPOLIA,
            ExecutionEncoding::New,
        );

        account
            .execute_v3(vec![Call {
                // STRK
                to: Felt::from_hex_unchecked(
                    "0x04718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d",
                ),
                selector: get_selector_from_name("transfer").unwrap(),
                calldata: vec![account.address(), 100.into(), Felt::ZERO],
            }])
            .send()
            .await
            .unwrap();
    }
}

mod automation {
    use super::*;

    pub const ENABLE_BLIND_SIGN: AutomationRule<'static> = AutomationRule {
        text: None,
        regexp: Some(Cow::Borrowed("^(S)?tarknet$")),
        x: None,
        y: None,
        conditions: &[AutomationCondition {
            varname: Cow::Borrowed("blind_enabled"),
            value: false,
        }],
        actions: &[
            // Right
            AutomationAction::Button {
                button: Button::Right,
                pressed: true,
            },
            AutomationAction::Button {
                button: Button::Right,
                pressed: false,
            },
            // Right
            AutomationAction::Button {
                button: Button::Right,
                pressed: true,
            },
            AutomationAction::Button {
                button: Button::Right,
                pressed: false,
            },
            // Both
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
            // Both
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
            // Left
            AutomationAction::Button {
                button: Button::Left,
                pressed: true,
            },
            AutomationAction::Button {
                button: Button::Left,
                pressed: false,
            },
            // Both
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
            // Mark as done
            AutomationAction::Setbool {
                varname: Cow::Borrowed("blind_enabled"),
                value: true,
            },
        ],
    };

    /// Must be used with [`ENABLE_BLIND_SIGN`].
    pub const APPROVE_BLIND_SIGN_HASH: AutomationRule<'static> = AutomationRule {
        text: None,
        regexp: Some(Cow::Borrowed("^Cancel$")),
        x: None,
        y: None,
        conditions: &[
            AutomationCondition {
                varname: Cow::Borrowed("blind_enabled"),
                value: true,
            },
            AutomationCondition {
                varname: Cow::Borrowed("blind_sign_approved"),
                value: false,
            },
        ],
        actions: &[
            // Right
            AutomationAction::Button {
                button: Button::Right,
                pressed: true,
            },
            AutomationAction::Button {
                button: Button::Right,
                pressed: false,
            },
            // Both
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
            // Right
            AutomationAction::Button {
                button: Button::Right,
                pressed: true,
            },
            AutomationAction::Button {
                button: Button::Right,
                pressed: false,
            },
            // Right
            AutomationAction::Button {
                button: Button::Right,
                pressed: true,
            },
            AutomationAction::Button {
                button: Button::Right,
                pressed: false,
            },
            // Right
            AutomationAction::Button {
                button: Button::Right,
                pressed: true,
            },
            AutomationAction::Button {
                button: Button::Right,
                pressed: false,
            },
            // Both
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
            // Mark as done
            AutomationAction::Setbool {
                varname: Cow::Borrowed("blind_sign_approved"),
                value: true,
            },
        ],
    };
}
