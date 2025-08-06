use dotenvy_macro::dotenv;
use serde::Deserialize;
use tracing::{field::debug, info};
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
    prelude::*,
    services::{
        cdn_service::CdnFile,
        file_service::{ContentFile, FileService, ReadableFile},
        media_service::MediaService,
        ServiceContext,
    },
    utils::{
        date::parse_date,
        resize_image::{self, resize_image, ResizingConstraint},
    },
};

const ALBUMS_POSTS_DIR: &str = "albums";

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

pub async fn process_album(ctx: &ServiceContext, file: ContentFile) -> Result<Album> {
    info!("Processing album: [{}]", file);

    let yaml: FileAlbum = file.read_yaml()?;

    let file_name = file
        .as_path_buff()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .replace(".yml", "");

    let date = parse_date(&yaml.date)?;

    let slug_date = date.format("%Y/%m").to_string();

    let album_slug = Slug::new(&format!("albums/{}/{}", slug_date, file_name));

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

        let cdn_file = CdnFile::from_date_and_file_name(&date, url.as_str(), None);

        let image =
            MediaService::image_from_url(ctx, &url, &cdn_file, &photo.alt, Some(&album.slug))
                .await?;

        let file_name = url.path_segments().unwrap().last().unwrap();
        let file_name_without_extension = file_name.split('.').next().unwrap();

        let photo_slug = album
            .slug
            .append(&format!("{}", file_name_without_extension));

        let photo = AlbumPhoto::new(photo_slug, photo.description, album.date, tags, image)
            .set_featured(photo.featured.unwrap_or(false));

        album.photos.push(photo);
    }

    Ok(album)
}

pub async fn process_albums(ctx: &ServiceContext) -> Result<Albums> {
    let files = FileService::content(ALBUMS_POSTS_DIR.into()).find_files_recursive("yml")?;

    let mut albums = Albums::default();

    // for file in files {
    //     let file = FileService::content(file.into());

    //     albums.commit(&process_album(ctx, file).await?);
    // }

    Ok(albums)
}
