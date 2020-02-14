// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait CrashService {
    fn crash(&self, o: ::grpc::RequestOptions, p: super::crash::CrashRequest) -> ::grpc::SingleResponse<super::crash::CrashResponse>;

    fn stream(&self, o: ::grpc::RequestOptions, p: super::crash::CrashRequest) -> ::grpc::StreamingResponse<super::crash::CrashResponse>;
}

// client

pub struct CrashServiceClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_Crash: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::crash::CrashRequest, super::crash::CrashResponse>>,
    method_Stream: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::crash::CrashRequest, super::crash::CrashResponse>>,
}

impl ::grpc::ClientStub for CrashServiceClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        CrashServiceClient {
            grpc_client: grpc_client,
            method_Crash: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/CrashService/Crash".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Stream: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/CrashService/Stream".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl CrashService for CrashServiceClient {
    fn crash(&self, o: ::grpc::RequestOptions, p: super::crash::CrashRequest) -> ::grpc::SingleResponse<super::crash::CrashResponse> {
        self.grpc_client.call_unary(o, p, self.method_Crash.clone())
    }

    fn stream(&self, o: ::grpc::RequestOptions, p: super::crash::CrashRequest) -> ::grpc::StreamingResponse<super::crash::CrashResponse> {
        self.grpc_client.call_server_streaming(o, p, self.method_Stream.clone())
    }
}

// server

pub struct CrashServiceServer;


impl CrashServiceServer {
    pub fn new_service_def<H : CrashService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/CrashService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/CrashService/Crash".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.crash(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/CrashService/Stream".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.stream(o, p))
                    },
                ),
            ],
        )
    }
}
