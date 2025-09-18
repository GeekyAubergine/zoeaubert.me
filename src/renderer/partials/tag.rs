use crate::{domain::models::tag::Tag, prelude::*};
use hypertext::prelude::*;

pub fn tags<'l>(tags: &'l Vec<Tag>, limit: Option<usize>) -> impl Renderable + 'l {
    let limit = limit.unwrap_or(tags.len());

    maud! {
        @if tags.len() > 0 {
            ul class="tags-list" data-nosnippet {
                @for tag in tags.iter().take(limit) {
                    li {
                        a href=(format!("/tags/{}", tag.slug())) class="tag" {
                            (format!("#{}", tag.tag()))
                        }
                    }
                }
            }
        }
    }
}
