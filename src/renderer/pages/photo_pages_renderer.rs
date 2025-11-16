use hypertext::prelude::*;

use crate::domain::models::blog_post::{self, BlogPost};
use crate::domain::models::image::Image;
use crate::domain::models::media::Media;
use crate::domain::models::page::Page;
use crate::domain::models::slug::Slug;
use crate::domain::models::timeline_event::{TimelineEvent, TimelineEventPost};
use crate::prelude::*;
use crate::renderer::formatters::format_date::FormatDate;
use crate::renderer::formatters::format_markdown::FormatMarkdown;
use crate::renderer::partials::date::render_date;
use crate::renderer::partials::md::{self, md};
use crate::renderer::partials::page::{render_page, PageOptions, PageWidth};
use crate::renderer::partials::tag::render_tags;
use crate::renderer::RendererContext;
use crate::utils::paginator::paginate;

const PAGINATION_SIZE: usize = 40;

pub fn render_photo_pages(context: &RendererContext) -> Result<()> {
    let photos = context
        .data
        .timeline_events
        .all_by_date()
        .iter()
        .filter_map(|event| match event {
            TimelineEvent::Post(post) => match post {
                TimelineEventPost::BlogPost(_) => None,
                TimelineEventPost::MicroPost(post) => Some(post.media()),
                TimelineEventPost::MastodonPost(post) => Some(post.media()),
            },
            TimelineEvent::BookReview { .. } => None,
            TimelineEvent::MovieReview { .. } => None,
        })
        .flatten()
        .filter_map(|media| match media {
            Media::Image(image) => Some(image),
            _ => None,
        })
        .collect::<Vec<&Image>>();

    render_photos_list_page(context, &photos)?;

    Ok(())
}

fn photo<'l>(photo: &'l Image) -> impl Renderable + 'l {
    maud! {
        @if let Some(l) = &photo.link_on_click {
            li {
                a href=(l) {
                    (photo.render_small())
                }
            }
        } @else {
            li {
                (photo.render_small())
            }
        }
    }
}

pub fn render_photos_list_page(context: &RendererContext, photos: &[&Image]) -> Result<()> {
    let paginated = paginate(&photos, PAGINATION_SIZE);

    let page = Page::new(Slug::new("/photos"), Some("Photos".to_string()), None);
    for paginator_page in paginated {
        let page = Page::from_page_and_pagination_page(&page, &paginator_page, "Photos");

        let slug = page.slug.clone();

        let content = maud! {
            ul class="photos-list" {
                @for post in paginator_page.data {
                    (photo(post))
                }
            }
        };

        let options = PageOptions::new().with_main_class("photos-page");

        let renderer = render_page(&page, &options, &content, maud! {});

        context.renderer.render_page(&slug, &renderer, None)?;
    }

    Ok(())
}
