use crate::domain::repositories::MicroPostsRepo;
use crate::domain::{models::micro_post::MicroPost, state::State};

use crate::prelude::*;

pub async fn find_all_micro_posts(state: &impl State) -> Result<Vec<MicroPost>> {
    state.micro_posts_repo().find_all().await
}

pub async fn commit_micro_post(state: &impl State, post: &MicroPost) -> Result<()> {
    state.micro_posts_repo().commit(post).await
}
