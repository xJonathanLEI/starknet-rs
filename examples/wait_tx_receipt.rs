use eyre::{bail, eyre, Result};
use starknet::{
    accounts::{Account, Call, ConnectedAccount, ExecutionEncoding, SingleOwnerAccount},
    core::{
        chain_id,
        types::{
            BlockId, BlockTag, Felt, FunctionCall, TransactionReceiptWithBlockInfo,
            TransactionStatus, U256,
        },
        utils::get_selector_from_name,
    },
    providers::{
        jsonrpc::{HttpTransport, JsonRpcClient},
        Provider, Url,
    },
    signers::{LocalWallet, SigningKey},
};
use std::env;
use std::time::Duration;

async fn get_account_balance(
    token_contract: Felt,
    account_address: Felt,
    provider: &JsonRpcClient<HttpTransport>,
) -> Result<U256> {
    let felts = provider
        .call(
            FunctionCall {
                contract_address: token_contract,
                entry_point_selector: get_selector_from_name("balanceOf")?,
                calldata: vec![account_address],
            },
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .map_err(|e| eyre!("Error when fetching account balance:\n{e:#?}"))?;
    let low = u128::from_le_bytes(felts[0].to_bytes_le()[0..16].try_into()?);
    let high = u128::from_le_bytes(felts[1].to_bytes_le()[0..16].try_into()?);
    // the unit of data in Cairo is Felt (u252) but ERC20 standard suggests to return u256 from balanceOf
    // So the token contract returns a pair of low (128 bits) and high (128 bits) to construct a u256
    Ok(U256::from_words(low, high))
}

// Wait for the transaction to be accepted
async fn wait_for_transaction(
    provider: &JsonRpcClient<HttpTransport>,
    tx_hash: Felt,
) -> Result<TransactionReceiptWithBlockInfo> {
    let mut retries = 200;
    let retry_interval = Duration::from_millis(3000);

    while retries >= 0 {
        tokio::time::sleep(retry_interval).await; // sleep before the tx status to give some time for a tx get to the provider node
        let status = provider
            .get_transaction_status(tx_hash)
            .await
            .map_err(|e| eyre!("failed to get tx status: {e:#?}"))?;
        retries -= 1;
        match status {
            TransactionStatus::Received => continue,
            TransactionStatus::Rejected => bail!("transaction is rejected"),
            TransactionStatus::AcceptedOnL2(_) | TransactionStatus::AcceptedOnL1(_) => {
                match provider.get_transaction_receipt(tx_hash).await {
                    Ok(receipt) => return Ok(receipt),
                    // For some nodes even though the transaction has execution status SUCCEEDED finality status ACCEPTED_ON_L2,
                    // get_transaction_receipt returns "Transaction hash not found"
                    // see https://github.com/starknet-io/starknet.js/blob/v6.7.0/src/channel/rpc_0_7.ts#L248
                    Err(_) => continue,
                }
            }
        }
    }
    bail!("maximum retries attempts")
}

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_transport = HttpTransport::new(Url::parse(
        "https://free-rpc.nethermind.io/sepolia-juno/v0_7",
    )?);
    let provider = JsonRpcClient::new(rpc_transport);
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(Felt::from_hex(&env::var(
        "ACCOUNT2_PRIVATE_KEY",
    )?)?));
    let account_address = Felt::from_hex(&env::var("ACCOUNT2_ADDRESS")?)?;
    let recepient_address = Felt::from_hex(&env::var("ACCOUNT1_ADDRESS")?)?;
    let token_address =
        Felt::from_hex("0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7")?; // ETH ERC20
    let account_balance = get_account_balance(token_address, account_address, &provider).await?;
    let recepient_balance =
        get_account_balance(token_address, recepient_address, &provider).await?;
    println!("Accounts balances before transfer: {account_balance} WEI, recepient balance: {recepient_balance} WEI");

    let mut account = SingleOwnerAccount::new(
        provider,
        signer,
        account_address,
        chain_id::SEPOLIA,
        ExecutionEncoding::New,
    );
    account.set_block_id(BlockId::Tag(BlockTag::Pending));

    // 0.01 ETH
    let amount = Felt::from_dec_str("10000000000000000")?;

    let transfer_call = Call {
        to: token_address,
        selector: get_selector_from_name("transfer")?,
        calldata: vec![recepient_address, amount, Felt::ZERO],
    };

    let fee = account
        .execute_v1(vec![transfer_call.clone()])
        .estimate_fee()
        .await
        .map_err(|e| eyre!("Error while estimating fee:\n{e:#?}"))?;
    println!("fee etimation: {} WEI", fee.overall_fee);
    let tx = account
        .execute_v1(vec![transfer_call])
        .max_fee(fee.overall_fee + Felt::TWO)
        .send()
        .await
        .map_err(|e| eyre!("Error while estimating fee:\n{e:#?}"))?;
    println!(
        "sent transaction, check status:\nhttps://sepolia.voyager.online/tx/{:#x}",
        tx.transaction_hash
    );
    let receipt = wait_for_transaction(account.provider(), tx.transaction_hash).await?;
    println!("Transaction receipt: {receipt:#?}");
    Ok(())
}
