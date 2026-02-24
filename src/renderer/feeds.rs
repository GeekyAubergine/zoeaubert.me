use askama::Template;

use crate::{
    domain::models::{blog_post::BlogPost, data::Data, site_config::SITE_CONFIG},
    prelude::*,
    renderer::{RenderTask, RenderTasks},
};

use crate::renderer::formatters::format_markdown::FormatMarkdown;
use crate::renderer::formatters::format_relative_to_absolute_urls::FormatRelativeToAbsoluteUrls;

pub fn render_feeds<'d>(data: &'d Data, tasks: &mut RenderTasks<'d>) {
    let blog_posts = data
        .timeline_events
        .blog_posts_by_date()
        .collect::<Vec<&BlogPost>>();

    tasks.add(RenderBlogPostXmlPageTask { blog_posts });
}

struct RenderBlogPostXmlPageTask<'l> {
    blog_posts: Vec<&'l BlogPost>,
}

#[derive(Template)]
#[template(path = "feeds/blog_post_feed.xml")]
struct BlogPostXmlTemplate<'t> {
    site_description: String,
    feed_permalnk: String,
    blog_posts: &'t Vec<&'t BlogPost>,
}

impl<'l> RenderTask for RenderBlogPostXmlPageTask<'l> {
    fn render(
        self: Box<Self>,
        renderer: &crate::services::page_renderer::PageRenderer,
    ) -> Result<()> {
        let template = BlogPostXmlTemplate {
            site_description: SITE_CONFIG.description.clone(),
            feed_permalnk: format!("{}/feeds/blog-rss.xml", SITE_CONFIG.url),
            blog_posts: &self.blog_posts,
        };

        let rendered = template.render().unwrap();

        renderer.render_string("/feeds/blog-rss.xml".into(), &rendered)?;

        // Legacy location I don't want to break with possible redir
        let template = BlogPostXmlTemplate {
            site_description: SITE_CONFIG.description.clone(),
            feed_permalnk: format!("{}/rss.xml", SITE_CONFIG.url),
            blog_posts: &self.blog_posts,
        };

        let rendered = template.render().unwrap();

        renderer.render_string("/rss.xml".into(), &rendered)
    }
}
