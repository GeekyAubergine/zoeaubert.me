use axum::Router;

use crate::model::AppState;

use self::{api::api_routes, ssr::ssr_routes};

mod api;
mod ssr;

pub fn web_routes() -> Router<AppState> {
    Router::new().merge(ssr_routes()).nest("/api", api_routes())
}
