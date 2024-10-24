use chrono::{DateTime, Datelike, Utc};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use std::path::Path;
use tracing::info;

use crate::{
    domain::{
        models::{image::Image, media::Media, micro_post::MicroPost, slug::Slug, tag::Tag},
        repositories::{MicroPostsRepo, Profiler},
        services::{CdnService, FileService},
        state::State,
    },
    prelude::*,
};

const MICRO_POSTS_DIR: &str = "microBlogArchive/feed.json";

pub const HTML_IMAGE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?i)<img(((src="(?<src>([^"]+))")|(alt="(?<alt>([^"]+))")|(width="(?<width>([^"]+))")|(height="(?<height>([^"]+))"))|[^>])*>"#).unwrap()
});

const TAGS_TO_IGNORE: [&str; 2] = ["status", "photography"];
const SELF_URL: &str = "zoeaubert.me";

#[derive(Debug, Clone, Deserialize)]
struct ArchiveFileItem {
    id: String,
    // content_html: String,
    content_text: String,
    date_published: DateTime<Utc>,
    // url: String,
    tags: Option<Vec<String>>,
}

impl ArchiveFileItem {
    fn tags_mut(&mut self) -> &mut Option<Vec<String>> {
        &mut self.tags
    }
}

#[derive(Debug, Clone, Deserialize)]
struct ArchiveFile {
    // version: String,
    // title: String,
    // home_page_url: String,
    // feed_url: String,
    items: Vec<ArchiveFileItem>,
}

fn extract_images_from_html(markup: &str, date: &DateTime<Utc>, parent_slug: &Slug) -> Vec<Image> {
    let mut images = vec![];

    for cap in HTML_IMAGE_REGEX.captures_iter(markup) {
        let src = cap.name("src").map_or("", |m| m.as_str());
        let alt = cap.name("alt").map_or("", |m| m.as_str());
        let width = cap.name("width").map_or("", |m| m.as_str());
        let height = cap.name("height").map_or("", |m| m.as_str());

        let width = width.parse::<u32>().unwrap_or(0);
        let height = height.parse::<u32>().unwrap_or(0);

        let path = src.replace("uploads/", "");

        let path = Path::new(&path);

        images.push(
            Image::new(&path, alt, width, height)
                .with_date(date)
                .with_parent_slug(parent_slug),
        );
    }

    images
}

fn slug_for_item(item: &ArchiveFileItem) -> Slug {
    let id = item
        .id
        .split('/')
        .skip(6)
        .collect::<Vec<&str>>()
        .join("/")
        .replace(".html", "");

    let slug_date = item.date_published.format("%Y/%m/%d").to_string();

    let slug = format!("micros/{}/{}", slug_date, id);

    Slug::new(&slug)
}

fn archive_item_to_post(
    item: ArchiveFileItem,
    updated_at: DateTime<Utc>,
) -> Result<Option<MicroPost>> {
    let slug = slug_for_item(&item);

    let tags: Vec<String> = match item.tags {
        Some(tags) => tags,
        None => vec![],
    };

    if tags
        .iter()
        .any(|tag| TAGS_TO_IGNORE.contains(&tag.to_lowercase().as_str()))
    {
        return Ok(None);
    }

    let tags = tags.iter().map(|t| Tag::from_string(t)).collect();

    let content = item
        .content_text
        .replace("uploads/", "https://cdn.geekyaubergine.com/");

    if content.contains(SELF_URL) {
        return Ok(None);
    }

    let media = extract_images_from_html(&content, &item.date_published, &slug)
        .iter()
        .map(|i| Media::from(i))
        .collect();

    Ok(Some(MicroPost::new(
        slug.clone(),
        item.date_published,
        content,
        media,
        tags,
        updated_at,
    )))
}

pub async fn update_micro_blog_archive_posts_command(state: &impl State) -> Result<()> {
    info!("Updating micro blog archive posts");

    let archive_file: ArchiveFile = state
        .file_service()
        .read_json_file(
            &state
                .file_service()
                .make_content_file_path(&Path::new(MICRO_POSTS_DIR)),
        )
        .await?;

    let last_modified = state
        .file_service()
        .get_file_last_modified(
            &state
                .file_service()
                .make_content_file_path(&Path::new(MICRO_POSTS_DIR)),
        )
        .await?;

    for item in archive_file.items {
        state.profiler().post_processed().await?;

        let slug = slug_for_item(&item);

        let existing = state.micro_posts_repo().find_by_slug(&slug).await?;

        // These never update so if it's here, just skip
        if existing.is_some() {
            continue;
        }

        if let Some(post) = archive_item_to_post(item, last_modified.clone())? {
            state.micro_posts_repo().commit(&post).await?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use url::Url;

    use crate::domain::services::CdnService;

    use super::*;

    #[test]
    fn test_extract_media_from_html() {
        let parent_slug = Slug::new("test-slug");

        let markup = r#"Movie friend\n\n<img src="uploads/2022/ced7ff5352.jpg" width="600" height="450" alt="Picture of my tabby cat called Muffin. She is curled up in a ball with her tail reaching round to her forehead. She is a mix of black and brown fur with white feet. Some of her feet are sticking out. She is sat on a brown-grey textured sofa">\n"#;

        let date = Utc::now();

        let media = extract_images_from_html(markup, &date, &parent_slug);

        assert_eq!(media.len(), 1);

        let expected_path = Path::new("2022/ced7ff5352.jpg");

        let expected = Image::new(&expected_path,
            "Picture of my tabby cat called Muffin. She is curled up in a ball with her tail reaching round to her forehead. She is a mix of black and brown fur with white feet. Some of her feet are sticking out. She is sat on a brown-grey textured sofa",
            600,
             450,
        ).with_date(&date).with_parent_slug(&parent_slug);

        let image = media.get(0).unwrap();

        assert_eq!(image.path, expected.path);
        assert_eq!(image.alt, expected.alt);
        assert_eq!(image.width, expected.width);
        assert_eq!(image.height, expected.height);
    }
}
