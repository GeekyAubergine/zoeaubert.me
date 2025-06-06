use std::collections::HashMap;

use askama::Template;

use crate::{
    domain::{
        models::{omni_post::OmniPost, page::Page, referral::Referral, slug::Slug},
        queries::{omni_post_queries::find_all_omni_posts_by_tag, tags_queries::find_tag_counts},
        repositories::{FaqRepo, NowTextRepo, ReferralsRepo},
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
#[template(path = "now.html")]
struct FaqTemplate<'t> {
    page: Page,
    now: &'t str,
}

pub async fn render_now_page(context: &RendererContext) -> Result<()> {
    let page = Page::new(Slug::new("now"), Some("Now"), None);

    let template = FaqTemplate {
        page,
        now: &context.data.now_text.now_text,
    };

    context
        .renderer
        .render_page(&template.page.slug, &template, None)
        .await
}
