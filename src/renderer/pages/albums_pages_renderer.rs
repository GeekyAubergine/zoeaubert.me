use crate::domain::models::albums::album::Album;
use crate::domain::models::albums::album_photo::AlbumPhoto;
use crate::domain::models::timeline_event::{TimelineEvent, TimelineEventReview};
use crate::domain::models::{image::Image, review::book_review::BookReview};
use crate::prelude::*;
use crate::renderer::partials::date::render_date;
use hypertext::prelude::*;

use crate::{
    domain::models::{page::Page, slug::Slug},
    renderer::{
        RendererContext,
        partials::page::{PageOptions, render_page},
    },
};

pub fn render_alubms_pages<'l>(context: &'l RendererContext) -> Result<()> {
    render_alubms_list_page(context)?;

    let albums = context.data.albums.find_all_by_date();

    for album in albums {
        render_album_page(context, album)?;
    }

    render_all_albums_page(context)
}

pub fn render_alubms_list_page<'l>(context: &'l RendererContext) -> Result<()> {
    let page = Page::new(Slug::new("/albums"), Some("Albums".to_string()), None);

    let slug = page.slug.clone();

    let years = context.data.albums.find_grouped_by_year();

    let content = maud! {
        @for (year, albums) in &years {
            section {
                h2 { (year) }
                ol {
                    @for album in albums {
                        li {
                            a href=(album.slug.relative_string()) {
                                div class=(if album.cover_images().len() > 1 { "preview-multi" } else { "preview-single" }) {
                                    @for photo in album.cover_images() {
                                        (photo.render_small())
                                    }
                                }
                                div class="title-and-date" {
                                    h3 {
                                        (album.title)
                                    }
                                    (render_date(&album.date))
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    let options = PageOptions::new().with_main_class("albums-list-page");

    let renderer = render_page(&page, &options, &content, maud! {});

    context.renderer.render_page(&slug, &renderer, None)
}

pub fn render_album_page<'l>(context: &'l RendererContext, album: &Album) -> Result<()> {
    let page = album.page();

    let slug = page.slug.clone();

    let content = maud! {
        @if let Some(description) = &album.description {
            div class="description" {
                (description)
            }
        }
        (render_photo_grid(&album.photos))
    };

    let options = PageOptions::new().with_main_class("album-page");

    let renderer = render_page(&page, &options, &content, maud! {});

    context.renderer.render_page(&slug, &renderer, None)
}

pub fn render_all_albums_page<'l>(context: &'l RendererContext) -> Result<()> {
    let page = Page::new(
        Slug::new("/albums/all"),
        Some("All Album Photos".to_string()),
        None,
    );

    let slug = page.slug.clone();

    let albums = context.data.albums.find_all_by_date();

    let content = maud! {
        @for album in &albums {
            section {
                a href=(album.slug.relative_string()) {
                    h2 {
                        (album.title)
                    }
                }
                (render_photo_grid(&album.photos))
            }
        }
    };

    let options = PageOptions::new().with_main_class("all-albums-photos-page");

    let renderer = render_page(&page, &options, &content, maud! {});

    context.renderer.render_page(&slug, &renderer, None)
}

fn render_photo_grid<'l>(photos: &'l [AlbumPhoto]) -> impl Renderable + 'l {
    maud! {
        ul class="photo-grid-variable-orientation" {
            @for photo in photos {
                li class=(photo.image.orientation().to_string()) {
                    (photo.image.render_small())
                }
            }
        }
    }
}
