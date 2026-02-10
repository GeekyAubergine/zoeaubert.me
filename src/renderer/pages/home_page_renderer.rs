use std::slice::Iter;
use std::str::FromStr;

use askama::Template;
use hypertext::Raw;
use hypertext::prelude::*;
use maud::PreEscaped;
use tracing_subscriber::fmt::format;
use url::Url;

use crate::domain::models::data::Data;
use crate::domain::models::image::Image;
use crate::domain::models::slug::Link;
use crate::domain::models::slug::Slug;
use crate::domain::models::tag::Tag;
use crate::domain::models::timeline_event::TimelineEvent;
use crate::domain::models::timeline_event::TimelineEventPost;
use crate::domain::models::{blog_post::BlogPost, page::Page};
use crate::prelude::*;

use crate::domain::models::media::Media;
use crate::renderer::RendererContext;
use crate::renderer::formatters::format_date::FormatDate;
use crate::renderer::formatters::format_markdown::FormatMarkdown;
use crate::renderer::partials::date::render_date;
use crate::renderer::partials::javascript::home_page_scripts;
use crate::renderer::partials::md::MarkdownMediaOption;
use crate::renderer::partials::md::md;
use crate::renderer::partials::page::PageOptions;
use crate::renderer::partials::page::render_page;
use crate::renderer::partials::tag::render_tags;
use crate::services::file_service::ContentFile;

const BLOG_POSTS_COUNT: usize = 3;
const PHOTOS_COUNT: usize = 10;
const NOTES_BLOG_POST_TO_IGNORE: &str = "MonthlyNotes";

fn blog_post<'l>(post: &'l BlogPost) -> impl Renderable + 'l {
    maud! {
        li class="blog-post-list-item" {
            div class="title-and-date" {
                a href=(&post.slug.relative_string()) {
                    h3 class="title" { (&post.title) }
                }
                (render_date(&post.date))
            }
            p class="prose description" { (post.description )}
        }
    }
}

fn blog_posts<'l>(context: &'l RendererContext) -> impl Renderable + 'l {
    let posts = context
        .data
        .timeline_events
        .all_by_date()
        .iter()
        .filter_map(|event| match event {
            TimelineEvent::Post(post) => match post {
                TimelineEventPost::BlogPost(post) => {
                    if post
                        .tags
                        .iter()
                        .any(|t| t.tag().eq(NOTES_BLOG_POST_TO_IGNORE))
                    {
                        return None;
                    }
                    return Some(post);
                }
                _ => None,
            },
            _ => None,
        })
        .take(BLOG_POSTS_COUNT)
        .collect::<Vec<&BlogPost>>();

    maud! {
        ul class="blog-post-list" {
            @for post in &posts {
                (blog_post(post))
            }
        }
        a class="more-link" href="/blog" {
            ("More blog posts →")
        }
    }
}

fn photo<'l>(photo: &'l Image) -> impl Renderable + 'l {
    maud! {
        @if let Some(l) = &photo.link_on_click {
            li {
                a href=(l) {
                    (photo.render_small())
                }
            }
        } @else {
            li {
                (photo.render_small())
            }
        }
    }
}

fn photos<'l>(context: &'l RendererContext) -> impl Renderable + 'l {
    let photos = context
        .data
        .timeline_events
        .all_by_date()
        .iter()
        .filter_map(|event| match event {
            TimelineEvent::Post(post) => match post {
                TimelineEventPost::BlogPost(_) => None,
                TimelineEventPost::MicroPost(post) => Some(post.media()),
                TimelineEventPost::MastodonPost(post) => Some(post.media()),
            },
            _ => None,
        })
        .flatten()
        .filter_map(|media| match media {
            Media::Image(image) => Some(image),
            _ => None,
        })
        .take(PHOTOS_COUNT)
        .collect::<Vec<&Image>>();

    maud! {
        ul class="photos-list" {
            @for p in &photos {
                (photo(p))
            }
        }
        a class="more-link" href="/photos" {
            ("More photos →")
        }
    }
}

