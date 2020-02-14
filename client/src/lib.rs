use crate::crash as proto_gen;
use crate::crash_grpc as grpc_gen;
use grpc::ClientStubExt;
use grpc_gen::{CrashService, CrashServiceClient};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[allow(warnings)]
mod crash;
#[allow(warnings)]
mod crash_grpc;

pub struct Client {
    service: CrashServiceClient,
}

impl Client {
    pub fn new(host: &str, port: u16) -> Result<Self, Error> {
        let service =
            CrashServiceClient::new_plain(host, port, Default::default()).map_err(|e| Error {
                kind: ErrorKind::Internal,
                message: e.to_string(),
            })?;
        Ok(Self { service })
    }

    pub fn crash(&self, params: CrashRequest) -> Result<CrashResponse, Error> {
        let mut request = proto_gen::CrashRequest::new();
        request.set_size(params.size);
        let resp = self.service.crash(Default::default(), request);
        resp.wait_drop_metadata()
            .map_err(map_request_error)
            .map(|mut resp| {
                let payload = resp.take_payload();
                CrashResponse { payload }
            })
    }

    pub fn stream(&self, params: CrashRequest) -> Result<(), Error> {
        let mut req = proto_gen::CrashRequest::new();
        req.set_size(params.size);
        let start_time = SystemTime::now();
        let streamed_resp = self.service.stream(Default::default(), req);
        match streamed_resp.wait() {
            Err(e) => panic!("{:?}", e),
            Ok((_, stream)) => {
                let sum: usize = stream
                    .map(|item| item.unwrap().payload.len() / (1024 * 1024))
                    .sum();
                println!(
                    "This took {:?} for {:?} MiB",
                    start_time.elapsed().unwrap(),
                    sum
                );
            }
        }
        Ok(())
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

fn map_request_error(error: grpc::Error) -> Error {
    match error {
        grpc::Error::GrpcMessage(grpc_error) => {
            let kind = if grpc_error.grpc_status == grpc::GrpcStatus::NotFound as i32 {
                ErrorKind::NotFound
            } else if grpc_error.grpc_status == grpc::GrpcStatus::Argument as i32 {
                ErrorKind::InvalidArgument
            } else {
                ErrorKind::Internal
            };
            let message = grpc_error.grpc_message;
            Error { kind, message }
        }
        _ => Error {
            kind: ErrorKind::Internal,
            message: error.to_string(),
        },
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
