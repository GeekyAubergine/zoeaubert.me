use crate::domain::models::albums::album_photo::AlbumPhoto;
use crate::domain::models::data::Data;
use crate::prelude::*;
use crate::renderer::RenderTasks;
use crate::renderer::partials::date::render_date;
use crate::renderer::partials::javascript::album_photo_controls_scripts;
use crate::renderer::partials::tag::render_tags;
use crate::{domain::models::albums::album::Album, renderer::RenderTask};
use hypertext::prelude::*;

use crate::{
    domain::models::{page::Page, slug::Slug},
    renderer::{
        RendererContext,
        partials::page::{PageOptions, render_page},
    },
};

pub fn render_albums_pages<'d>(data: &'d Data, tasks: &mut RenderTasks<'d>) {
    tasks.add(RenderAlbumsListPageTask {
        years: data.albums.find_grouped_by_year(),
    });

    let albums = data.albums.find_all_by_date();

    for album in &albums {
        for photo in &album.photos {
            tasks.add(RenderAlbumPhotoPageTask { album, photo });
        }
    }

    tasks.add(RenderAllAlbumsPageTask { albums });
}

pub fn render_alubms_list_page(context: &RendererContext) -> Result<()> {
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

struct RenderAlbumsListPageTask<'l> {
    years: Vec<(u16, Vec<&'l Album>)>,
}

impl<'l> RenderTask for RenderAlbumsListPageTask<'l> {
    fn render(
        self: Box<Self>,
        renderer: &crate::services::page_renderer::PageRenderer,
    ) -> Result<()> {
        let page = Page::new(Slug::new("/albums"), Some("Albums".to_string()), None);

        let slug = page.slug.clone();

        let content = maud! {
            @for (year, albums) in &self.years {
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

        let rendered = render_page(&page, &options, &content, maud! {});

        renderer.render_page(&slug, &rendered, None)
    }
}

struct RenderAlbumPageTask<'l> {
    album: &'l Album,
}

impl<'l> RenderTask for RenderAlbumPageTask<'l> {
    fn render(
        self: Box<Self>,
        renderer: &crate::services::page_renderer::PageRenderer,
    ) -> Result<()> {
        let album = self.album;

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

        let rendered = render_page(&page, &options, &content, maud! {});

        renderer.render_page(&slug, &rendered, None)
    }
}

struct RenderAlbumPhotoPageTask<'l> {
    album: &'l Album,
    photo: &'l AlbumPhoto,
}

impl<'l> RenderTask for RenderAlbumPhotoPageTask<'l> {
    fn render(
        self: Box<Self>,
        renderer: &crate::services::page_renderer::PageRenderer,
    ) -> Result<()> {
        let album = self.album;
        let photo = self.photo;

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

        let rendered = render_page(
            &page,
            &options,
            &content,
            album_photo_controls_scripts(previous, next),
        );

        renderer.render_page(&slug, &rendered, Some(photo.date))
    }
}

struct RenderAllAlbumsPageTask<'l> {
    albums: Vec<&'l Album>,
}

impl<'l> RenderTask for RenderAllAlbumsPageTask<'l> {
    fn render(
        self: Box<Self>,
        renderer: &crate::services::page_renderer::PageRenderer,
    ) -> Result<()> {
        let page = Page::new(
            Slug::new("/albums/all"),
            Some("All Album Photos".to_string()),
            None,
        );

        let slug = page.slug.clone();

        let content = maud! {
            @for album in &self.albums {
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

        let rendered = render_page(&page, &options, &content, maud! {});

        renderer.render_page(&slug, &rendered, None)
    }
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
