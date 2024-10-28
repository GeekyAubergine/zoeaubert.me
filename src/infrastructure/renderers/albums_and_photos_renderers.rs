use crate::{
    domain::{
        models::{
            album::{Album, AlbumPhoto},
            image::Image,
            slug::Slug,
        },
        repositories::AlbumsRepo,
        state::State,
    },
    infrastructure::utils::cover_photos_for_album::cover_photos_for_album,
    prelude::*,
};

use askama::Template;

use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

use crate::domain::models::page::Page;

use super::render_page_with_template;

pub async fn render_albums_and_photo_pages(state: &impl State) -> Result<()> {
    render_album_list_page(state).await?;
    render_all_albums_page(state).await?;

    let albums = state.albums_repo().find_all_by_date().await?;

    for album in albums {
        render_album_page(state, album).await?;
    }

    // for album in albums {
    //     let page = Page::new(album.permalink.clone(), None, None)
    //         .with_date(album.date)
    //         .with_tags(album.tags.clone());

    //     let template = AlbumListPage { page: &page };

    //     render_page_with_template(state, &page, template).await?;
    // }

    Ok(())
}

struct AlbumListItem {
    pub album: Album,
    pub cover_images: Vec<Image>,
}

#[derive(Template)]
#[template(path = "albums/album_list.html")]
struct AlbumListPage<'t> {
    page: &'t Page<'t>,
    albums_by_year: Vec<(u16, Vec<AlbumListItem>)>,
}

async fn render_album_list_page(state: &impl State) -> Result<()> {
    let page = Page::new(
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
                    let cover_photos = cover_photos_for_album(&album);

                    let cover_images = cover_photos
                        .into_iter()
                        .map(|photo| photo.small_image.clone())
                        .collect::<Vec<_>>();

                    Ok(AlbumListItem {
                        album,
                        cover_images,
                    })
                })
                .collect::<Result<Vec<_>>>();

            Ok((year, albums?))
        })
        .collect::<Result<Vec<_>>>()?;

    let template = AlbumListPage {
        page: &page,
        albums_by_year,
    };

    render_page_with_template(state, &page, template).await?;

    Ok(())
}

#[derive(Template)]
#[template(path = "albums/all_albums.html")]
struct AllAlbumsPage<'t> {
    page: &'t Page<'t>,
    albums: Vec<Album>,
}

async fn render_all_albums_page(state: &impl State) -> Result<()> {
    let page = Page::new(
        Slug::new("/albums/all"),
        Some("All Albums"),
        Some("All photo albums"),
    );

    let albums = state.albums_repo().find_all_by_date().await?;

    let template = AllAlbumsPage {
        page: &page,
        albums,
    };

    render_page_with_template(state, &page, template).await?;

    Ok(())
}

#[derive(Template)]
#[template(path = "albums/album.html")]
struct AlbumPage<'t> {
    page: &'t Page<'t>,
    album: Album,
    description: String,
}

pub async fn render_album_page(state: &impl State, album: Album) -> Result<()> {
    let description = album.description.clone().unwrap_or("".to_string());

    let page =
        Page::new(album.slug.clone(), Some(&album.title), Some(&description)).with_date(album.date);

    let template = AlbumPage {
        page: &page,
        album: album.clone(),
        description,
    };

    render_page_with_template(state, &page, template).await?;

    let total_photos = album.photos.len();

    for i in 0..album.photos.len() {
        let photo = &album.photos[i];

        let previous_photo = if i > 0 {
            Some(&album.photos[i - 1])
        } else {
            None
        };

        let next_photo = if i < album.photos.len() - 1 {
            Some(&album.photos[i + 1])
        } else {
            None
        };

        render_album_photo_page(
            state,
            &album,
            photo,
            previous_photo,
            next_photo,
            i,
            total_photos,
        )
        .await?;
    }

    Ok(())
}

#[derive(Template)]
#[template(path = "albums/album_photo.html")]
struct AlbumPhotoPage<'t> {
    page: &'t Page<'t>,
    album: &'t Album,
    photo: &'t AlbumPhoto,
    previous_photo: Option<&'t AlbumPhoto>,
    next_photo: Option<&'t AlbumPhoto>,
    index: usize,
    total_photos: usize,
}

pub async fn render_album_photo_page(
    state: &impl State,
    album: &Album,
    photo: &AlbumPhoto,
    previous_photo: Option<&AlbumPhoto>,
    next_photo: Option<&AlbumPhoto>,
    index: usize,
    total_photos: usize,
) -> Result<()> {
    let page = Page::new(photo.slug.clone(), Some(&photo.description), None)
        .with_image(photo.small_image.into());

    let template = AlbumPhotoPage {
        page: &page,
        album,
        photo,
        previous_photo,
        next_photo,
        index,
        total_photos,
    };

    render_page_with_template(state, &page, template).await?;

    Ok(())
}
