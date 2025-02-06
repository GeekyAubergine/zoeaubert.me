use askama::Template;

use crate::{
    domain::{
        models::{
            omni_post::OmniPost,
            page::{Page, PagePagination},
            slug::Slug,
        },
        queries::omni_post_queries::{find_all_omni_posts, OmniPostFilterFlags},
        services::PageRenderingService,
        state::State,
    },
    infrastructure::utils::paginator::{paginate, PaginatorPage},
};

use crate::prelude::*;

use crate::domain::models::media::Media;
use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

#[derive(Template)]
#[template(path = "omni_post/omni_post_page/omni_post_page.html")]
pub struct PostTemplate {
    page: Page,
    omni_post: OmniPost,
}

pub async fn render_omni_post_pages(state: &impl State) -> Result<()> {
    let omni_posts = find_all_omni_posts(
        state,
        OmniPostFilterFlags::filter_all(),
    )
    .await?;

    for omni_post in omni_posts {
        render_omni_post_page(state, &omni_post).await?;
    }

    Ok(())
}

async fn render_omni_post_page(state: &impl State, omni_post: &OmniPost) -> Result<()> {
    match omni_post.page() {
        Some(page) => {
            let template = PostTemplate {
                page: page.clone(),
                omni_post: omni_post.clone(),
            };

            state
                .page_rendering_service()
                .add_page(state, page.slug.clone(), template, None)
                .await
        }
        None => {
            Ok(())
        }
    }
}
