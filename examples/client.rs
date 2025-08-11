use nill::{Nil, nil};
use tondi_client::{
    client::{RpcApi, TondiClient},
    error::Result,
};

#[tokio::main]
async fn main() -> Result<Nil> {
    let client = TondiClient::connect("127.0.0.1:16110").await?;
    let ret = client.get_server_info().await?;
    println!("{ret:?}");
    Ok(nil)
}
