use askama::Template;

use crate::domain::models::omni_post::OmniPost;
use crate::domain::models::slug::Slug;
use crate::domain::models::tag::Tag;
use crate::domain::models::{blog_post::BlogPost, page::Page};
use crate::domain::queries::omni_post_queries::{find_all_omni_posts, OmniPostFilterFlags};
use crate::domain::repositories::{AboutTextRepo, BlogPostsRepo, SillyNamesRepo};
use crate::domain::services::PageRenderingService;
use crate::domain::state::State;
use crate::prelude::*;

use crate::domain::models::media::Media;
use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

const RECENT_POSTS_COUNT: usize = 5;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    page: Page,
    about_text: String,
    silly_names: Vec<String>,
    recent_blog_posts: Vec<BlogPost>,
    recent_omni_posts: Vec<OmniPost>,
}

pub async fn render_home_page(state: &impl State) -> Result<()> {
    let page = Page::new(Slug::new("/"), None, None);

    let silly_names = state.silly_names_repo().find_all().await?;

    let about_text = state.about_text_repo().find_short().await?;

    let recent_blog_posts = state
        .blog_posts_repo()
        .find_all_by_date()
        .await?
        .iter()
        .filter(|p| {
            !p.tags
                .contains(&Tag::from_string("MonthlyNotes"))
        })
        .take(RECENT_POSTS_COUNT)
        .cloned()
        .collect::<Vec<_>>();

    let most_recent_post = recent_blog_posts.first().cloned();

    let recent_omni_posts = find_all_omni_posts(state, OmniPostFilterFlags::filter_home_page())
        .await?
        .iter()
        .take(RECENT_POSTS_COUNT)
        .cloned()
        .collect::<Vec<OmniPost>>();

    let template = IndexTemplate {
        page,
        silly_names,
        about_text,
        recent_blog_posts,
        recent_omni_posts,
    };

    let updated_at = most_recent_post.map(|p| p.updated_at);

    state
        .page_rendering_service()
        .add_page(
            state,
            template.page.slug.clone(),
            template,
            updated_at.as_ref(),
        )
        .await
}
