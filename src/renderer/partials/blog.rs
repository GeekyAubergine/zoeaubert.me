use crate::{
    domain::models::blog_post::BlogPost,
    prelude::*,
    renderer::{formatters::format_date::FormatDate, partials::tag::tags, RendererContext},
};
use hypertext::prelude::*;

pub fn blog_post_list_item<'l>(post: &'l BlogPost) -> impl Renderable + 'l {
    maud! {
        li class="blog-post-list-item" {
            h3 class="title" { (&post.title) }
            p class="description prose" { (post.description )}
            (tags(&post.tags, Some(3)))
            time class="date" datetime=(post.date.datetime()) {
                (post.date.month_as_word())
            }
        }
    }
}
