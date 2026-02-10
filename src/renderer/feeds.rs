use std::collections::HashMap;

use askama::Template;
use chrono::Utc;
use futures::try_join;

use crate::{
    domain::models::{
        blog_post::BlogPost,
        page::Page,
        site_config::SITE_CONFIG,
        slug::Slug,
        timeline_event::{TimelineEvent, TimelineEventPost},
    },
    error::TemplateError,
    prelude::*,
    renderer::RendererContext,
    utils::paginator::{PaginatorPage, paginate},
};

use crate::domain::models::media::Media;

use crate::domain::models::tag::Tag;
use crate::renderer::formatters::format_markdown::FormatMarkdown;
use crate::renderer::formatters::format_relative_to_absolute_urls::FormatRelativeToAbsoluteUrls;

const DEFAULT_PAGINATION_SIZE: usize = 25;

pub fn render_feeds(context: &RendererContext) -> Result<()> {
    render_blog_post_feed_xml(context)
}

#[derive(Template)]
#[template(path = "feeds/blog_post_feed.xml")]
struct BlogPostXmlTemplate<'t> {
    site_description: String,
    feed_permalnk: String,
    blog_posts: &'t Vec<&'t BlogPost>,
}

fn render_blog_post_feed_xml(context: &RendererContext) -> Result<()> {
    let blog_posts = context
        .data
        .timeline_events
        .all_by_date()
        .iter()
        .filter_map(|event| match event {
            TimelineEvent::Post(post) => match post {
                TimelineEventPost::BlogPost(post) => Some(post),
                _ => None,
            },
            _ => None,
        })
        .collect::<Vec<&BlogPost>>();

    let template = BlogPostXmlTemplate {
        site_description: SITE_CONFIG.description.clone(),
        feed_permalnk: format!("{}/feeds/blog-rss.xml", SITE_CONFIG.url),
        blog_posts: &blog_posts,
    };

    let rendered = template.render().unwrap();

    context
        .renderer
        .render_string("/feeds/blog-rss.xml".into(), &rendered)?;

    // Legacy location I don't want to break with possible redir
    let template = BlogPostXmlTemplate {
        site_description: SITE_CONFIG.description.clone(),
        feed_permalnk: format!("{}/rss.xml", SITE_CONFIG.url),
        blog_posts: &blog_posts,
    };

    let rendered = template.render().unwrap();

    context.renderer.render_string("/rss.xml".into(), &rendered)
}
