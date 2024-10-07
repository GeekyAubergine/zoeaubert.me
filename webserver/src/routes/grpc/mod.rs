use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use axum::Router;
use dotenvy_macro::dotenv;
use micro_posts_grpc_service::MicroPostsGrpcService;
use shared::{
    make_reflection_descriptor_service,
    zoeaubert_proto::webserver::{
        micro_posts_server::MicroPostsServer, silly_names_server::SillyNamesServer,
    },
};
use silly_names_grpc_service::SillyNamesGprcService;
use tonic::{body::boxed, service::Routes, transport::Server};
use tower::ServiceBuilder;
use tracing::{error, info};

use crate::infrastructure::app_state::AppState;

pub mod micro_posts_grpc_service;
pub mod silly_names_grpc_service;

// pub mod zoeaubert_proto {
//     tonic::include_proto!("zoeaubert");
// }

pub fn build_grpc_service(state: &AppState) {
    let state = state.clone();
    tokio::task::spawn(async move {
        info!("Starting gRPC server");

        let addr = SocketAddrV4::new(
            Ipv4Addr::new(0, 0, 0, 0),
            dotenv!("GRPC_PORT").parse::<u16>().unwrap(),
        )
        .into();

        info!("gRPC server address {:?}", addr);

        let reflection_service = match make_reflection_descriptor_service() {
            Ok(service) => service,
            Err(e) => {
                error!("Failed to create reflection service {:?}", e);
                return;
            }
        };

        match Server::builder()
            .add_service(reflection_service)
            .add_service(SillyNamesServer::new(SillyNamesGprcService::new(
                state.clone(),
            )))
            .add_service(MicroPostsServer::new(MicroPostsGrpcService::new(
                state.clone(),
            )))
            .serve(addr)
            .await
        {
            Ok(_) => info!("gRPC server started"),
            Err(e) => error!("gRPC server failed {:?}", e),
        }
    });

    // let addr = "[::1]:50051".parse()?;

    // Server::builder()
    //     .add_service(SillyNamesServer::new(SillyNamesGprcService::new(
    //         state.clone(),
    //     )))
    //     .serve(addr)
    //     .await?;

    // .into_service()
    // .map_response(|r| r.map(boxed(body)))
    // .boxed_clone()
}
