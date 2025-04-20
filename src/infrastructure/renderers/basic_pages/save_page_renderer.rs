use std::collections::HashMap;

use askama::Template;

use crate::{
    domain::{
        models::{omni_post::OmniPost, page::Page, referral::Referral, slug::Slug},
        queries::{omni_post_queries::find_all_omni_posts_by_tag, tags_queries::find_tag_counts},
        repositories::ReferralsRepo,
        services::PageRenderingService,
        state::State,
    },
    infrastructure::{
        renderers::RendererContext,
        utils::paginator::{paginate, PaginatorPage},
    },
    prelude::*,
};

use crate::domain::models::media::Media;
use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

use crate::domain::models::tag::Tag;

#[derive(Template)]
#[template(path = "save.html")]
struct SaveTemplate<'t> {
    page: Page,
    referrals: &'t Vec<Referral>,
}

pub async fn render_save_page(context: &RendererContext) -> Result<()> {
    let page = Page::new(Slug::new("save"), Some("Referrals"), None);

    let template = SaveTemplate {
        page,
        referrals: &context.data.referrals.referrals,
    };

    context
        .renderer
        .render_page(&template.page.slug, &template, None)
        .await
}
