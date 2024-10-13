use crate::domain::models::mastodon_post::MastodonPost;
use crate::domain::{repositories::MastodonPostsRepo, state::State};

use crate::prelude::*;

pub async fn find_all_mastodon_posts(state: &impl State) -> Result<Vec<MastodonPost>> {
    state.mastodon_posts_repo().find_all().await
}

pub async fn commit_mastodon_post(state: &impl State, mastodon_post: &MastodonPost) -> Result<()> {
    state.mastodon_posts_repo().commit(mastodon_post).await
}
