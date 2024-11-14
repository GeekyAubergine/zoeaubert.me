use crate::{
    domain::{
        models::{
            album::{Album, AlbumPhoto},
            image::Image,
            slug::Slug,
        },
        repositories::AlbumsRepo,
        services::PageRenderingService,
        state::State,
    },
    infrastructure::utils::cover_photos_for_album::cover_photos_for_album,
    prelude::*,
};

use askama::Template;

use crate::infrastructure::renderers::formatters_renderer::format_date::FormatDate;
use crate::infrastructure::renderers::formatters_renderer::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters_renderer::format_number::FormatNumber;

use crate::domain::models::page::Page;

pub async fn render_albums_and_photo_pages(state: &impl State) -> Result<()> {
    render_album_list_page(state).await?;
    render_all_albums_page(state).await?;

    let albums = state.albums_repo().find_all_by_date().await?;

    for album in albums {
        render_album_page(state, album).await?;
    }

    Ok(())
}

struct AlbumListItem {
    pub album: Album,
    pub cover_images: Vec<Image>,
}

#[derive(Template)]
#[template(path = "albums/album_list.html")]
struct AlbumListPage {
    page: Page,
    albums_by_year: Vec<(u16, Vec<AlbumListItem>)>,
}

async fn render_album_list_page(state: &impl State) -> Result<()> {
    let mut page = Page::new(
        Slug::new("/albums"),
        Some("Albums"),
        Some("My photo albums"),
    );

    let albums_by_year = state.albums_repo().find_grouped_by_year().await?;

    let albums_by_year = albums_by_year
        .into_iter()
        .map(|(year, albums)| {
            let albums = albums
                .into_iter()
                .map(|album| {
                    Ok(AlbumListItem {
                        cover_images: album.cover_images(),
                        album,
                    })
                })
                .collect::<Result<Vec<_>>>();

            Ok((year, albums?))
        })
        .collect::<Result<Vec<_>>>()?;

    if let Some((_, albums)) = albums_by_year.first() {
        if let Some(album) = albums.first() {
            if let Some(cover_image) = album.album.cover_images().first() {
                page = page.with_image(cover_image.clone().into());
            }
        }
    }

    let template = AlbumListPage {
        page,
        albums_by_year,
    };

    state
        .page_rendering_service()
        .add_page(state, template.page.slug.clone(), template)
        .await?;

    Ok(())
}

#[derive(Template)]
#[template(path = "albums/all_albums.html")]
struct AllAlbumsPage {
    page: Page,
    albums: Vec<Album>,
}

async fn render_all_albums_page(state: &impl State) -> Result<()> {
    let mut page = Page::new(
        Slug::new("/albums/all"),
        Some("All Albums"),
        Some("All photo albums"),
    );

    let albums = state.albums_repo().find_all_by_date().await?;

    if let Some(album) = albums.first() {
        if let Some(cover_image) = album.cover_images().first() {
            page = page.with_image(cover_image.clone().into());
        }
    }

    let template = AllAlbumsPage { page, albums };

    state
        .page_rendering_service()
        .add_page(state, template.page.slug.clone(), template)
        .await?;

    Ok(())
}

#[derive(Template)]
#[template(path = "albums/album.html")]
struct AlbumPage {
    page: Page,
    album: Album,
    description: String,
}

pub async fn render_album_page(state: &impl State, album: Album) -> Result<()> {
    let total_photos = album.photos.len();

    for i in 0..total_photos {
        let photo = album.photos[i].clone();

        let previous_photo = if i > 0 {
            Some(album.photos[i - 1].clone())
        } else {
            None
        };

        let next_photo = if i < album.photos.len() - 1 {
            Some(album.photos[i + 1].clone())
        } else {
            None
        };

        render_album_photo_page(
            state,
            album.clone(),
            photo,
            previous_photo,
            next_photo,
            i,
            total_photos,
        )
        .await?;
    }

    let description = album.description.clone().unwrap_or("".to_string());

    let mut page =
        Page::new(album.slug.clone(), Some(&album.title), Some(&description)).with_date(album.date);

    let cover_images = album.cover_images();

    if let Some(cover_image) = cover_images.first() {
        page = page.with_image(cover_image.clone().into());
    }

    let template = AlbumPage {
        page,
        album: album.clone(),
        description,
    };

    state
        .page_rendering_service()
        .add_page(state, template.page.slug.clone(), template)
        .await?;

    Ok(())
}

#[derive(Template)]
#[template(path = "albums/album_photo.html")]
struct AlbumPhotoPage {
    page: Page,
    album: Album,
    photo: AlbumPhoto,
    previous_photo: Option<AlbumPhoto>,
    next_photo: Option<AlbumPhoto>,
    index: usize,
    total_photos: usize,
}

pub async fn render_album_photo_page(
    state: &impl State,
    album: Album,
    photo: AlbumPhoto,
    previous_photo: Option<AlbumPhoto>,
    next_photo: Option<AlbumPhoto>,
    index: usize,
    total_photos: usize,
) -> Result<()> {
    let page = Page::new(
        photo.slug.clone(),
        Some(&photo.description),
        Some(&photo.small_image.alt),
    )
    .with_image(photo.small_image.clone().into());

    let template = AlbumPhotoPage {
        page,
        album,
        photo,
        previous_photo,
        next_photo,
        index,
        total_photos,
    };

    state
        .page_rendering_service()
        .add_page(state, template.page.slug.clone(), template)
        .await?;

    Ok(())
}
