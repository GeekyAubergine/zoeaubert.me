use askama::Template;

use crate::domain::{
    models::{mastodon_post::MastodonPost, media::Media, micro_post::MicroPost, page::Page},
    repositories::MastodonPostsRepo,
    services::PageRenderingService,
    state::State,
};

use crate::prelude::*;

use crate::infrastructure::renderers::formatters_renderer::format_date::FormatDate;
use crate::infrastructure::renderers::formatters_renderer::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters_renderer::format_number::FormatNumber;

#[derive(Template)]
#[template(path = "mastodon_posts/post.html")]
pub struct MastodonPostTemplate {
    page: Page,
    post: MastodonPost,
}

pub async fn render_mastodon_post_pages(state: &impl State) -> Result<()> {
    let mastodon_posts = state.mastodon_posts_repo().find_all_by_date().await?;

    for mastodon_post in mastodon_posts {
        let first_line = mastodon_post.content().lines().next();

        let mut page = Page::new(
            mastodon_post.slug().clone(),
            None,
            first_line,
        )
        .with_date(*mastodon_post.created_at())
        .with_tags(mastodon_post.tags().clone());

        if let Some(first) = mastodon_post.media().first() {
            match first {
                Media::Image(image) => {
                    page = page.with_image(image.clone().into());
                }
            }
        }

        let template = MastodonPostTemplate {
            page,
            post: mastodon_post,
        };

        state
            .page_rendering_service()
            .add_page(state, template.page.slug.clone(), template)
            .await?;
    }

    Ok(())
}
