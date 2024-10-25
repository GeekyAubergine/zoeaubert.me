use serde::Deserialize;
use std::path::Path;
use tracing::{debug, info};
use uuid::Uuid;

use crate::{
    application::commands::blog_posts_commands::update_blog_post_command::{
        self, update_blog_post_command,
    },
    domain::{
        models::{blog_post::BlogPost, image::Image, tag::Tag},
        services::FileService,
        state::State,
    },
    error::{BlogPostError, Error},
    prelude::*,
};

const BLOG_POSTS_DIR: &str = "blogPosts";

pub async fn update_blog_posts_command(state: &impl State) -> Result<()> {
    let blog_posts_files = state
        .file_service()
        .find_files_rescurse(
            &state
                .file_service()
                .make_content_file_path(&Path::new(BLOG_POSTS_DIR)),
            "md",
        )
        .await?;

    for file_path in blog_posts_files {
        let file_path = Path::new(&file_path);
        update_blog_post_command(state, &file_path).await?;
    }

    Ok(())
}
