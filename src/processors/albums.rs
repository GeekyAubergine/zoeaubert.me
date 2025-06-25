use dotenvy_macro::dotenv;
use serde::Deserialize;
use tracing::info;
use url::Url;

use crate::{
    domain::models::{
        albums::{album::Album, album_photo::AlbumPhoto, Albums},
        image::Image,
        media::{MediaDimensions, MediaOrientation},
        slug::Slug,
        tag::Tag,
    },
    error::AlbumError,
    utils::{
        date::parse_date,
        resize_image::{self, resize_image, ResizingConstraint},
    },
    prelude::*,
    services::{file_service::FilePath, ServiceContext},
};

const ALBUMS_POSTS_DIR: &str = "albums";

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

pub async fn process_album(ctx: &ServiceContext, file_path: FilePath) -> Result<Album> {
    let yaml: FileAlbum = file_path.read_as_yaml().await?;

    let file_name = file_path
        .file_name()
        .ok_or(AlbumError::invalid_file_name(file_path.clone()))?
        .to_str()
        .ok_or(AlbumError::invalid_file_name(file_path.clone()))?
        .replace(".yml", "");

    let date = parse_date(&yaml.date)?;

    let slug_date = date.format("%Y/%m").to_string();

    let album_slug = Slug::new(&format!("albums/{}/{}", slug_date, file_name));

    info!("Processing album: {:?}", album_slug);

    let mut album = Album::new(album_slug.clone(), yaml.title, yaml.description, date);

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
            FilePath::path_from_date_and_file_name(&date, url.as_str(), None);

        let cdn_path_large_size =
            FilePath::path_from_date_and_file_name(&date, url.as_str(), Some("large"));

        let cdn_path_small_size =
            FilePath::path_from_date_and_file_name(&date, url.as_str(), Some("small"));

        let original_image = ctx
            .image
            .copy_image_from_url(ctx, &url, &cdn_path_original_size, &photo.alt)
            .await?
            .with_date(&date)
            .with_description(&photo.description)
            .with_parent_slug(&album.slug);

        let large_dimensions = image_size_resized_large(&original_image);

        let small_dimensions = image_size_resized_small(&original_image);

        let large_image = ctx
            .image
            .copy_and_resize_image_from_url(
                ctx,
                &url,
                &cdn_path_large_size,
                &photo.alt,
                &large_dimensions,
            )
            .await?
            .with_date(&date)
            .with_description(&photo.description)
            .with_parent_slug(&album.slug);

        let small_image = ctx
            .image
            .copy_and_resize_image_from_url(
                ctx,
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
        )
        .set_featured(photo.featured.unwrap_or(false));

        album.photos.push(photo);
    }

    Ok(album)
}

pub async fn process_albums(ctx: &ServiceContext) -> Result<Albums> {
    let files = FilePath::content(ALBUMS_POSTS_DIR).find_recurisve_files("yml")?;

    let mut albums = Albums::default();

    for file in files {
        albums.commit(&process_album(ctx, file).await?);
    }

    Ok(albums)
}
