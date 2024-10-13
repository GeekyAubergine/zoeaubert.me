use serde::Deserialize;
use tracing::{debug, info};
use uuid::Uuid;

use crate::{
    application::commands::blog_posts_commands::update_blog_post_command::{
        self, update_blog_post_command,
    },
    domain::{
        models::{blog_post::BlogPost, image::Image, tag::Tag},
        state::State,
    },
    error::{BlogPostError, Error},
    infrastructure::utils::file_system::{find_files_rescurse, make_content_file_path},
    prelude::*,
};

const BLOG_POSTS_DIR: &str = "blogPosts";

pub async fn update_blog_posts_command(state: &impl State) -> Result<()> {
    info!("Updating blog posts");

    let blog_posts_files = find_files_rescurse(&make_content_file_path(BLOG_POSTS_DIR), "md")?;

    for file_path in blog_posts_files {
        update_blog_post_command(state, &file_path).await?;
    }

    Ok(())
}
