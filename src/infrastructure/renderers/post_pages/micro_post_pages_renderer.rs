use askama::Template;

use crate::domain::{
    models::{media::Media, micro_post::MicroPost, page::Page},
    repositories::MicroPostsRepo,
    services::PageRenderingService,
    state::State,
};

use crate::prelude::*;

use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

#[derive(Template)]
#[template(path = "micro_posts/post.html")]
pub struct MicroPostTemplate {
    page: Page,
    post: MicroPost,
}

pub async fn render_micro_post_pages(state: &impl State) -> Result<()> {
    let micro_posts = state.micro_posts_repo().find_all().await?;

    for micro_post in micro_posts {
        let first_line: Option<&str> = match micro_post.content.lines().next() {
            Some(line) => match line.contains("](ht") {
                true => None,
                false => Some(line),
            },
            None => None,
        };

        let mut page = Page::new(
            micro_post.slug.clone(),
            None,
            first_line,
        )
        .with_date(micro_post.date)
        .with_tags(micro_post.tags.clone());

        if let Some(first) = micro_post.media.first() {
            match first {
                Media::Image(image) => {
                    page = page.with_image(image.clone().into());
                }
            }
        }

        let template = MicroPostTemplate {
            page,
            post: micro_post.clone(),
        };

        state
            .page_rendering_service()
            .add_page(
                state,
                template.page.slug.clone(),
                template,
                micro_post.updated_at.as_ref(),
            )
            .await?;
    }

    Ok(())
}
