use std::time::Duration;

use ethers::{
    prelude::{Address, LocalWallet, Middleware, Provider, Signer, TransactionRequest, U256},
    utils::Ganache,
};
use eyre::{ContextCompat, Result};
use hex::ToHex;

#[tokio::main]
async fn main() -> Result<()> {
    // spawn a ganache instance
    let mnemonic = "gas monster ski craft below illegal discover limit dog bundle bus artefact";
    let ganache = Ganache::new().mnemonic(mnemonic).spawn();
    println!("HTTP Endpoint: {}", ganache.endpoint());

    // get the first wallet managed by ganache
    let wallet: LocalWallet = ganache.keys()[0].clone().into();
    let first_address = wallet.address();
    println!(
        "Wallet first address: {}",
        first_address.encode_hex::<String>()
    );

    // a provider is an Ethereum JsonRPC client
    let provider = Provider::try_from(ganache.endpoint())?.interval(Duration::from_millis(10));

    // query the balance of our account
    let first_balance = provider.get_balance(first_address, None).await?;
    println!("Wallet first address balance: {}", first_balance);

    // query the blance of some random account
    let other_address_hex = "0x618192628F36b2bCB101caa3259774793c8379De";
    let other_address = "0x618192628F36b2bCB101caa3259774793c8379De".parse::<Address>()?;
    let other_balance = provider.get_balance(other_address, None).await?;
    println!(
        "Balance for address {}: {}",
        other_address_hex, other_balance
    );

    // create a transaction to transfer 1000 wei to `other_address`
    let tx = TransactionRequest::pay(other_address, U256::from(1000u64)).from(first_address);
    // send the transaction and wait for receipt
    let receipt = provider
        .send_transaction(tx, None)
        .await?
        .log_msg("Pending transfer")
        .confirmations(1) // nnumber of confirmations required
        .await?
        .context("Missing receipt")?;

    println!(
        "TX mined in block {}",
        receipt.block_number.context("Can not get block number")?
    );
    println!(
        "Balance of {} {}",
        other_address_hex,
        provider.get_balance(other_address, None).await?
    );

    Ok(())
}
