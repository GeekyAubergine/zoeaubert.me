use axum::{extract::State, response::Html, routing::get, Router};
use axum_macros::debug_handler;
use dioxus::prelude::*;
use reqwest::StatusCode;

use crate::model::{mastodon_post::MastodonPostsRepo, AppState};

pub fn ssr_routes() -> Router<AppState> {
    Router::new().route("/", get(hello))
}

async fn hello(
    State(masto): State<MastodonPostsRepo>,
) -> Html<String> {
    println!("masto: {:?}", masto);

    let app: Component = |cx| cx.render(rsx!(div { p {"hefesllo world!"} }));

    let mut vdom = VirtualDom::new(app);
    let _ = vdom.rebuild();

    let text = dioxus_ssr::render(&vdom);

    Html(text)
}