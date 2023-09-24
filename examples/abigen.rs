use starknet::{
    // Note here, we import an ABI type. This applies for
    // ContractAddress, ClassHash, EthAddress only.
    accounts::{ExecutionEncoding, SingleOwnerAccount},
    contract::abi::ContractAddress,
    core::{chain_id, types::FieldElement},
    macros::{abigen, felt},
    providers::{Provider, SequencerGatewayProvider},
    signers::{LocalWallet, SigningKey},
};

use std::sync::Arc;

// TODO: search where this one is missing...!
use starknet::contract::abi::cairo_types;

// Generate the bindings for the contract and also includes
// all the structs and enums present in the ABI with the exact
// same name.
abigen!(TokenContract, "./examples/contracts_abis/mini_erc20.json");

#[tokio::main]
async fn main() {
    let provider = Arc::new(SequencerGatewayProvider::starknet_alpha_goerli());
    let eth_goerli_token_address =
        felt!("0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7");

    let token_contract = TokenContract::new(eth_goerli_token_address, Arc::clone(&provider));

    // To call a view, there is no need to initialize an account. You can directly
    // use the name of the method in the ABI to realize the call.
    let balance: u256 = token_contract
        .balanceOf(&ContractAddress(felt!("YOUR_ACCOUNT_ADDRESS_HEX_HERE")))
        .await
        .expect("Call to get balance failed");

    // For the inputs / outputs of the ABI functions, all the types are
    // defined where the abigen! macro is expanded. Note that `u256`
    // for the balance were already in the scope as it's generated from
    // the ABI.

    // If you want to do some invoke for external functions, you must use an account.
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be("YOUR_PRIVATE_KEY_IN_HEX_HERE").unwrap(),
    ));
    let address = FieldElement::from_hex_be("YOUR_ACCOUNT_CONTRACT_ADDRESS_IN_HEX_HERE").unwrap();
    let mut account = SingleOwnerAccount::new(
        provider,
        signer,
        address,
        chain_id::TESTNET,
        ExecutionEncoding::Legacy,
    );

    let token_contract = token_contract.with_account(Arc::new(account));

    token_contract.approve(
        &ContractAddress(felt!("SPENDER_ADDRESS_HEX")),
        &u256 {
            low: 10000,
            high: 0,
        },
    );

    println!("Your balance: {:?}", balance);
}
