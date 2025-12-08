use crate::domain::models::timeline_event::{TimelineEvent, TimelineEventReview};
use crate::domain::models::{image::Image, review::book_review::BookReview};
use crate::prelude::*;
use hypertext::prelude::*;

use crate::{
    domain::models::{page::Page, slug::Slug},
    renderer::{
        RendererContext,
        partials::page::{PageOptions, render_page},
    },
};

pub fn render_alubms_pages<'l>(context: &'l RendererContext) -> Result<()> {
    let projects = context.data.projects.find_all_by_rank_and_name();

    let page = Page::new(Slug::new("/albums"), Some("Albums".to_string()), None);

    let slug = page.slug.clone();

    let years = context.data.albums.find_grouped_by_year();

    let content = maud! {
        @for (year, albums) in &years {
            section {
                h2 { (year) }
                ul {
                    @for album in albums {
                        li {
                            a href=(album.slug.relative_string()) {
                                // (album.cover_images().)
                                h3 {
                                    (album.title)
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    let options = PageOptions::new().with_main_class("albums-page");

    let renderer = render_page(&page, &options, &content, maud! {});

    context.renderer.render_page(&slug, &renderer, None)
}
