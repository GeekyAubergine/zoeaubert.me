use serde::{Deserialize, Serialize};
use tracing::{debug, info};
use url::Url;

use crate::{
    domain::models::{blog_post::BlogPost, slug::Slug, tag::Tag},
    error::BlogPostError,
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

pub const BLOG_POSTS_DIR: &str = "blog-posts";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPostFileFrontMatter {
    slug: String,
    date: String,
    title: String,
    description: String,
    tags: Vec<String>,
    hero: Option<String>,
    #[serde(rename = "heroAlt")]
    hero_alt: Option<String>,
    #[serde(rename = "heroWidth")]
    hero_width: Option<u32>,
    #[serde(rename = "heroHeight")]
    hero_height: Option<u32>,
}

pub fn front_matter_from_string(s: &str) -> Result<BlogPostFileFrontMatter> {
    serde_yaml::from_str(s).map_err(BlogPostError::unparsable_front_matter)
}

struct ProcessBlogPost {
    file_path: ContentFile,
}

impl Task for ProcessBlogPost {
    type Output = BlogPost;

    fn run(self, ctx: &ServiceContext) -> Result<Self::Output> {
        let file_contents = self.file_path.read_text()?;

        let split = file_contents.split("---").collect::<Vec<&str>>();

        let front_matter = split.get(1);
        let front_matter_len = front_matter.map(|s| s.len()).unwrap_or(0);

        let content = file_contents.get(front_matter_len + 6..);

        match (front_matter, content) {
            (Some(front_matter), Some(content)) => {
                let front_matter = front_matter_from_string(front_matter)?;

                let tags = front_matter
                    .tags
                    .iter()
                    .map(|tag| Tag::from_string(tag))
                    .collect::<Vec<Tag>>();

                let date = parse_date(front_matter.date.as_str())?;

                let slug = Slug::new(&format!("/blog/{}", front_matter.slug));

                let mut post = BlogPost::new(
                    slug.clone(),
                    date,
                    front_matter.title,
                    front_matter.description,
                    tags,
                    content.to_owned().to_owned(),
                );

                if let (Some(url), Some(alt), Some(width), Some(height)) = (
                    front_matter.hero,
                    front_matter.hero_alt,
                    front_matter.hero_width,
                    front_matter.hero_height,
                ) {
                    let url: Url = url.parse().unwrap();
                    let cdn_file = CdnFile::from_str(&url.path());

                    let image = MediaService::image_from_url(
                        ctx,
                        &url,
                        &cdn_file,
                        &alt,
                        Some(&&slug.relative_string()),
                        Some(date),
                    )?;

                    post = post.with_hero_image(image);
                }

                post = post.with_images(MediaService::find_images_in_markdown(
                    ctx,
                    content,
                    Some(date),
                    Some(&slug.permalink_string()),
                )?);

                Ok(post)
            }
            _ => Err(BlogPostError::unparsable_blog_post()),
        }
    }
}

pub fn load_blog_posts(ctx: &ServiceContext) -> Result<Vec<BlogPost>> {
    info!("Processing Blog Posts");

    let blog_posts_files =
        FileService::content(BLOG_POSTS_DIR.into()).find_files_recursive("md")?;

    let tasks = blog_posts_files
        .iter()
        .map(|file_path| ProcessBlogPost {
            file_path: FileService::content(file_path.into()),
        })
        .collect();

    run_tasks(tasks, ctx)
}
