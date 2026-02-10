use crate::domain::models::albums::album::Album;
use crate::domain::models::albums::album_photo::AlbumPhoto;
use crate::domain::models::timeline_event::{TimelineEvent, TimelineEventReview};
use crate::domain::models::{image::Image, review::book_review::BookReview};
use crate::prelude::*;
use crate::renderer::partials::date::render_date;
use crate::renderer::partials::javascript::album_photo_controls_scripts;
use crate::renderer::partials::tag::render_tags;
use hypertext::{Raw, prelude::*};

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

    context.renderer.render_page(&slug, &renderer, None)?;

    for photo in &album.photos {
        render_album_photo(context, &album, &photo)?;
    }

    Ok(())
}

pub fn render_album_photo<'l>(
    context: &'l RendererContext,
    album: &Album,
    photo: &AlbumPhoto,
) -> Result<()> {
    let page = photo.page();

    let slug = page.slug.clone();

    let previous = album.previous_photo(photo);
    let next = album.next_photo(photo);
    let index = album.index_of_photo(photo);

    let content = maud! {
        h1 {
            (photo.description)
        }
        div class="image-container" {
            (photo.image.render_large())
        }
        div class="buttons-and-description" {
            (render_tags(&photo.tags, None))
            div class="nav" {
                @if let Some(previous) = previous {
                    a
                        href=(previous.slug.relative_string())
                        class="arrow" {
                            "←"
                    }
                } @else {
                    div class="arrow" {
                    }
                }
                @if let Some(index) = index {
                    p { (format!("{} / {}", index + 1, album.total_photos()))}
                }
                @if let Some(next) = next {
                    a
                        href=(next.slug.relative_string())
                        class="arrow" {
                            "→"
                    }
                } @else {
                    div class="arrow" {
                    }
                }
            }
            div class="links" {
                a href=(album.slug.relative_string()) {
                    "Album"
                }
                p class="mx-2" { "–" }
                a href=(photo.image.original.file.as_cdn_url().as_str()) target="_blank" rel="noopener" {
                    "Original"
                }
            }
        }
    };

    let options = PageOptions::new()
        .with_body_class("album-photo-page")
        .with_main_class("album-photo-main")
        .hide_header()
        .hide_footer();

    let renderer = render_page(
        &page,
        &options,
        &content,
        album_photo_controls_scripts(photo, previous, next),
    );

    context
        .renderer
        .render_page(&slug, &renderer, Some(photo.date))
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
                    a href=(photo.slug.relative_string()) {
                        (photo.image.render_small())
                    }
                }
            }
        }
    }
}
