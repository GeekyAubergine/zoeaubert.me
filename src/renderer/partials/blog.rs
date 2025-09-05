use crate::{
    domain::models::{blog_post::BlogPost, slug::Link},
    prelude::*,
    renderer::{
        formatters::format_date::FormatDate,
        partials::{tag::tags, utils::link},
        RendererContext,
    },
};
use hypertext::prelude::*;

pub fn blog_post_list_item<'l>(post: &'l BlogPost) -> impl Renderable + 'l {
    let title = maud! {
        h3 class="title" { (&post.title) }
    };

    maud! {
        li class="blog-post-list-item" {
            div class="title-and-date" {
                (link(&post.slug.as_link(), &title))
                time class="date" datetime=(post.date.datetime()) {
                    (post.date.month_as_word())
                }
            }
            p class="description prose" { (post.description )}
            (tags(&post.tags, Some(3)))
        }
    }
}
