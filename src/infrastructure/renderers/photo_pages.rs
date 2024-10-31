use askama::Template;

use crate::{
    domain::{
        models::{
            image::Image,
            omni_post::OmniPost,
            page::{Page, PagePagination},
            slug::Slug,
        },
        queries::omni_post_queries::{find_all_omni_posts, OmniPostFilterFlags},
        state::State,
    },
    infrastructure::utils::paginator::{paginate, PaginatorPage},
};

use crate::prelude::*;

use super::render_page_with_template;

use crate::domain::models::media::Media;
use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

const DEFAULT_PAGINATION_SIZE: usize = 48;

#[derive(Template)]
#[template(path = "photos.html")]
pub struct PhotosPage<'t> {
    page: &'t Page<'t>,
    photos: &'t [Image],
}

pub async fn render_photos_page<'d>(state: &impl State) -> Result<()> {
    let omni_posts =
        find_all_omni_posts(state, OmniPostFilterFlags::filter_photos_page()).await?;

    let photos = omni_posts
        .iter()
        .flat_map(|post| post.optimised_media())
        .filter_map(|media| match media {
            Media::Image(image) => Some(image),
            _ => None,
        })
        .collect::<Vec<_>>();

    let paginated = paginate(&photos, DEFAULT_PAGINATION_SIZE);

    for paginator_page in paginated {
        let page = Page::new(Slug::new("photos"), Some("Photos"), Some("All my photos"))
            .with_pagination_from_paginator(&paginator_page, "Posts");

        let template = PhotosPage {
            page: &page,
            photos: paginator_page.data,
        };

        render_page_with_template(state, &page, template).await?;
    }

    Ok(())
}
