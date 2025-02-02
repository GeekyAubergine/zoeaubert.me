use std::path::Path;

use chrono::Utc;
use serde::Deserialize;
use tracing::info;
use url::Url;

use dotenvy_macro::dotenv;

use crate::{
    calculate_hash,
    domain::{
        models::{
            album::{Album, AlbumPhoto}, image::Image, media::{MediaDimensions, MediaOrientation}, slug::Slug, tag::Tag
        },
        repositories::{AlbumsRepo, Profiler},
        services::{FileService, ImageService},
        state::State,
    },
    error::AlbumError,
    infrastructure::utils::{
        date::parse_date,
        resize_image::{resize_image, ResizingConstraint},
    },
    prelude::*,
};

const LANDSCAPE_SMALL_IMAGE_WIDTH: u32 = 500;
const LANDSCAPE_LARGE_IMAGE_WIDTH: u32 = 2000;

const PORTRAIT_SMALL_IMAGE_WIDTH: u32 = 300;
const PORTRAIT_LARGE_IMAGE_WIDTH: u32 = 1500;

const SQUARE_SMALL_IMAGE_WIDTH: u32 = 400;
const SQUARE_LARGE_IMAGE_WIDTH: u32 = 1500;

fn image_size_resized_small(image: &Image) -> MediaDimensions {
    let target_width = match image.dimensions.orientation() {
        MediaOrientation::Landscape => LANDSCAPE_SMALL_IMAGE_WIDTH,
        MediaOrientation::Portrait => PORTRAIT_SMALL_IMAGE_WIDTH,
        MediaOrientation::Square => SQUARE_SMALL_IMAGE_WIDTH,
    };

    resize_image(
        &image.dimensions,
        &ResizingConstraint::max_width(target_width),
    )
}

fn image_size_resized_large(image: &Image) -> MediaDimensions {
    let target_width = match image.dimensions.orientation() {
        MediaOrientation::Landscape => LANDSCAPE_LARGE_IMAGE_WIDTH,
        MediaOrientation::Portrait => PORTRAIT_LARGE_IMAGE_WIDTH,
        MediaOrientation::Square => SQUARE_LARGE_IMAGE_WIDTH,
    };

    resize_image(
        &image.dimensions,
        &ResizingConstraint::max_width(target_width),
    )
}

#[derive(Debug, Clone, Deserialize, Hash)]
pub struct FileAlbumPhoto {
    pub url: String,
    pub description: String,
    pub alt: String,
    pub tags: Vec<String>,
    pub featured: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Hash)]
pub struct FileAlbum {
    pub title: String,
    pub description: Option<String>,
    pub date: String,
    pub photos: Vec<FileAlbumPhoto>,
}

pub async fn update_album_command(state: &impl State, file_path: &Path) -> Result<()> {
    let yaml: FileAlbum = state.file_service().read_yaml_file(file_path).await?;

    let hash = calculate_hash(&yaml);

    let file_name = file_path
        .file_name()
        .ok_or(AlbumError::invalid_file_name(file_path.to_path_buf()))?
        .to_str()
        .ok_or(AlbumError::invalid_file_name(file_path.to_path_buf()))?
        .replace(".yml", "");

    let date = parse_date(&yaml.date)?;

    let slug_date = date.format("%Y/%m").to_string();

    let album_slug = Slug::new(&format!("albums/{}/{}", slug_date, file_name));

    if let Some(album) = state.albums_repo().find_by_slug(&album_slug).await? {
        if album.original_data_hash == hash {
            return Ok(());
        }
    }

    info!("Processing album: {:?}", album_slug);

    state.profiler().entity_processed().await?;

    let mut album = Album::new(
        album_slug.clone(),
        yaml.title,
        yaml.description,
        date,
        Utc::now(),
        hash,
    );

    for photo in yaml.photos {
        let url: Url = format!("{}{}", dotenv!("CDN_URL"), photo.url)
            .parse()
            .unwrap();

        let tags = photo
            .tags
            .iter()
            .map(|t| Tag::from_string(t))
            .collect::<Vec<Tag>>();

        let cdn_path_original_size =
            state
                .file_service()
                .make_file_path_from_date_and_file(&date, url.as_str(), None);

        let cdn_path_large_size = state.file_service().make_file_path_from_date_and_file(
            &date,
            url.as_str(),
            Some("large"),
        );

        let cdn_path_small_size = state.file_service().make_file_path_from_date_and_file(
            &date,
            url.as_str(),
            Some("small"),
        );

        let original_image = state
            .image_service()
            .copy_image_from_url(state, &url, &cdn_path_original_size, &photo.alt)
            .await?
            .with_date(&date)
            .with_description(&photo.description)
            .with_parent_slug(&album.slug);

        let large_dimensions = image_size_resized_large(&original_image);

        let small_dimensions = image_size_resized_small(&original_image);

        let large_image = state
            .image_service()
            .copy_and_resize_image_from_url(
                state,
                &url,
                &cdn_path_large_size,
                &photo.alt,
                &large_dimensions,
            )
            .await?
            .with_date(&date)
            .with_description(&photo.description)
            .with_parent_slug(&album.slug);

        let small_image = state
            .image_service()
            .copy_and_resize_image_from_url(
                state,
                &url,
                &cdn_path_small_size,
                &photo.alt,
                &small_dimensions,
            )
            .await?
            .with_date(&date)
            .with_description(&photo.description)
            .with_parent_slug(&album.slug);

        let file_name = url.path_segments().unwrap().last().unwrap();
        let file_name_without_extension = file_name.split('.').next().unwrap();

        let photo_slug = album
            .slug
            .append(&format!("{}", file_name_without_extension));

        let photo = AlbumPhoto::new(
            photo_slug,
            photo.description,
            album.date,
            tags,
            small_image,
            large_image,
            original_image,
            Utc::now(),
        )
        .set_featured(photo.featured.unwrap_or(false));

        album.photos.push(photo);
    }

    state.albums_repo().commit(&album).await?;

    Ok(())
}
