use crate::domain::repositories::BlogPostsRepo;
use crate::domain::{models::blog_post::BlogPost, state::State};
use crate::prelude::*;

pub async fn find_all_blog_posts(state: &impl State) -> Result<Vec<BlogPost>> {
    state.blog_posts_repo().find_all().await
}

pub async fn commit_blog_post(state: &impl State, blog_post: &BlogPost) -> Result<()> {
    state.blog_posts_repo().commit(blog_post).await
}
