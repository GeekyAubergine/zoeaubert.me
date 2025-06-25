use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;

use crate::{
    domain::models::{
        image::Image,
        media::{Media, MediaDimensions},
        micro_post::MicroPost,
        slug::Slug,
        tag::Tag,
    },
    prelude::*,
    services::{file_service::FilePath, ServiceContext},
};

const MICRO_POSTS_DIR: &str = "micro-blog-archive/feed.json";

pub const HTML_IMAGE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?i)<img(((src="(?<src>([^"]+))")|(alt="(?<alt>([^"]+))")|(width="(?<width>([^"]+))")|(height="(?<height>([^"]+))"))|[^>])*>"#).unwrap()
});
pub const MARKDOWN_LINK_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"\(https?://[^\s]+\)"#).unwrap());

const TAGS_TO_IGNORE: [&str; 2] = ["status", "photography"];
const SELF_URL: &str = "zoeaubert.me";

#[derive(Debug, Clone, Deserialize, Hash)]
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

// Find first line or sentence. Remove markdown links and html tags.
fn extract_description(markup: &str) -> Option<String> {
    let lines = markup
        .split("\\n")
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();

    let first_line = lines.iter().next()?;

    let first_line = first_line.replace("[", "").replace("]", "");

    let first_line = MARKDOWN_LINK_REGEX.replace_all(&first_line, "");

    if first_line.contains("<") {
        let first_line = first_line.split('<').collect::<Vec<&str>>().join("");
    }

    let sentences = first_line.split('.').collect::<Vec<&str>>();

    let first_sentence = sentences.iter().next()?;

    return Some(first_sentence.to_string());
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

        let path = FilePath::content(&path);

        images.push(
            Image::new(&path, alt, &MediaDimensions::new(width, height))
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

fn archive_item_to_post(item: ArchiveFileItem) -> Result<Option<MicroPost>> {
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

    let description = extract_description(&content);

    let media = extract_images_from_html(&content, &item.date_published, &slug)
        .iter()
        .map(|i| Media::from(i))
        .collect();

    Ok(Some(MicroPost::new(
        slug.clone(),
        item.date_published,
        content,
        description,
        media,
        tags,
    )))
}

pub async fn process_micro_blog_archive(
    ctx: &ServiceContext,
) -> Result<Vec<MicroPost>> {
    let archive_file: ArchiveFile = FilePath::content(MICRO_POSTS_DIR).read_as_json().await?;

    let mut posts = vec![];

    for item in archive_file.items {
        if let Some(post) = archive_item_to_post(item)? {
            posts.push(post);
        }
    }

    Ok(posts)
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use url::Url;

    use super::*;

    #[test]
    fn it_should_extract_media_from_html() {
        let parent_slug = Slug::new("test-slug");

        let markup = r#"Movie friend\n\n<img src="uploads/2022/ced7ff5352.jpg" width="600" height="450" alt="Picture of my tabby cat called Muffin. She is curled up in a ball with her tail reaching round to her forehead. She is a mix of black and brown fur with white feet. Some of her feet are sticking out. She is sat on a brown-grey textured sofa">\n"#;

        let date = Utc::now();

        let media = extract_images_from_html(markup, &date, &parent_slug);

        assert_eq!(media.len(), 1);

        let expected_path = FilePath::cache("2022/ced7ff5352.jpg");

        let expected = Image::new(&expected_path,
            "Picture of my tabby cat called Muffin. She is curled up in a ball with her tail reaching round to her forehead. She is a mix of black and brown fur with white feet. Some of her feet are sticking out. She is sat on a brown-grey textured sofa",
            &MediaDimensions::new(600, 450),
        ).with_date(&date).with_parent_slug(&parent_slug);

        let image = media.get(0).unwrap();

        assert_eq!(image.path, expected.path);
        assert_eq!(image.alt, expected.alt);
        assert_eq!(image.dimensions.width, expected.dimensions.width);
        assert_eq!(image.dimensions.height, expected.dimensions.height);
    }

    #[test]
    fn it_should_extract_description() {
        let markup = r#"Movie friend\n\n<img src="uploads/2022/ced7ff5352.jpg" width="600" height="450" alt="Picture of my tabby cat called Muffin. She is curled up in a ball with her tail reaching round to her forehead. She is a mix of black and brown fur with white feet. Some of her feet are sticking out. She is sat on a brown-grey textured sofa">\n"#;

        let expected = Some(String::from("Movie friend"));

        assert_eq!(extract_description(&markup), expected);

        let markup = r#"Finished my [Goff Rocker](https://www.games-workshop.com/en-GB/ork-goff-rocker-xmas-promo-2022). Pretty pleased with the result; still a few little touch-ups to do, but for my first Ork, I'm very happy, the skin is a lot of fun to highlight.\n\n<img src=\"uploads/2022/e09fcfa66a.jpg\" width=\"600\" height=\"600\" alt=\"Several angles of my painted Goff Rocker Ork miniature. The ork is posed with one foot raised on a squig, their left arm holding a microphone up to sing into and th other holding a guitar that's strapped to their back. He is wearing biking leathers and a hat. The orks skin is painted a dark green and is fully shaded and highlighted. The highlights maintain the colour without becoming yellow. The leathers are black with medium grey highlights. The squig is painted red with orange-red highlights. The guitar has been painted a hot yellow. Various metal accents are in a mix of bronze and silver colours. The base has been left a simple flat black\">\n"#;

        let expected = Some(String::from("Finished my Goff Rocker"));

        assert_eq!(extract_description(markup), expected);
    }
}
