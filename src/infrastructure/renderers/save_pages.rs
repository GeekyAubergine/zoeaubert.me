use std::collections::HashMap;

use askama::Template;

use crate::{
    domain::{
        models::{omni_post::OmniPost, page::Page, referral::Referral, slug::Slug},
        queries::{omni_post_queries::find_all_omni_posts_by_tag, tags_queries::find_tag_counts},
        repositories::ReferralsRepo,
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

use super::render_page_with_template;

#[derive(Template)]
#[template(path = "save.html")]
struct SaveTemplate<'t> {
    page: &'t Page<'t>,
    referrals: Vec<Referral>,
}

pub async fn render_save_page(state: &impl State) -> Result<()> {
    let referrals = state.referrals_repo().find_all().await?;

    let page = Page::new(Slug::new("save"), Some("Save"), None);

    let template = SaveTemplate { page: &page, referrals };

    render_page_with_template(state, &page, template).await
}
