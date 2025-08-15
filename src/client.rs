use std::ops::Deref;

pub use tondi_grpc_client::{
    GrpcClient,
    rpc_core::{RpcBlock, api::rpc::RpcApi, model::hash::RpcHash},
};

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

    pub async fn get_block(&self, hash: RpcHash) -> Result<RpcBlock> {
        let block = self.inner.get_block(hash, true).await?;
        Ok(block)
    }
}

impl Deref for TondiClient {
    type Target = GrpcClient;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
