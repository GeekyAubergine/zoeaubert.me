use askama::Template;

use crate::domain::{
    models::{mastodon_post::MastodonPost, media::Media, micro_post::MicroPost, page::Page},
    state::State,
};

use super::render_page_with_template;

use crate::prelude::*;

use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

#[derive(Template)]
#[template(path = "mastodon_posts/post.html")]
pub struct MastodonPostTemplate<'t> {
    page: &'t Page<'t>,
    post: &'t MastodonPost,
}

pub async fn render_mastodon_post_page(state: &impl State, mastodon_post: &MastodonPost) -> Result<()> {
    let page = Page::new(mastodon_post.slug().clone(), None, None)
        .with_date(*mastodon_post.created_at())
        .with_tags(mastodon_post.tags().clone());

    let template = MastodonPostTemplate {
        page: &page,
        post: mastodon_post,
    };

    render_page_with_template(state, &page, template).await
}
