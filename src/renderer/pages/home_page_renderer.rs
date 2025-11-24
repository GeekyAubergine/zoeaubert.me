use std::slice::Iter;
use std::str::FromStr;

use askama::Template;
use hypertext::prelude::*;
use hypertext::Raw;
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
use crate::renderer::formatters::format_date::FormatDate;
use crate::renderer::formatters::format_markdown::FormatMarkdown;
use crate::renderer::partials::date::render_date;
use crate::renderer::partials::md::md;
use crate::renderer::partials::md::MarkdownMediaOption;
use crate::renderer::partials::page::render_page;
use crate::renderer::partials::page::PageOptions;
use crate::renderer::partials::tag::render_tags;
use crate::renderer::RendererContext;
use crate::services::file_service::ContentFile;

const RECENT_POSTS_COUNT: usize = 5;
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
        .take(4)
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
        .take(8)
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

    let silly_names_as_string = context
        .data
        .silly_names
        .silly_names
        .iter()
        .map(|n| format!("'{}'", n))
        .collect::<Vec<String>>()
        .join(", ");

    let name_script = format!(
        r#"
    <script type="text/javascript">
        const TIME_BETWEEN_NAME_CHANGES = 5000;

        const TYPING_DELAY_MAX = 200;
        const TYPING_DELAY_MIN = 80;

        const nameElement = document.querySelector('.typing-name');
        const cursorElement = document.querySelector('.typing-cursor');

        // Cursor, split first and last names

        const names = [{}]
        let memory = ['Zoe Aubert'];

        const MEMORY_SIZE = Math.floor(names.length / 2);

        function typingDelay() {{
            return Math.floor(Math.random() * (TYPING_DELAY_MAX - TYPING_DELAY_MIN)) + TYPING_DELAY_MIN;
        }}

        function pickNewName() {{
            let next = names[Math.floor(Math.random() * names.length)];

            while (memory.includes(next)) {{
                next = names[Math.floor(Math.random() * names.length)];
            }}

            memory.push(next);

            memory = memory.slice(-MEMORY_SIZE);

            return next;
        }}

        async function typeName(nextName) {{
            if (!nameElement) {{
                return
            }}

            while (nameElement.innerHTML.length > 0) {{
                nameElement.innerHTML = nameElement
                    .innerHTML
                    .substring(0, nameElement.innerHTML.length - 1);
                await new Promise(resolve => setTimeout(resolve, typingDelay()));
            }}

            await new Promise(resolve => setTimeout(resolve, 500));

            for (let i = 0; i < nextName.length; i++) {{
                nameElement.innerHTML = nextName.substring(0, i + 1);

                await new Promise(resolve => setTimeout(resolve, typingDelay()));
            }}
        }}

        async function changeName() {{
            const nextName = pickNewName();

            await typeName(nextName);

            setTimeout(changeName, TIME_BETWEEN_NAME_CHANGES);
        }}

        async function main() {{
            if (nameElement && cursorElement) {{
                await new Promise(resolve => setTimeout(resolve, 2000));

                cursorElement
                    .classList
                    .remove('!opacity-0')

                setTimeout(changeName, 1500);
            }}
        }}

        main();
    </script>
    "#,
        silly_names_as_string
    );

    let scripts = maud! {
        (Raw::dangerously_create(&name_script))
    };

    let options = PageOptions::new().with_main_class("home");

    let render = render_page(&page, &options, &content, Some(&scripts));

    context.renderer.render_page(&slug, &render, None)
}
