use crate::{
    build_data::BUILD_DATE,
    domain::models::{page::PagePaginationData, site_config::SITE_CONFIG},
    prelude::*,
};
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
                        @if page.slug.as_str().starts_with(&link.url) {
                            a
                                href=(link.url)
                                class="active"
                                aria-current="location" {
                                    (link.name)
                                }
                        }
                        @else {
                            a href=(link.url) { (link.name) }
                        }
                    }
                }
            }
        }
    }
}

pub fn page_pagination<'l>(pagination: &'l PagePaginationData) -> impl Renderable + 'l {
    let show_first = pagination.current_index > 2;
    let show_previous = pagination.current_index > 1;

    let last_index = pagination.last.index;

    let show_next = pagination.current_index < last_index;
    let show_last = pagination.current_index < last_index - 1;

    maud! {
        div class="pagination" {
            div class="left" {
                @if show_previous {
                    a
                        href=(pagination.previous.slug.relative_string())
                        class="previous" {
                            ("〈")
                        }
                } @else {
                    div
                        class="spacer previous disabled"
                        aria_disabled {
                            p { ("〈") }
                        }
                }
                @if show_first {
                    a
                        href=(pagination.first.slug.relative_string())
                        class="first" {
                            (pagination.first.index)
                        }
                } @else {
                    div class="spacer first" aria_hidden {}
                }
                @if show_previous {
                    a
                        href=(pagination.previous.slug.relative_string())
                        class="previous" {
                            (pagination.previous.index)
                        }
                } @else {
                    div class="spacer previous" aria_hidden {}
                }
            }
            a
                href="#"
                class="active"
                aria_current="page" {
                (pagination.current_index)
            }
            div class="right" {
                @if show_next {
                    a
                        href=(pagination.next.slug.relative_string())
                        class="next" {
                            (pagination.next.index)
                    }
                } @else {
                    div class="spacer next" aria_hidden {}
                }
                @if show_last {
                    a
                        href=(pagination.last.slug.relative_string())
                        class="last" {
                            (pagination.last.index)
                        }
                } @else {
                    div class="spacer last" aria_hidden {}
                }
                @if show_next {
                    a
                        href=(pagination.next.slug.relative_string())
                        class="next" {
                            ("〉")
                    }
                } @else {
                    div
                        class="spacer next disabled"
                        aria_disabled {
                            p { ("〉") }
                        }
                }
            }
        }
    }
}

#[component]
fn page_base_component<'l>(page: &'l Page, body: &'l dyn Renderable) -> impl Renderable + 'l {
    let title = match &page.title {
        Some(t) => format!("{} | {}", t, SITE_CONFIG.title),
        None => SITE_CONFIG.title.clone(),
    };

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

                meta name="title" content=(title);
                meta name="og:title" content=(title);
                meta name="twitter:title" content=(title);

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

pub enum PageWidth {
    Wide,
    Mid,
    Narrow,
}

impl PageWidth {
    pub fn wide() -> Self {
        PageWidth::Wide
    }

    pub fn mid() -> Self {
        PageWidth::Mid
    }

    pub fn narrow() -> Self {
        PageWidth::Narrow
    }
}

pub struct PageOptions {
    width: PageWidth,
}

impl PageOptions {
    pub fn new() -> Self {
        Self {
            width: PageWidth::Narrow,
        }
    }

    pub fn with_width(mut self, width: PageWidth) -> Self {
        self.width = width;
        self
    }
}

#[component]
pub fn page_component<'l>(
    page: &'l Page,
    options: &'l PageOptions,
    content: &'l dyn Renderable,
) -> impl Renderable + 'l {
    let main_class = match options.width {
        PageWidth::Narrow => "width-narrow",
        PageWidth::Mid => "width-middle",
        PageWidth::Wide => "width-wide",
    };

    let body = maud! {
        NavBarComponent page=(&page);
        main class=(main_class) {
            @if let Some(title) = &page.title {
                h1 { (title) }
            }
            (content)
            @if let Some(pagination) = &page.page_pagination {
                (page_pagination(pagination))
            }
        }
    };

    maud! {
        PageBaseComponent page=(&page) body=(&body);
    }
}
