use std::collections::HashMap;

use askama::Template;
use chrono::Utc;
use futures::try_join;

use crate::{
    domain::{
        models::{
            blog_post::BlogPost, omni_post::OmniPost, page::Page, site_config::SITE_CONFIG,
            slug::Slug,
        },
        queries::omni_post_queries::{find_all_omni_posts, OmniPostFilterFlags},
        repositories::BlogPostsRepo,
        services::PageRenderingService,
        state::State,
    },
    infrastructure::utils::paginator::{paginate, PaginatorPage},
    prelude::*,
};

use crate::domain::models::media::Media;
use crate::infrastructure::renderers::formatters_renderer::format_date::FormatDate;
use crate::infrastructure::renderers::formatters_renderer::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters_renderer::format_number::FormatNumber;
use crate::infrastructure::renderers::formatters_renderer::format_relative_to_absolute_urls::FormatRelativeToAbsoluteUrls;

use crate::domain::models::tag::Tag;

const DEFAULT_PAGINATION_SIZE: usize = 25;

pub async fn render_feed_files(state: &impl State) -> Result<()> {
    try_join!(
        render_interests_list_page(state),
        render_blog_post_feed_xml(state)
    )?;

    Ok(())
}

#[derive(Template)]
#[template(path = "feeds/feeds_list.html")]
pub struct FeedsListTemplate {
    page: Page,
}

pub async fn render_interests_list_page(state: &impl State) -> Result<()> {
    let page = Page::new(Slug::new("/feeds"), Some("Feeds"), None);

    let template = FeedsListTemplate { page };

    state
        .page_rendering_service()
        .add_page(state, template.page.slug.clone(), template, None)
        .await
}

#[derive(Template)]
#[template(path = "feeds/blog_post_feed.xml")]
struct BlogPostXmlTemplate {
    site_description: String,
    feed_permalnk: String,
    blog_posts: Vec<BlogPost>,
}

async fn render_blog_post_feed_xml(state: &impl State) -> Result<()> {
    let blog_posts = state.blog_posts_repo().find_all_by_date().await?;

    let template = BlogPostXmlTemplate {
        site_description: SITE_CONFIG.description.clone(),
        feed_permalnk: format!("{}/feeds/blog-rss.xml", SITE_CONFIG.url),
        blog_posts: blog_posts.clone(),
    };

    state
        .page_rendering_service()
        .add_file(state, "/feeds/blog-rss.xml".into(), template)
        .await?;

    // Legacy location I don't want to break with possible redir
    let template = BlogPostXmlTemplate {
        site_description: SITE_CONFIG.description.clone(),
        feed_permalnk: format!("{}/rss.xml", SITE_CONFIG.url),
        blog_posts,
    };

    state
        .page_rendering_service()
        .add_file(state, "/rss.xml".into(), template)
        .await?;

    Ok(())
}
