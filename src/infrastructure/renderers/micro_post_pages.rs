use askama::Template;

use crate::domain::{
    models::{media::Media, micro_post::MicroPost, page::Page},
    repositories::MicroPostsRepo,
    state::State,
};

use super::render_page_with_template;

use crate::prelude::*;

use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

#[derive(Template)]
#[template(path = "micro_posts/post.html")]
pub struct MicroPostTemplate<'t> {
    page: &'t Page<'t>,
    post: &'t MicroPost,
}

pub async fn render_micro_post_pages(state: &impl State) -> Result<()> {
    let micro_posts = state.micro_posts_repo().find_all().await?;

    for micro_post in micro_posts {
        let page = Page::new(micro_post.slug.clone(), None, None)
            .with_date(micro_post.date)
            .with_tags(micro_post.tags.clone());

        let template = MicroPostTemplate {
            page: &page,
            post: &micro_post,
        };

        render_page_with_template(state, &page, template).await?;
    }

    Ok(())
}
