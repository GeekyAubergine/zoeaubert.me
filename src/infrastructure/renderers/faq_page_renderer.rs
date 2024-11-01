use std::collections::HashMap;

use askama::Template;

use crate::{
    domain::{
        models::{omni_post::OmniPost, page::Page, referral::Referral, slug::Slug},
        queries::{omni_post_queries::find_all_omni_posts_by_tag, tags_queries::find_tag_counts},
        repositories::{FaqRepo, ReferralsRepo},
        services::PageRenderingService,
        state::State,
    },
    infrastructure::utils::paginator::{paginate, PaginatorPage},
    prelude::*,
};

use crate::domain::models::media::Media;
use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

use crate::domain::models::tag::Tag;

#[derive(Template)]
#[template(path = "faq.html")]
struct FaqTemplate {
    page: Page,
    faq: String,
}

pub async fn render_faq_page(state: &impl State) -> Result<()> {
    let faq = state.faq_repo().find().await?;

    let page = Page::new(Slug::new("faq"), Some("FAQ"), None);

    let template = FaqTemplate { page, faq };

    state
        .page_rendering_service()
        .add_page(state, template.page.slug.clone(), template)
        .await
}
