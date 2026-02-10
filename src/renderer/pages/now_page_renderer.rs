use std::slice::Iter;
use std::str::FromStr;

use askama::Template;
use hypertext::prelude::*;
use hypertext::Raw;
use maud::PreEscaped;
use tracing_subscriber::fmt::format;
use url::Url;

use crate::domain::models::data::Data;
use crate::domain::models::image::Image;
use crate::domain::models::slug::Link;
use crate::domain::models::slug::Slug;
use crate::domain::models::tag::Tag;
use crate::domain::models::timeline_event::TimelineEvent;
use crate::domain::models::timeline_event::TimelineEventPost;
use crate::domain::models::{blog_post::BlogPost, page::Page};
use crate::prelude::*;

use crate::domain::models::media::Media;
use crate::renderer::formatters::format_date::FormatDate;
use crate::renderer::formatters::format_markdown::FormatMarkdown;
use crate::renderer::partials::date::render_date;
use crate::renderer::partials::md::md;
use crate::renderer::partials::md::MarkdownMediaOption;
use crate::renderer::partials::page::render_page;
use crate::renderer::partials::page::PageOptions;
use crate::renderer::partials::tag::render_tags;
use crate::renderer::RendererContext;
use crate::services::file_service::ContentFile;

pub fn render_now_page(context: &RendererContext) -> Result<()> {
    let page = Page::new(Slug::new("/now"), Some("Now".to_string()), None);
    let slug = page.slug.clone();

    let content = maud! {
        article {
            (md(&context.data.now_text.now_text, MarkdownMediaOption::WithMedia))
        }
    };

    let options = PageOptions::new().with_main_class("now-page");

    let render = render_page(&page, &options, &content, maud! {});

    context.renderer.render_page(&slug, &render, None)
}
