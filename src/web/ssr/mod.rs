use axum::{response::Html, routing::get, Router};
use dioxus::prelude::*;

pub fn ssr_routes() -> Router {
    Router::new().route("/", get(hello))
}

async fn hello() -> Html<String> {
    let app: Component = |cx| cx.render(rsx!(div { p {"hello world!"} Button {} }));

    let mut vdom = VirtualDom::new(app);
    let _ = vdom.rebuild();

    let text = dioxus_ssr::render(&vdom);

    Html(text)
}

fn Button(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 0);

    cx.render(rsx! {
        h1 { "High-Five counter: {count}" }
        button {
            onclick: move |_| {
                // changing the count will cause the component to re-render
                count += 1
            },
            "Up high!"
        }
        button {
            onclick: move |_| {
                // changing the count will cause the component to re-render
                count -= 1
            },
            "Down low!"
        }
    })
}
