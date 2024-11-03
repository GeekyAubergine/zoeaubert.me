use std::collections::HashMap;

use askama::Template;
use chrono::Utc;

use crate::{
    domain::{
        models::{omni_post::OmniPost, page::Page, slug::Slug},
        queries::omni_post_queries::{find_all_omni_posts, OmniPostFilterFlags},
        services::PageRenderingService,
        state::State,
    },
    infrastructure::utils::paginator::{paginate, PaginatorPage},
    prelude::*,
};

use crate::domain::models::media::Media;
use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

use crate::domain::models::tag::Tag;

const DEFAULT_PAGINATION_SIZE: usize = 25;

fn group_posts_by_tag(posts: Vec<OmniPost>) -> HashMap<Tag, Vec<OmniPost>> {
    let mut grouped = HashMap::new();

    for post in posts {
        for tag in post.tags() {
            grouped
                .entry(tag)
                .or_insert_with(Vec::new)
                .push(post.clone());
        }
    }

    grouped
}

pub struct TagCount {
    pub tag: Tag,
    pub count: usize,
}

pub async fn render_tags_pages(state: &impl State) -> Result<()> {
    let posts = find_all_omni_posts(state, OmniPostFilterFlags::filter_tags_page()).await?;

    let grouped = group_posts_by_tag(posts);

    let tag_counts = grouped
        .iter()
        .map(|(tag, posts)| (tag.clone(), posts.len()))
        .collect::<HashMap<Tag, usize>>();

    render_tags_list_page(state, &tag_counts).await?;

    for (tag, posts) in grouped {
        let paginated = paginate(&posts, DEFAULT_PAGINATION_SIZE);

        let base_page = Page::new(
            Slug::new(&format!("tags/{}", tag.slug())),
            Some(&format!("{} Posts", tag.title())),
            Some(&format!("#{} posts", tag.title())),
        );

        for page in paginated {
            render_tag_page(state, &base_page, &tag, &page).await?;
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

async fn render_tags_list_page(state: &impl State, tag_counts: &HashMap<Tag, usize>) -> Result<()> {
    let mut tags = tag_counts
        .into_iter()
        .map(|(tag, count)| TagCount {
            tag: tag.clone(),
            count: *count,
        })
        .collect::<Vec<TagCount>>();
    tags.sort_by(|a, b| a.tag.cmp(&b.tag));

    let page = Page::new(Slug::new("tags"), Some("Tags"), Some("All Tags"));

    let template = TagsListTemplate { page, tags };

    state
        .page_rendering_service()
        .add_page(state, template.page.slug.clone(), template)
        .await
}

#[derive(Template)]
#[template(path = "tags/tag.html")]
struct TagPostsTemplate {
    page: Page,
    posts: Vec<OmniPost>,
}

async fn render_tag_page<'d>(
    state: &impl State,
    base_page: &Page,
    tag: &Tag,
    paginator_page: &PaginatorPage<'d, OmniPost>,
) -> Result<()> {
    let page = Page::from_page_and_pagination_page(base_page, paginator_page, "Posts");

    let template = TagPostsTemplate {
        page,
        posts: paginator_page.data.to_vec(),
    };

    state
        .page_rendering_service()
        .add_page(state, template.page.slug.clone(), template)
        .await
}
