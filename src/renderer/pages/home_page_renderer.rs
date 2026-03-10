use std::ops::Deref;

use crate::domain::models::about_text::AboutText;
use crate::domain::models::data::Data;
use crate::domain::models::image::Image;
use crate::domain::models::silly_names::SillyNames;
use crate::domain::models::slug::Slug;
use crate::domain::models::timeline_event::TimelineEvent;
use crate::domain::models::timeline_event::TimelineEventPost;
use crate::domain::models::{blog_post::BlogPost, page::Page};
use crate::prelude::*;
use crate::renderer::RenderTask;
use crate::renderer::RenderTasks;
use crate::services::page_renderer::PageRenderer;
use hypertext::prelude::*;

use crate::domain::models::media::Media;
use crate::renderer::formatters::format_markdown::FormatMarkdown;
use crate::renderer::partials::date::render_date;
use crate::renderer::partials::javascript::home_page_scripts;
use crate::renderer::partials::md::MarkdownMediaOption;
use crate::renderer::partials::md::md;
use crate::renderer::partials::page::PageOptions;
use crate::renderer::partials::page::render_page;

const BLOG_POSTS_COUNT: usize = 3;
const PHOTOS_COUNT: usize = 10;
const NOTES_BLOG_POST_TO_IGNORE: &str = "MonthlyNotes";

fn blog_post(post: &BlogPost) -> impl Renderable {
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

fn blog_posts(posts: &Vec<&BlogPost>) -> impl Renderable {
    maud! {
        ul class="blog-post-list" {
            @for post in posts {
                (blog_post(post))
            }
        }
        a class="more-link" href="/blog" {
            ("All Posts")
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

fn photos(photos: &Vec<&Image>) -> impl Renderable {
    maud! {
        ul class="photos-list" {
            @for p in photos {
                (photo(p))
            }
        }
        a class="more-link" href="/photos" {
            ("All Photos")
        }
    }
}

pub fn render_home_page<'d>(data: &'d Data, render_tasks: &mut RenderTasks<'d>) {
    let posts = data
        .timeline_events
        .all_by_date()
        .iter()
        .filter_map(|event| match event {
            TimelineEvent::Post(TimelineEventPost::BlogPost(post)) => {
                if post
                    .tags
                    .iter()
                    .any(|t| t.tag().eq(NOTES_BLOG_POST_TO_IGNORE))
                {
                    return None;
                }
                Some(post)
            }
            _ => None,
        })
        .take(BLOG_POSTS_COUNT)
        .map(|p| p.deref())
        .collect::<Vec<&BlogPost>>();

    let photos = data
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
        .map(|media| match media {
            Media::Image(image) => image,
        })
        .take(PHOTOS_COUNT)
        .collect::<Vec<&Image>>();

    render_tasks.add(RenderHomePageTask {
        about_text: &data.about_text,
        posts,
        photos,
        silly_names: &data.silly_names,
    });
}

struct RenderHomePageTask<'l> {
    about_text: &'l AboutText,
    posts: Vec<&'l BlogPost>,
    photos: Vec<&'l Image>,
    silly_names: &'l SillyNames,
}

impl<'l> RenderTask for RenderHomePageTask<'l> {
    fn render(self: Box<Self>, renderer: &PageRenderer) -> Result<()> {
        let page = Page::new(Slug::new("/"), None, None);
        let slug = page.slug.clone();

        let silly_names = self.silly_names;

        let content = maud! {
            section class="header" {
                div class="name-and-cursor" {
                    h1 class="typing-name" { ("Zoe Aubert") }
                    p class="typing-cursor !opacity-0" {}
                }
                p { ("zo-e o-bear") }
                div class="about" {
                    (md(&self.about_text.short.to_html(), MarkdownMediaOption::NoMedia))
                }
            }
            section class="blog" {
                div class="width-middle" {
                    a href="/blog" {
                        h2 { ("Blog") }
                    }
                    (blog_posts(&self.posts))
                }
            }
            section class="photos" {
                div class="width-middle" {
                    a href="/photos" {
                        h2 { ("Photos") }
                    }
                    (photos(&self.photos))
                }
            }
        };

        let options = PageOptions::new().with_main_class("home");

        let render = render_page(
            &page,
            &options,
            &content,
            home_page_scripts(&silly_names.silly_names),
        );

        renderer.render_page(&slug, &render, None)
    }
}
