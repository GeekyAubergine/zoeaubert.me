use crate::{build_data::BUILD_DATE, domain::models::page::PagePagination, prelude::*};
use hypertext::prelude::*;
use maud::DOCTYPE;

use crate::domain::models::page::Page;

#[component]
pub fn nav_bar_component<'l>(page: &'l Page) -> impl Renderable + 'l {
    println!("Slug [{:?}]", page.slug.as_str());

    maud! {
        nav
            data-pagefind-ignore
            aria-label="Primary Navigation"
        {
            div class="name-and-toggle"
            {
                a class="name" href="/" {
                    ("Zoe Aubert")
                }
                div class="toggle" {
                    // TODO THEME TOGGLE
                }
            }
            ul class="links" {
                @for link in &page.header_links {
                    li class=(link.url.replace("/", "")) {
                        // @if page.slug.as_str().starts_with(&link.url) {
                        //     a href=(link.url) class="active" { (link.name) }
                        // }
                        // @else {
                            a href=(link.url) { (link.name) }
                        // }
                    }
                }
            }
        }
    }
}

#[component]
pub fn page_pagination_component<'l>(pagination: &'l PagePagination) -> impl Renderable + 'l {
    maud! {
        div class="flex-row w-full justify-between mt-11" {
            div {
                @if let Some(link) = &pagination.previous {
                    a href=(link.url.relative_string())
                        class="secondary items-center under"
                        { (link.label) }
                }
            }
            div {
                @if let Some(link) = &pagination.next {
                    a href=(link.url.relative_string())
                        class="secondary items-center under"
                        { (link.label) }
                }
            }
        }
    }
}

#[component]
pub fn page_base_component<'l>(page: &'l Page, body: &'l dyn Renderable) -> impl Renderable + 'l {
    maud! {
        !DOCTYPE
        html lang={ (page.language) } {
            head {
                meta charset="utf-8";
                link rel="preconnect" href="https://cdn.geekyaubergine.com";

                meta name="viewport" content="width=device-width, initial-scale=1";
                meta name="theme-color" media="(prefers-color-scheme: light" content="#FFFFFF";
                meta name="theme-color" media="(prefers-color-scheme: dark" content="#0C0C0E";

                link rel="icon" type="image/x-icon" href="/assets/img/icon.png";
                link rel="apple-touch-icon" sizes="256x256" href="/assets/img/icon.png";

                link rel="alternate" type="application/rss+xml" title="Zoe Aubert's RSS Feed" href="https://zoeaubert.me/feeds/blog-rss.xml";

                title {(page.title)}

                meta name="title" content=(page.title);
                meta name="og:title" content=(page.title);
                meta name="twitter:title" content=(page.title);

                meta name="description" content=(page.description);
                meta name="og:description" content=(page.description);
                meta name="twitter:description" content=(page.description);

                meta name="og:url" content=(page.permalink());
                meta name="twitter:url" content=(page.permalink());

                meta name="og:image" content=(page.image_url());
                meta name="twitter:image" content=(page.image_url());

                meta name="og:image:alt" content=(page.image_alt());
                meta name="twitter:image:alt" content=(page.image_alt());

                meta name="fediverse:creator" content="@geekyaubergine@social.lol";

                link rel="stylesheet" href={"/assets/css/styles-" BUILD_DATE ".css"};

                script src="https://cdn.usefathom.com/script.js" data-site="XPKVFMEO" defer {}
            }
            body {
                (body)
            }
        }
    }
}

#[component]
pub fn page_component<'l>(page: &'l Page, content: &'l dyn Renderable) -> impl Renderable + 'l {
    let body = maud! {
        NavBarComponent page=(&page);
        main {
            (content)
        }
    };

    maud! {
        PageBaseComponent page=(&page) body=(&body);
    }
}
