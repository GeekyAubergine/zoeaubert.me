use std::net::SocketAddr;

use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use web::ssr::ssr_routes;

mod error;
mod prelude;
mod web;
mod model;

#[tokio::main]
async fn main() {
    let routes = Router::new().merge(ssr_routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}
