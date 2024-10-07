use askama::Template;

use crate::models::{blog_post::BlogPost, page::Page};
use crate::prelude::*;

use super::render_page_to_file;

use crate::renderers::formatters::format_date::FormatDate;
use crate::renderers::formatters::format_number::FormatNumber;
use crate::renderers::formatters::format_markdown::FormatMarkdown;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'t, 'p> {
    page: &'t Page<'p>,
    about_text: String,
    silly_names: Vec<String>,
    recent_blog_posts: Vec<BlogPost>,
}

pub async fn render_home_page() -> Result<()> {
    let page = Page::new("/", None, None);

    // let about_text = state.about_repo().get().await.short().to_owned();

    // let silly_names = SillyNamesQueryService::find_all(&state)
    //     .await?
    //     .values()
    //     .map(|n| n.name.to_owned())
    //     .collect();

    // let recent_blog_posts = state
    //     .blog_posts_repo()
    //     .get_all_by_published_date()
    //     .await
    //     .iter()
    //     .take(RECENT_POSTS_COUNT)
    //     .cloned()
    //     .collect::<Vec<_>>();

    let template = IndexTemplate {
        page: &page,
        silly_names: vec![],
        about_text: ";sldkjflksdjf".to_string(),
        recent_blog_posts: vec![],
    };

    let rendered = template.render().unwrap();

    render_page_to_file(&page, &rendered).await
}
