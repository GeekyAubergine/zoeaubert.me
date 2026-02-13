use serde::Deserialize;
use tracing::info;
use url::Url;

use crate::{
    config::CONFIG,
    domain::models::{
        albums::{Albums, album::Album, album_photo::AlbumPhoto},
        slug::Slug,
        tag::Tag,
    },
    prelude::*,
    processors::tasks::{Task, run_tasks},
    services::{
        ServiceContext,
        cdn_service::CdnFile,
        file_service::{ContentFile, FileService, ReadableFile},
        media_service::MediaService,
    },
    utils::date::parse_date,
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

struct ProcessAlbumPhoto<'l> {
    photo: FileAlbumPhoto,
    album: &'l Album,
}

impl<'l> Task for ProcessAlbumPhoto<'l> {
    type Output = AlbumPhoto;

    fn run(self, ctx: &ServiceContext) -> Result<Self::Output> {
        let url: Url = format!("{}{}", CONFIG.cdn_url, self.photo.url)
            .parse()
            .unwrap();

        let tags = self
            .photo
            .tags
            .iter()
            .map(|t| Tag::from_string(t))
            .collect::<Vec<Tag>>();

        let cdn_file = CdnFile::from_date_and_file_name(&self.album.date, url.as_str(), None);

        let image = MediaService::image_from_url(
            ctx,
            &url,
            &cdn_file,
            &self.photo.alt,
            Some(&self.album.slug.permalink_string()),
            Some(self.album.date),
        )?;

        let file_name = url.path_segments().unwrap().next_back().unwrap();
        let file_name_without_extension = file_name.split('.').next().unwrap();

        let photo_slug = self
            .album
            .slug
            .append(&file_name_without_extension.to_string());

        let photo = AlbumPhoto::new(
            photo_slug,
            self.photo.description,
            self.album.date,
            tags,
            image,
        )
        .set_featured(self.photo.featured.unwrap_or(false));

        Ok(photo)
    }
}

pub fn process_album(ctx: &ServiceContext, file: ContentFile) -> Result<Album> {
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

    let tasks = yaml
        .photos
        .into_iter()
        .map(|photo| ProcessAlbumPhoto {
            album: &album,
            photo,
        })
        .collect();

    let photos = run_tasks(tasks, ctx)?;

    for photo in photos {
        album.photos.push(photo);
    }

    album
        .photos
        .sort_by(|a, b| a.slug.as_str().cmp(b.slug.as_str()));

    Ok(album)
}

pub fn load_albums(ctx: &ServiceContext) -> Result<Albums> {
    info!("Processing Albums");

    let files = FileService::content(ALBUMS_POSTS_DIR.into()).find_files_recursive("yml")?;

    let mut albums = Albums::default();

    for file in files {
        let file = FileService::content(file.into());

        albums.commit(&process_album(ctx, file)?);
    }

    Ok(albums)
}
