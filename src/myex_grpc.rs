// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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


// server interface

pub trait MyExService {
    fn record_cab_location(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::myex::CabLocationRequest>, resp: ::grpc::ServerResponseUnarySink<super::myex::CabLocationResponse>) -> ::grpc::Result<()>;

    fn get_cab(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::myex::GetCabRequest>, resp: ::grpc::ServerResponseUnarySink<super::myex::GetCabResponse>) -> ::grpc::Result<()>;
}

// client

pub struct MyExServiceClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
}

impl ::grpc::ClientStub for MyExServiceClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        MyExServiceClient {
            grpc_client: grpc_client,
        }
    }
}

impl MyExServiceClient {
    pub fn record_cab_location(&self, o: ::grpc::RequestOptions, req: super::myex::CabLocationRequest) -> ::grpc::SingleResponse<super::myex::CabLocationResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/myex.MyExService/record_cab_location"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn get_cab(&self, o: ::grpc::RequestOptions, req: super::myex::GetCabRequest) -> ::grpc::SingleResponse<super::myex::GetCabResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/myex.MyExService/get_cab"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }
}

// server

pub struct MyExServiceServer;


impl MyExServiceServer {
    pub fn new_service_def<H : MyExService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/myex.MyExService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/myex.MyExService/record_cab_location"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).record_cab_location(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/myex.MyExService/get_cab"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).get_cab(ctx, req, resp))
                    },
                ),
            ],
        )
    }
}
