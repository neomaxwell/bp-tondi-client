use std::io::Error as StdIoError;

use thiserror::Error;
use tondi_grpc_client::{error::Error as GrpcClientError, rpc_core::RpcError as RpcCoreError};

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    StdIoError(#[from] StdIoError),

    #[error(transparent)]
    RpcCoreError(#[from] RpcCoreError),

    #[error(transparent)]
    GrpcClientError(#[from] GrpcClientError),

    #[error("{0}")]
    Generic(String),
}

#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {
        Err($crate::error::Error::Generic(format!($($arg)*)))
    }
}

#[allow(unused_imports)]
pub(crate) use err;

pub type Result<T, E = Error> = std::result::Result<T, E>;
