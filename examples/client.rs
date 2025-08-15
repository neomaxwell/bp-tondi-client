use bp_tondi::{client::TondiClient, error::Result, rpc_core::api::rpc::RpcApi};
use hex_literal::hex;
use nill::{Nil, nil};

#[tokio::main]
async fn main() -> Result<Nil> {
    let client = TondiClient::connect("127.0.0.1:16110").await?;
    let info = client.get_server_info().await?;
    println!("{info:?}");
    let hash = hex!("7ba62b07f55160ff0e737ada5b009bce95b4f173205078e283c029b5c0a3ed23");
    let block = client.get_block(hash.into(), true).await?;
    println!("{block:#?}");
    let hash = hex!("bd5c2985bbceac7670cb56e52b386b2dfeaa216783b2ac6cf728ce6069f8cc73");
    let tx = client.get_transaction(hash.into()).await?;
    println!("{tx:#?}");
    Ok(nil)
}
