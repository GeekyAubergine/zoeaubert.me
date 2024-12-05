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

const DEFAULT_PAGINATION_SIZE: usize = 25;

#[derive(Template)]
#[template(path = "timeline/index.html")]
pub struct TimelineTemplate {
    page: Page,
    posts: Vec<OmniPost>,
}

pub async fn render_timeline_page<'d>(state: &impl State) -> Result<()> {
    let omni_posts =
        find_all_omni_posts(state, OmniPostFilterFlags::filter_main_timeline()).await?;

    let paginated = paginate(&omni_posts, DEFAULT_PAGINATION_SIZE);

    let page = Page::new(Slug::new("timeline"), Some("Timeline"), Some("My timeline"));

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
