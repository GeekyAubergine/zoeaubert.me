use askama::Template;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
    Router,
};
use tracing::error;

use crate::{
    domain::{
        models::{
            page::{Page, PagePagination, PagePaginationLabel},
            tag::{Tag, TagSlug},
        },
        omni_post::{omni_post_models::OmniPost, omni_post_repo::OmniPostRepo},
    },
    infrastructure::app_state::AppState,
    routes::Pagination,
};

use crate::utils::{FormatDate, FormatMarkdown, FormatNumber};

const POSTS_PER_PAGE: usize = 25;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/:tag", get(tag))
}

pub struct TagPair {
    tag: Tag,
    count: usize,
}

impl TagPair {
    pub fn tag(&self) -> &Tag {
        &self.tag
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

#[derive(Template)]
#[template(path = "tags/index.html")]
pub struct IndexTemplate {
    page: Page,
    tags: Vec<TagPair>,
}
async fn index(State(state): State<AppState>) -> Result<IndexTemplate, (StatusCode, &'static str)> {
    let tags = OmniPostRepo::get_posts_tags_and_counts(&state)
        .await
        .map_err(|e| {
            error!("Failed to get posts ordered by date: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get posts ordered by date",
            )
        })?;

    let mut tags: Vec<TagPair> = tags
        .iter()
        .map(|(tag, count)| TagPair {
            tag: tag.clone(),
            count: *count,
        })
        .collect();

    tags.sort_by(|a, b| a.tag().cmp(b.tag()));

    let page = Page::new(
        state.site(),
        "/tags",
        Some("Tags"),
        Some("All Tags"),
        None,
        None,
        None,
        vec![],
    )
    .set_no_index();

    Ok(IndexTemplate { page, tags })
}

#[derive(Template)]
#[template(path = "tags/tag.html")]
pub struct TagTemplate {
    page: Page,
    posts: Vec<OmniPost>,
}

async fn tag(
    Path(tag): Path<String>,
    State(state): State<AppState>,
    pagination: Query<Pagination>,
) -> Result<TagTemplate, (StatusCode, &'static str)> {
    println!("tag: {:?}", tag);
    let tag = &TagSlug::from_string(&tag).to_tag();
    println!("tag: {:?}", tag);

    let posts = OmniPostRepo::get_posts_by_tag_ordered_by_date(&state, tag)
        .await
        .map_err(|e| {
            error!("Failed to get posts ordered by date: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get posts ordered by date",
            )
        })?;

    let total_posts_count = posts.len();

    let posts = posts
        .iter()
        .skip((pagination.page() - 1) * pagination.per_page())
        .take(POSTS_PER_PAGE)
        .cloned()
        .collect::<Vec<OmniPost>>();

    let previous_nav = match total_posts_count > pagination.page() * pagination.per_page() {
        true => Some(PagePaginationLabel::new(
            &format!("/tags/{}?page={}", tag.slug(), pagination.page() + 1),
            "Older posts",
        )),
        false => None,
    };

    let next_nav = match pagination.page() {
        1 => None,
        _ => Some(PagePaginationLabel::new(
            &format!("/tags/{}?page={}", tag.slug(), pagination.page() - 1),
            "Newer posts",
        )),
    };

    let page = Page::new(
        state.site(),
        &format!("/tags/{}", tag.slug()),
        Some(&format!("{} Posts", tag.title())),
        Some(&format!("#{} posts", tag.title())),
        None,
        None,
        None,
        vec![],
    )
    .set_no_index()
    .with_pagination(PagePagination::new(previous_nav, next_nav));

    Ok(TagTemplate { page, posts })
}
