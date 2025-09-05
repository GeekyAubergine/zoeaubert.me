use std::slice::Iter;
use std::str::FromStr;

use askama::Template;
use hypertext::prelude::*;
use url::Url;

use crate::domain::models::data::Data;
use crate::domain::models::image::Image;
use crate::domain::models::post::Post;
use crate::domain::models::post::PostFilter;
use crate::domain::models::slug::Slug;
use crate::domain::models::{blog_post::BlogPost, page::Page};
use crate::prelude::*;

use crate::domain::models::media::Media;
use crate::renderer::formatters::format_date::FormatDate;
use crate::renderer::partials::bento::BentoBoxComponent;
use crate::renderer::partials::bento::BentoBoxOptions;
use crate::renderer::partials::page::PageComponent;
use crate::renderer::partials::page::PageOptions;
use crate::renderer::partials::utils::link;
use crate::renderer::RendererContext;
use crate::services::file_service::ContentFile;

const RECENT_POSTS_COUNT: usize = 5;

fn blog_post<'l>(post: &'l BlogPost) -> impl Renderable + 'l {
    maud! {
        li class="blog-post-list-item" {
            div class="title-and-date" {
                h3 class="title" { (&post.title) }
                time class="date" datetime=(post.date.datetime()) {
                    (post.date.without_time())
                }
            }
            p class="description prose" { (post.description )}
        }
    }
}

fn blog_posts<'l>(context: &'l RendererContext) -> impl Renderable + 'l {
    let options = BentoBoxOptions {
        title: "Blog",
        width: 6,
        height: Some(2),
        row: 1,
        class_name: "blog",
    };

    let posts = context
        .data
        .posts
        .find_all_by_filter_iter(PostFilter::BLOG_POST)
        .take(5)
        .filter_map(|post| match post {
            Post::BlogPost(post) => Some(post),
            _ => None,
        })
        .collect::<Vec<&BlogPost>>();

    let content = maud! {
        @for post in &posts {
            ul class="blog-post-list" {
                (blog_post(post))
            }
        }
    };

    maud! {
        BentoBoxComponent options=(&options) content=(&content);
    }
}

fn photos<'l>(context: &'l RendererContext) -> impl Renderable + 'l {
    let options = BentoBoxOptions {
        title: "Photos",
        width: 3,
        height: None,
        row: 1,
        class_name: "photos",
    };

    let photos = context
        .data
        .posts
        .find_all_by_filter_iter(PostFilter::filter_photos_page())
        .flat_map(|post| post.media())
        .filter_map(|media| match media {
            Media::Image(image) => Some(image),
        })
        .take(5)
        .collect::<Vec<Image>>();

    let content = maud! {
        @for photo in &photos {
            @match &photo.link_on_click {
                Some(l) => (link(&l.as_link(), &photo.render_tiny())),
                None => (photo.render_tiny()),
            }
        }
    };

    maud! {
        BentoBoxComponent options=(&options) content=(&content);
    }
}

fn exercise_activity<'l>(context: &'l RendererContext) -> impl Renderable + 'l {
    let options = BentoBoxOptions {
        title: "Recent Activity",
        width: 3,
        height: None,
        row: 2,
        class_name: "exercise-activity",
    };

    let posts = context
        .data
        .posts
        .find_all_by_filter_iter(PostFilter::BLOG_POST)
        .take(5)
        .filter_map(|post| match post {
            Post::BlogPost(post) => Some(post),
            _ => None,
        })
        .collect::<Vec<&BlogPost>>();

    let content = maud! {
        @for post in &posts {
            div class="post" {
                p { (&post.title) }
            }
        }
    };

    maud! {
        BentoBoxComponent options=(&options) content=(&content);
    }
}

fn exercise_stats_monthly<'l>(context: &'l RendererContext) -> impl Renderable + 'l {
    let options = BentoBoxOptions {
        title: "This Month",
        width: 3,
        height: None,
        row: 3,
        class_name: "exercise-monthly",
    };

    let posts = context
        .data
        .posts
        .find_all_by_filter_iter(PostFilter::BLOG_POST)
        .take(5)
        .filter_map(|post| match post {
            Post::BlogPost(post) => Some(post),
            _ => None,
        })
        .collect::<Vec<&BlogPost>>();

    let content = maud! {
        @for post in &posts {
            div class="post" {
                p { (&post.title) }
            }
        }
    };

    maud! {
        BentoBoxComponent options=(&options) content=(&content);
    }
}

fn exercise_stats_yearly<'l>(context: &'l RendererContext) -> impl Renderable + 'l {
    let options = BentoBoxOptions {
        title: "This Year",
        width: 3,
        height: None,
        row: 3,
        class_name: "exercise-yearly",
    };

    let posts = context
        .data
        .posts
        .find_all_by_filter_iter(PostFilter::BLOG_POST)
        .take(5)
        .filter_map(|post| match post {
            Post::BlogPost(post) => Some(post),
            _ => None,
        })
        .collect::<Vec<&BlogPost>>();

    let content = maud! {
        @for post in &posts {
            div class="post" {
                p { (&post.title) }
            }
        }
    };

    maud! {
        BentoBoxComponent options=(&options) content=(&content);
    }
}

fn blog_posts_6<'l>(context: &'l RendererContext) -> impl Renderable + 'l {
    let options = BentoBoxOptions {
        title: "Blog",
        width: 3,
        height: None,
        row: 3,
        class_name: "photos",
    };

    let posts = context
        .data
        .posts
        .find_all_by_filter_iter(PostFilter::BLOG_POST)
        .take(5)
        .filter_map(|post| match post {
            Post::BlogPost(post) => Some(post),
            _ => None,
        })
        .collect::<Vec<&BlogPost>>();

    let content = maud! {
        @for post in &posts {
            div class="post" {
                p { (&post.title) }
            }
        }
    };

    maud! {
        BentoBoxComponent options=(&options) content=(&content);
    }
}

pub async fn render_home_page(context: &RendererContext) -> Result<()> {
    let page = Page::new(Slug::new("/"), None, None);
    let slug = page.slug.clone();

    // BLOG -

    // Sport Recent - Month - Yeah

    let content = maud! {
        // div class="bento home-bento" {
        //     (blog_posts(&context))
        //     (photos(&context))
        //     (exercise_activity(&context))
        //     (exercise_stats_monthly(&context))
        //     (exercise_stats_yearly(&context))
        // }
    };

    let options = PageOptions::new();

    let renderer = maud! {
        PageComponent page=(&page) options=(&options) content=(&content);
    };

    context.renderer.render_page(&slug, &renderer, None)
}
