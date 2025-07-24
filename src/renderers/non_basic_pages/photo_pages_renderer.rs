use askama::Template;

use crate::{
    domain::models::{
        image::LegacyImage,
        page::{Page, PagePagination},
        post::Post,
        post::PostFilter,
        slug::Slug,
    },
    renderers::RendererContext,
    utils::paginator::{paginate, PaginatorPage},
};

use crate::prelude::*;

use crate::domain::models::media::Media;
use crate::renderers::formatters::format_date::FormatDate;
use crate::renderers::formatters::format_markdown::FormatMarkdown;
use crate::renderers::formatters::format_number::FormatNumber;

const DEFAULT_PAGINATION_SIZE: usize = 40;

#[derive(Template)]
#[template(path = "photos.html")]
pub struct PhotosPage {
    page: Page,
    photos: Vec<LegacyImage>,
}

pub async fn render_photos_page<'d>(context: &RendererContext) -> Result<()> {
    let omni_posts = context
        .data
        .posts
        .find_all_by_filter(PostFilter::filter_photos_page());

    let photos = omni_posts
        .iter()
        .flat_map(|post| post.optimised_media())
        .filter_map(|media| match media {
            Media::Image(image) => Some(image),
            _ => None,
        })
        .collect::<Vec<_>>();

    let paginated = paginate(&photos, DEFAULT_PAGINATION_SIZE);

    let page = Page::new(
        Slug::new("photos"),
        Some("Photos"),
        Some("All my photos".to_string()),
    );

    for paginator_page in paginated {
        let mut page = Page::from_page_and_pagination_page(&page, &paginator_page, "Photos");

        if let Some(first_image) = paginator_page.data.first() {
            page = page.with_image(first_image.clone().into());
        }

        let template = PhotosPage {
            page,
            photos: paginator_page.data.to_vec(),
        };

        context
            .renderer
            .render_page(&template.page.slug, &template, None)
            .await?;
    }

    Ok(())
}
