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

use tokio::try_join;

const DEFAULT_PAGINATION_SIZE: usize = 25;

#[derive(Template)]
#[template(path = "omni_post/omni_post_list/omni_post_list_page.html")]
pub struct TimelineTemplate {
    page: Page,
    posts: Vec<OmniPost>,
}

pub async fn render_timeline_pages(state: &impl State) -> Result<()> {
    try_join!(render_timeline_page(state), render_firehost_page(state),)?;

    Ok(())
}

async fn render_timeline_page(state: &impl State) -> Result<()> {
    let omni_posts =
        find_all_omni_posts(state, OmniPostFilterFlags::filter_main_timeline()).await?;

    let paginated = paginate(&omni_posts, DEFAULT_PAGINATION_SIZE);

    let page = Page::new(Slug::new("timeline"), Some("Timeline"), Some("My timeline".to_string()));

    for paginator_page in paginated {
        let page = Page::from_page_and_pagination_page(&page, &paginator_page, "Posts");

        let template = TimelineTemplate {
            page,
            posts: paginator_page.data.to_vec(),
        };

        state
            .page_rendering_service()
            .add_page(
                state,
                template.page.slug.clone(),
                template,
                paginator_page
                    .data
                    .first()
                    .map(|p| p.last_updated_at())
                    .flatten(),
            )
            .await?;
    }

    Ok(())
}

async fn render_firehost_page(state: &impl State) -> Result<()> {
    let omni_posts =
        find_all_omni_posts(state, OmniPostFilterFlags::filter_firehose()).await?;

    let paginated = paginate(&omni_posts, 100);

    let page = Page::new(
        Slug::new("firehose"),
        Some("Firehose"),
        Some("All the things, all the time".to_string()),
    );

    for paginator_page in paginated {
        let page = Page::from_page_and_pagination_page(&page, &paginator_page, "Posts");

        let template = TimelineTemplate {
            page,
            posts: paginator_page.data.to_vec(),
        };

        state
            .page_rendering_service()
            .add_page(
                state,
                template.page.slug.clone(),
                template,
                paginator_page
                    .data
                    .first()
                    .map(|p| p.last_updated_at())
                    .flatten(),
            )
            .await?;
    }

    Ok(())
}
