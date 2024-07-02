use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
    Router,
};

use crate::{
    build_data,
    domain::models::{
        mastodon_post::MastodonPost,
        media::{image::Image, Media},
        micro_post::MicroPost,
        microblog_archive::MicroblogArchivePost,
        page::Page,
        tag::Tag,
    },
    infrastructure::app_state::AppState,
};

pub use crate::infrastructure::services::date::FormatDate;
pub use crate::infrastructure::services::markdown::FormatMarkdown;
pub use crate::infrastructure::services::number::FormatNumber;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/:year/:month/:day/:title",
            get(old_micropost_slug_redirect),
        )
        .route(
            "/:year/:month/:day/:title/",
            get(old_micropost_slug_redirect),
        )
        .route("/:year/:month/:id", get(old_mastodon_slug_redirect))
        .route("/:year/:month/:id/", get(old_mastodon_slug_redirect))
        .route("/:slug", get(post_page))
}

async fn old_micropost_slug_redirect(
    Path((year, month, day, title)): Path<(String, String, String, String)>,
    State(state): State<AppState>,
) -> Redirect {
    Redirect::permanent(&format!("/micros/{}-{}-{}-{}", year, month, day, title))
}

async fn old_mastodon_slug_redirect(
    Path((_, _, id)): Path<(String, String, String)>,
    State(state): State<AppState>,
) -> Redirect {
    Redirect::permanent(&format!("/micros/{}", id))
}

pub enum Post {
    MicroPost(MicroPost),
    MicroBlogArchive(MicroblogArchivePost),
    MastodonPost(MastodonPost),
}

impl Post {
    pub fn slug(&self) -> &str {
        match self {
            Self::MicroPost(micro_post) => micro_post.slug(),
            Self::MicroBlogArchive(archive_post) => archive_post.slug(),
            Self::MastodonPost(mastodon_post) => mastodon_post.id(),
        }
    }

    pub fn date(&self) -> &chrono::DateTime<chrono::Utc> {
        match self {
            Self::MicroPost(micro_post) => micro_post.date(),
            Self::MicroBlogArchive(archive_post) => archive_post.date(),
            Self::MastodonPost(mastodon_post) => mastodon_post.created_at(),
        }
    }

    pub fn content(&self) -> &str {
        match self {
            Self::MicroPost(micro_post) => micro_post.content(),
            Self::MicroBlogArchive(archive_post) => archive_post.content(),
            Self::MastodonPost(mastodon_post) => mastodon_post.content(),
        }
    }

    pub fn tags(&self) -> &Vec<Tag> {
        match self {
            Self::MicroPost(micro_post) => micro_post.tags(),
            Self::MicroBlogArchive(archive_post) => archive_post.tags(),
            Self::MastodonPost(mastodon_post) => mastodon_post.tags(),
        }
    }

    pub fn media(&self) -> Vec<Media> {
        match self {
            Self::MicroPost(micro_post) => vec![],
            Self::MicroBlogArchive(archive_post) => vec![],
            Self::MastodonPost(mastodon_post) => mastodon_post.media().to_owned(),
        }
    }

    pub fn original_url(&self) -> Option<&str> {
        match self {
            Self::MicroPost(_) => None,
            Self::MicroBlogArchive(_) => None,
            Self::MastodonPost(mastodon_post) => Some(mastodon_post.original_uri()),
        }
    }
}

#[derive(Template)]
#[template(path = "micro_posts/post.html")]
pub struct PostTemplate {
    page: Page,
    post: Post,
}

async fn post_page(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<PostTemplate, (StatusCode, &'static str)> {
    let micro_post = state.micro_posts_repo().get_by_slug(&id).await;

    let archive_post = state.microblog_archive_repo().get_by_slug(&id).await;

    let mastodon_post = state.mastodon_posts_repo().get_by_id(&id).await;

    let post = match (micro_post, archive_post, mastodon_post) {
        (Some(micro_post), _, _) => Post::MicroPost(micro_post),
        (_, Some(archive_post), _) => Post::MicroBlogArchive(archive_post),
        (_, _, Some(mastodon_post)) => Post::MastodonPost(mastodon_post),
        _ => return Err((StatusCode::NOT_FOUND, "Post not found")),
    };

    let page = Page::new(
        state.site(),
        &format!("/micros/{}", post.slug()),
        None,
        None,
    )
    .with_date(*post.date())
    .with_tags(post.tags().to_vec());

    Ok(PostTemplate { page, post })
}
