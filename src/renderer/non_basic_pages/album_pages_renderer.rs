use crate::{
    domain::models::{albums::album::Album, image::Image, slug::Slug},
    prelude::*,
    renderers::RendererContext,
    utils::cover_photos_for_album::cover_photos_for_album,
};

use askama::Template;

use crate::renderers::formatters::format_date::FormatDate;
use crate::renderers::formatters::format_markdown::FormatMarkdown;
use crate::renderers::formatters::format_number::FormatNumber;

use crate::domain::models::page::Page;

pub async fn render_album_pages(context: &RendererContext) -> Result<()> {
    render_album_list_page(context).await?;
    render_all_albums_page(context).await?;

    let albums = context.data.albums.find_all_by_date();

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

async fn render_album_list_page(context: &RendererContext) -> Result<()> {
    let mut page = Page::new(
        Slug::new("/albums"),
        Some("Albums"),
        Some("My photo albums".to_string()),
    );

    let albums_by_year = context.data.albums.find_grouped_by_year();

    let albums_by_year = albums_by_year
        .into_iter()
        .map(|(year, albums)| {
            let albums = albums
                .into_iter()
                .map(|album| {
                    Ok(AlbumListItem {
                        cover_images: album.cover_images().iter().cloned().cloned().collect(),
                        album: album.clone(),
                    })
                })
                .collect::<Result<Vec<_>>>();

            Ok((year, albums?))
        })
        .collect::<Result<Vec<_>>>()?;

    if let Some((_, albums)) = albums_by_year.first() {
        if let Some(album) = albums.first() {
            if let Some(cover_image) = album.album.cover_images().first() {
                page = page.with_image(cover_image.clone().clone().into());
            }
        }
    }

    let most_recent_updated_at = albums_by_year
        .first()
        .map(|(_, albums)| albums.first().map(|album| album.album.date));

    let template = AlbumListPage {
        page,
        albums_by_year,
    };

    context
        .renderer
        .render_page(
            &template.page.slug,
            &template,
            most_recent_updated_at.flatten(),
        )
        .await
}

#[derive(Template)]
#[template(path = "albums/all_albums.html")]
struct AllAlbumsPage {
    page: Page,
    albums: Vec<Album>,
}

async fn render_all_albums_page(context: &RendererContext) -> Result<()> {
    let mut page = Page::new(
        Slug::new("/albums/all"),
        Some("All Albums"),
        Some("All photo albums".to_string()),
    );

    let albums = context
        .data
        .albums
        .find_all_by_date()
        .iter()
        .cloned()
        .cloned()
        .collect::<Vec<Album>>();

    if let Some(album) = albums.first() {
        if let Some(cover_image) = album.cover_images().first() {
            page = page.with_image(cover_image.clone().clone().into());
        }
    }

    let most_recent_updated_at = albums.first().map(|album| album.date);

    let template = AllAlbumsPage { page, albums };

    context
        .renderer
        .render_page(&template.page.slug, &template, most_recent_updated_at)
        .await
}
