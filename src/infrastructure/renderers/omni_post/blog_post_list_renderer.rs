use askama::Template;

use crate::domain::models::omni_post::OmniPost;
use crate::domain::models::page::Page;
use crate::domain::models::post::PostFilter;
use crate::domain::models::slug::Slug;
use crate::domain::repositories::BlogPostsRepo;
use crate::domain::services::PageRenderingService;
use crate::domain::{models::blog_post::BlogPost, state::State};

use crate::infrastructure::renderers::RendererContext;
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

pub async fn render_blog_list_page(context: &RendererContext) -> Result<()> {
    let blog_posts = context.data.posts.find_all_by_filter(PostFilter::BLOG_POST);

    let blog_posts = blog_posts
        .iter()
        .filter_map(|post| match post {
            OmniPost::BlogPost(post) => Some(post.clone()),
            _ => None,
        })
        .collect::<Vec<BlogPost>>();

    let page = Page::new(
        Slug::new("/blog"),
        Some("Blog Posts"),
        Some("My blog posts".to_string()),
    );

    let most_recent_blog_post = blog_posts.first().map(|p| p.updated_at);

    let template = BlogsListTemplate { page, blog_posts };

    context.renderer.render_page(&template.page.slug, &template, most_recent_blog_post).await
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
