use std::ops::Deref;

use tondi_grpc_client::GrpcClient;

use crate::error::Result;

#[derive(Debug, Clone)]
pub struct TondiClient {
    inner: GrpcClient,
}

impl TondiClient {
    pub async fn connect(url: &str) -> Result<Self> {
        let url = format!("grpc://{url}");
        let inner = GrpcClient::connect(url).await?;
        Ok(Self { inner })
    }

    pub const fn client(&self) -> &GrpcClient {
        &self.inner
    }
}

impl Deref for TondiClient {
    type Target = GrpcClient;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
