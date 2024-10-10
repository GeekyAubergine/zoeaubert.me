use askama::Template;

use crate::domain::models::slug::Slug;
use crate::domain::models::{blog_post::BlogPost, page::Page};
use crate::domain::queries::about_queries::find_about_text_short;
use crate::domain::queries::blog_post_queries::find_all_blog_posts;
use crate::domain::queries::silly_names_queries::find_silly_names;
use crate::domain::state::State;
use crate::prelude::*;

use super::{render_page_with_template};

use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

const RECENT_POSTS_COUNT: usize = 5;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'t, 'p> {
    page: &'t Page<'p>,
    about_text: String,
    silly_names: Vec<String>,
    recent_blog_posts: Vec<BlogPost>,
}

pub async fn render_home_page(state: &impl State) -> Result<()> {
    let page = Page::new(Slug::new("/"), None, None);

    // let about_text = state.about_repo().get().await.short().to_owned();

    // let silly_names = SillyNamesQueryService::find_all(&state)
    //     .await?
    //     .values()
    //     .map(|n| n.name.to_owned())
    //     .collect();

    let silly_names = find_silly_names(state).await?;

    let about_text = find_about_text_short(state).await?;

    let recent_blog_posts = find_all_blog_posts(state)
        .await?
        .iter()
        .take(RECENT_POSTS_COUNT)
        .cloned()
        .collect::<Vec<_>>();

    let template = IndexTemplate {
        page: &page,
        silly_names,
        about_text,
        recent_blog_posts,
    };

    render_page_with_template(&page, template).await
}
