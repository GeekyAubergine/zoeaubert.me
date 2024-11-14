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
        services::PageRenderingService,
        state::State,
    },
    infrastructure::utils::paginator::{paginate, PaginatorPage},
};

use crate::prelude::*;

use crate::domain::models::media::Media;
use crate::infrastructure::renderers::formatters_renderer::format_date::FormatDate;
use crate::infrastructure::renderers::formatters_renderer::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters_renderer::format_number::FormatNumber;

const DEFAULT_PAGINATION_SIZE: usize = 48;

#[derive(Template)]
#[template(path = "photos.html")]
pub struct PhotosPage {
    page: Page,
    photos: Vec<Image>,
}

pub async fn render_photos_page<'d>(state: &impl State) -> Result<()> {
    let omni_posts = find_all_omni_posts(state, OmniPostFilterFlags::filter_photos_page()).await?;

    let photos = omni_posts
        .iter()
        .flat_map(|post| post.optimised_media())
        .filter_map(|media| match media {
            Media::Image(image) => Some(image),
            _ => None,
        })
        .collect::<Vec<_>>();

    let paginated = paginate(&photos, DEFAULT_PAGINATION_SIZE);

    let page = Page::new(Slug::new("photos"), Some("Photos"), Some("All my photos"));

    for paginator_page in paginated {
        let mut page = Page::from_page_and_pagination_page(&page, &paginator_page, "Posts");

        if let Some(first_image) = paginator_page.data.first() {
            page = page.with_image(first_image.clone().into());
        }

        let template = PhotosPage {
            page,
            photos: paginator_page.data.to_vec(),
        };

        state
            .page_rendering_service()
            .add_page(state, template.page.slug.clone(), template)
            .await?;
    }

    Ok(())
}
