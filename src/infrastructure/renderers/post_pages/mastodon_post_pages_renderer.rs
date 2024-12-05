use askama::Template;

use crate::domain::{
    models::{mastodon_post::MastodonPost, media::Media, micro_post::MicroPost, page::Page},
    repositories::MastodonPostsRepo,
    services::PageRenderingService,
    state::State,
};

use crate::prelude::*;

use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

#[derive(Template)]
#[template(path = "mastodon_posts/post.html")]
pub struct MastodonPostTemplate {
    page: Page,
    post: MastodonPost,
}

pub async fn render_mastodon_post_pages(state: &impl State) -> Result<()> {
    let mastodon_posts = state.mastodon_posts_repo().find_all_by_date().await?;

    for mastodon_post in mastodon_posts {
        let content = mastodon_post
            .content()
            .replace("<p>", "\n")
            .replace("</p>", "\n");

        let lines = content.lines().collect::<Vec<&str>>();

        let lines = lines
            .iter()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<&str>>();

        let first_line = match lines.first() {
            Some(first) => Some(*first),
            None => None,
        };

        let mut page = Page::new(mastodon_post.slug().clone(), None, first_line)
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
            post: mastodon_post.clone(),
        };

        state
            .page_rendering_service()
            .add_page(
                state,
                template.page.slug.clone(),
                template,
                Some(mastodon_post.updated_at()),
            )
            .await?;
    }

    Ok(())
}