// fn exercise_activity<'l>(context: &'l RendererContext) -> impl Renderable + 'l {
//     let options = BentoBoxOptions {
//         title: "Recent Activity",
//         width: 3,
//         height: None,
//         row: 2,
//         class_name: "exercise-activity",
//     };

//     let posts = context
//         .data
//         .posts
//         .find_all_by_filter_iter(PostFilter::BLOG_POST)
//         .filter_map(|post| match post {
//             Post::BlogPost(post) => Some(post),
//             _ => None,
//         })
//         .take(5)
//         .collect::<Vec<&BlogPost>>();

//     let content = maud! {
//         @for post in &posts {
//             div class="post" {
//                 p { (&post.title) }
//             }
//         }
//     };

//     maud! {
//         BentoBoxComponent options=(&options) content=(&content);
//     }
// }

// fn exercise_stats_monthly<'l>(context: &'l RendererContext) -> impl Renderable + 'l {
//     let options = BentoBoxOptions {
//         title: "This Month",
//         width: 3,
//         height: None,
//         row: 3,
//         class_name: "exercise-monthly",
//     };

//     let posts = context
//         .data
//         .posts
//         .find_all_by_filter_iter(PostFilter::BLOG_POST)
//         .take(5)
//         .filter_map(|post| match post {
//             Post::BlogPost(post) => Some(post),
//             _ => None,
//         })
//         .collect::<Vec<&BlogPost>>();

//     let content = maud! {
//         @for post in &posts {
//             div class="post" {
//                 p { (&post.title) }
//             }
//         }
//     };

//     maud! {
//         BentoBoxComponent options=(&options) content=(&content);
//     }
// }

// fn exercise_stats_yearly<'l>(context: &'l RendererContext) -> impl Renderable + 'l {
//     let options = BentoBoxOptions {
//         title: "This Year",
//         width: 3,
//         height: None,
//         row: 3,
//         class_name: "exercise-yearly",
//     };

//     let posts = context
//         .data
//         .posts
//         .find_all_by_filter_iter(PostFilter::BLOG_POST)
//         .take(5)
//         .filter_map(|post| match post {
//             Post::BlogPost(post) => Some(post),
//             _ => None,
//         })
//         .collect::<Vec<&BlogPost>>();

//     let content = maud! {
//         @for post in &posts {
//             div class="post" {
//                 p { (&post.title) }
//             }
//         }
//     };

//     maud! {
//         BentoBoxComponent options=(&options) content=(&content);
//     }
// }

pub fn render_home_page(context: &RendererContext) -> Result<()> {
    let page = Page::new(Slug::new("/"), None, None);
    let slug = page.slug.clone();

    // BLOG -

    // Sport Recent - Month - Yeah

    let content = maud! {
        section class="header" {
            div class="name-and-cursor" {
                h1 class="typing-name" { ("Zoe Aubert") }
                p class="typing-cursor !opacity-0" {}
            }
            p { ("zo-e o-bear") }
            div class="about" {
                (md(&context.data.about_text.short.to_html(), MarkdownMediaOption::NoMedia))
            }
        }
        section class="blog" {
            div class="width-middle" {
                h2 { ("Blog") }
                (blog_posts(context))
            }
        }
        // section class="toots" {
        //     div class="width-narrow" {
        //         h2 { ("Toots.") }
        //         (blog_posts(context))
        //     }
        // }
        section class="photos" {
            div class="width-middle" {
                h2 { ("Photos") }
                (photos(context))
            }
        }
        // div class="bento home-bento" {
        //     (blog_posts(&context))
        //     (photos(&context))
        //     (exercise_activity(&context))
        //     (exercise_stats_monthly(&context))
        //     (exercise_stats_yearly(&context))
        // }
    };

    let options = PageOptions::new().with_main_class("home");

    let render = render_page(
        &page,
        &options,
        &content,
        home_page_scripts(&context.data.silly_names.silly_names),
    );

    context.renderer.render_page(&slug, &render, None)
}
