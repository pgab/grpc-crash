
use crash::crash_service_client::{CrashServiceClient};
use crash::{CrashRequest as CR};

pub mod crash {
    tonic::include_proto!("crash");
}

use serde::{Deserialize, Serialize};

use tonic::{transport::Channel};


pub struct Client {
    service: crash::crash_service_client::CrashServiceClient<Channel>,
}

impl Client {
    pub async fn new(host: &str, port: u16) -> Result<Self, Error> {
        let connect_string = format!("http://{host}:{port}", host=host, port=port);
        let service =
            CrashServiceClient::connect(connect_string)
                .await
                .map_err(|e| Error {
                    kind: ErrorKind::Internal,
                    message: e.to_string(),
                })?;
        Ok(Self { service })
    }

    pub async fn crash(
        &mut self,
        params: CrashRequest,
    ) -> Result<CrashResponse, Error> {
        let request = tonic::Request::new(CR {
            size: params.size
        });
        let resp = self
            .service
            .crash(request).await
            .map_err(|e| Error {
                kind: ErrorKind::Internal,
                message: e.to_string(),
            })?;
        Ok(CrashResponse { payload: resp.into_inner().payload })
    }


}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ErrorKind {
    NotFound,
    InvalidArgument,
    Internal,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        self.message.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CrashRequest {
    pub size: u64,
}
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CrashResponse {
    pub payload: Vec<u8>,
}

