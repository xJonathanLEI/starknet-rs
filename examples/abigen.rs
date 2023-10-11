use starknet::{
    // Note here, we import an ABI type. This applies for
    // ContractAddress, ClassHash, EthAddress only.
    accounts::{ExecutionEncoding, SingleOwnerAccount},
    contract::abi::ContractAddress,
    core::{chain_id, types::FieldElement},
    macros::abigen,
    providers::SequencerGatewayProvider,
    signers::{LocalWallet, SigningKey},
};

// Generate the bindings for the contract and also includes
// all the structs and enums present in the ABI with the exact
// same name.
abigen!(TokenContract, "./examples/contracts_abis/mini_erc20.json");

#[tokio::main]
async fn main() {
    let provider = SequencerGatewayProvider::starknet_alpha_goerli();
    println!("provider {:?}", provider);
    let eth_goerli_token_address = FieldElement::from_hex_be(
        "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
    )
    .unwrap();

    // If you only plan to call views functions, you can use the `Reader`, which
    // only requires a provider along with your contract address.
    let token_contract = TokenContractReader::new(eth_goerli_token_address, &provider);

    // To call a view, there is no need to initialize an account. You can directly
    // use the name of the method in the ABI to realize the call.
    let balance: u256 = token_contract
        .balanceOf(&ContractAddress(
            FieldElement::from_hex_be("YOUR_HEX_CONTRACT_ADDRESS_HERE").unwrap(),
        ))
        .await
        .expect("Call to get balance failed");

    println!("Your ETH (goerli) balance: {:?}", balance);

    // For the inputs / outputs of the ABI functions, all the types are
    // defined where the abigen! macro is expanded. Note that `u256`
    // for the balance were already in the scope as it's generated from
    // the ABI.

    // If you want to do some invoke for external functions, you must use an account.
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be("YOUR_PRIVATE_KEY_IN_HEX_HERE").unwrap(),
    ));
    let address = FieldElement::from_hex_be("YOUR_ACCOUNT_CONTRACT_ADDRESS_IN_HEX_HERE").unwrap();
    let account = SingleOwnerAccount::new(
        provider,
        signer,
        address,
        chain_id::TESTNET,
        ExecutionEncoding::Legacy,
    );

    // The `TokenContract` also contains a reader field that you can use if you need both
    // to call external and views with the same instance.
    let token_contract = TokenContract::new(eth_goerli_token_address, &account);

    // Example here of querying again the balance, using the internal reader of the
    // contract setup with an account.
    token_contract
        .reader
        .balanceOf(&ContractAddress(
            FieldElement::from_hex_be("YOUR_HEX_CONTRACT_ADDRESS_HERE").unwrap(),
        ))
        .await
        .expect("Call to get balance failed");

    let _ = token_contract
        .approve(
            &ContractAddress(FieldElement::from_hex_be("SPENDER_ADDRESS_HEX").unwrap()),
            &u256 {
                low: 10000,
                high: 0,
            },
        )
        .await;
}
