use askama::Template;

use crate::domain::models::omni_post::OmniPost;
use crate::domain::models::page::Page;
use crate::domain::models::slug::Slug;
use crate::domain::repositories::BlogPostsRepo;
use crate::domain::services::PageRenderingService;
use crate::domain::{models::blog_post::BlogPost, state::State};

use crate::prelude::*;

use crate::domain::models::media::Media;
use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

#[derive(Template)]
#[template(path = "blog_post_list.html")]
struct BlogsListTemplate {
    page: Page,
    blog_posts: Vec<BlogPost>,
}

pub async fn render_blog_list_page(state: &impl State) -> Result<()> {
    let blog_posts = state.blog_posts_repo().find_all_by_date().await?;

    let page = Page::new(
        Slug::new("/blog"),
        Some("Blog Posts"),
        Some("My blog posts"),
    );

    let template = BlogsListTemplate {
        page,
        blog_posts: blog_posts.to_vec(),
    };

    let most_recent_blog_post = blog_posts.first().map(|p| p.updated_at);

    state
        .page_rendering_service()
        .add_page(
            state,
            template.page.slug.clone(),
            template,
            most_recent_blog_post.as_ref(),
        )
        .await
}

// #[derive(Template)]
// #[template(path = "omni_post/omni_post_page/omni_post_page.html")]
// struct BlogPostTemplate {
//     page: Page,
//     omni_post: OmniPost,
// }

// async fn render_blog_post_page(state: &impl State, blog_post: BlogPost) -> Result<()> {
//     let mut page = Page::new(
//         blog_post.slug.clone(),
//         Some(&blog_post.title),
//         Some(&blog_post.description),
//     )
//     .with_date(blog_post.date)
//     .with_tags(blog_post.tags.clone());

//     if let Some(image) = &blog_post.hero_image {
//         page = page.with_image(image.clone().into());
//     }

//     let updated_at = Some(blog_post.updated_at);

//     let template = BlogPostTemplate {
//         page,
//         omni_post: blog_post.into(),
//     };

//     state
//         .page_rendering_service()
//         .add_page(
//             state,
//             template.page.slug.clone(),
//             template,
//             updated_at.as_ref(),
//         )
//         .await
// }
