use std::collections::HashMap;

use uuid::Uuid;

use crate::domain::models::media::Media;
use crate::infrastructure::repos::micro_posts_repo::MicroPostsRepo;
use crate::{domain::models::micro_post::MicroPost, infrastructure::app_state::AppState};

use crate::prelude::Result;

pub async fn find_micro_posts_by_date(
    repo: &impl MicroPostsRepo,
) -> Result<Vec<MicroPost>> {
    repo.find_all_by_date().await
}

pub async fn find_micro_post_by_slug(
    repo: &impl MicroPostsRepo,
    slug: &str,
) -> Result<Option<MicroPost>> {
    repo.find_by_slug(slug).await
}

pub async fn commit_micropost(post: &MicroPost, repo: &impl MicroPostsRepo) -> Result<()> {
    repo.commit(post).await
}
