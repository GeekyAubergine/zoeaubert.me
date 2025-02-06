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

use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

use crate::domain::models::page::Page;

pub async fn render_album_pages(state: &impl State) -> Result<()> {
    render_album_list_page(state).await?;
    render_all_albums_page(state).await?;

    let albums = state.albums_repo().find_all_by_date().await?;

    for album in albums {
        // render_album_page(state, album).await?;
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
        Some("My photo albums".to_string()),
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

    let most_recent_updated_at = albums_by_year
        .first()
        .map(|(_, albums)| albums.first().map(|album| album.album.updated_at));

    let template = AlbumListPage {
        page,
        albums_by_year,
    };

    state
        .page_rendering_service()
        .add_page(
            state,
            template.page.slug.clone(),
            template,
            most_recent_updated_at.flatten().as_ref(),
        )
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
        Some("All photo albums".to_string()),
    );

    let albums = state.albums_repo().find_all_by_date().await?;

    if let Some(album) = albums.first() {
        if let Some(cover_image) = album.cover_images().first() {
            page = page.with_image(cover_image.clone().into());
        }
    }

    let most_recent_updated_at = albums.first().map(|album| album.updated_at);

    let template = AllAlbumsPage { page, albums };

    state
        .page_rendering_service()
        .add_page(
            state,
            template.page.slug.clone(),
            template,
            most_recent_updated_at.as_ref(),
        )
        .await?;

    Ok(())
}
