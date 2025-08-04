use std::collections::HashMap;

use askama::Template;
use chrono::Utc;

use crate::{
    domain::{
        models::{post::Post, page::Page, post::PostFilter, slug::Slug},
    },
        renderers::RendererContext,
        utils::paginator::{paginate, PaginatorPage},
    prelude::*,
};

use crate::domain::models::media::Media;
use crate::renderers::formatters::format_date::FormatDate;
use crate::renderers::formatters::format_markdown::FormatMarkdown;
use crate::renderers::formatters::format_number::FormatNumber;

use crate::domain::models::tag::Tag;

const DEFAULT_PAGINATION_SIZE: usize = 25;

fn group_posts_by_tag(posts: Vec<&Post>) -> HashMap<Tag, Vec<&Post>> {
    let mut grouped = HashMap::new();

    for post in posts {
        for tag in post.tags() {
            grouped.entry(tag).or_insert_with(Vec::new).push(post);
        }
    }

    grouped
}

pub struct TagCount {
    pub tag: Tag,
    pub count: usize,
}

pub async fn render_tags_pages(context: &RendererContext) -> Result<()> {
    let posts = context
        .data
        .posts
        .find_all_by_filter(PostFilter::filter_tags_page());

    let grouped = group_posts_by_tag(posts);

    let tag_counts = grouped
        .iter()
        .map(|(tag, posts)| (tag.clone(), posts.len()))
        .collect::<HashMap<Tag, usize>>();

    render_tags_list_page(context, &tag_counts).await?;

    for (tag, posts) in grouped {
        let paginated = paginate(&posts, DEFAULT_PAGINATION_SIZE);

        let base_page = Page::new(
            Slug::new(&format!("tags/{}", tag.slug())),
            Some(&format!("{} Posts", tag.title())),
            Some(format!("#{} posts", tag.title())),
        );

        for page in paginated {
            render_tag_page(context, &base_page, &tag, &page).await?;
        }
    }

    Ok(())
}

#[derive(Template)]
#[template(path = "tags/index.html")]
struct TagsListTemplate {
    page: Page,
    tags: Vec<TagCount>,
}

async fn render_tags_list_page(
    context: &RendererContext,
    tag_counts: &HashMap<Tag, usize>,
) -> Result<()> {
    let mut tags = tag_counts
        .into_iter()
        .map(|(tag, count)| TagCount {
            tag: tag.clone(),
            count: *count,
        })
        .collect::<Vec<TagCount>>();

    tags.sort_by(|a, b| a.tag.cmp(&b.tag));

    let page = Page::new(
        Slug::new("tags"),
        Some("Tags"),
        Some("All Tags".to_string()),
    );

    let template = TagsListTemplate { page, tags };

    context
        .renderer
        .render_page(&template.page.slug, &template, None)
        .await
}

#[derive(Template)]
#[template(path = "posts/post_list/post_list_page.html")]
struct TagPostsTemplate {
    page: Page,
    posts: Vec<Post>,
}

async fn render_tag_page<'d>(
    context: &RendererContext,
    base_page: &Page,
    tag: &Tag,
    paginator_page: &PaginatorPage<'d, &Post>,
) -> Result<()> {
    let page = Page::from_page_and_pagination_page(base_page, paginator_page, "Posts");

    let template = TagPostsTemplate {
        page,
        posts: paginator_page.data.iter().cloned().cloned().collect(),
    };

    context
        .renderer
        .render_page(&template.page.slug, &template, None)
        .await
}
