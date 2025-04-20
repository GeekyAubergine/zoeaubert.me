use std::collections::HashMap;

use askama::Template;
use chrono::Utc;
use futures::try_join;

use crate::{
    domain::{
        models::{
            blog_post::BlogPost, data::PostFilter, omni_post::OmniPost, page::Page,
            site_config::SITE_CONFIG, slug::Slug,
        },
        queries::omni_post_queries::{find_all_omni_posts, OmniPostFilterFlags},
        repositories::BlogPostsRepo,
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
use crate::infrastructure::renderers::formatters::format_relative_to_absolute_urls::FormatRelativeToAbsoluteUrls;

use crate::domain::models::tag::Tag;

const DEFAULT_PAGINATION_SIZE: usize = 25;

pub async fn render_feed_files(context: &RendererContext) -> Result<()> {
    try_join!(
        render_interests_list_page(context),
        render_blog_post_feed_xml(context)
    )?;

    Ok(())
}

#[derive(Template)]
#[template(path = "feeds/feeds_list.html")]
pub struct FeedsListTemplate {
    page: Page,
}

pub async fn render_interests_list_page(context: &RendererContext) -> Result<()> {
    let page = Page::new(Slug::new("/feeds"), Some("Feeds"), None);

    let template = FeedsListTemplate { page };

    context
        .renderer
        .render_page(&template.page.slug, &template, None)
        .await
}

#[derive(Template)]
#[template(path = "feeds/blog_post_feed.xml")]
struct BlogPostXmlTemplate<'t> {
    site_description: String,
    feed_permalnk: String,
    blog_posts: &'t Vec<&'t BlogPost>,
}

async fn render_blog_post_feed_xml(context: &RendererContext) -> Result<()> {
    let blog_posts = context
        .data
        .posts
        .find_all_by_filter(PostFilter::BLOG_POST)
        .iter()
        .filter_map(|post| match post {
            OmniPost::BlogPost(post) => Some(post),
            _ => None,
        })
        .collect::<Vec<&BlogPost>>();

    let template = BlogPostXmlTemplate {
        site_description: SITE_CONFIG.description.clone(),
        feed_permalnk: format!("{}/feeds/blog-rss.xml", SITE_CONFIG.url),
        blog_posts: &blog_posts,
    };

    context
        .renderer
        .render_file("/feeds/blog-rss.xml".into(), &template)
        .await?;

    // Legacy location I don't want to break with possible redir
    let template = BlogPostXmlTemplate {
        site_description: SITE_CONFIG.description.clone(),
        feed_permalnk: format!("{}/rss.xml", SITE_CONFIG.url),
        blog_posts: &blog_posts,
    };

    context
        .renderer
        .render_file("/rss.xml".into(), &template)
        .await
}
