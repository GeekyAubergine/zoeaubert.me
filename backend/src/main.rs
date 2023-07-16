use common::app;
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

use std::net::SocketAddr;

use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use web::ssr::ssr_routes;

mod error;
mod model;
mod prelude;
mod web;

#[tokio::main]
async fn main() {
    let routes = Router::new().serve_dioxus_application("", ServeConfigBuilder::new(app, ()));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}
