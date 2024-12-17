use askama::Template;

use crate::domain::models::image::{Image, ImageOrientation};
use crate::domain::models::lego::{LegoMinifig, LegoSet};
use crate::domain::models::media::Media;
use crate::domain::models::slug::Slug;
use crate::domain::queries::games_queries::find_all_games_by_most_recently_played;
use crate::domain::queries::omni_post_queries::{find_all_omni_posts, OmniPostFilterFlags};
use crate::domain::repositories::{AlbumsRepo, LegoRepo};
use crate::domain::services::PageRenderingService;
use crate::domain::{models::page::Page, state::State};

use crate::prelude::*;

use crate::infrastructure::renderers::formatters::format_date::FormatDate;
use crate::infrastructure::renderers::formatters::format_markdown::FormatMarkdown;
use crate::infrastructure::renderers::formatters::format_number::FormatNumber;

struct Interest {
    name: String,
    slug: Slug,
    image: Image,
}

#[derive(Template)]
#[template(path = "interests/interests_list.html")]
pub struct InterestListTemplate {
    page: Page,
    interests: Vec<Interest>,
}

pub async fn render_interests_list_page(state: &impl State) -> Result<()> {
    let page = Page::new(
        Slug::new("/interests"),
        Some("Interests"),
        Some("My Interests"),
    );

    let mut interests = vec![];

    if let Some(most_recent_game) = find_all_games_by_most_recently_played(state)
        .await?
        .first()
        .cloned()
    {
        interests.push(Interest {
            name: "Games".to_string(),
            slug: page.slug.append("games"),
            image: most_recent_game.image().clone(),
        });
    }

    if let Some(biggest_lego_set) = state.lego_repo().find_all_sets().await?.first().cloned() {
        interests.push(Interest {
            name: "Lego".to_string(),
            slug: page.slug.append("lego"),
            image: biggest_lego_set.image.clone(),
        });
    }

    if let Some(latest_album) = state
        .albums_repo()
        .find_all_by_date()
        .await?
        .first()
        .cloned()
    {
        let cover_images = latest_album.cover_images();

        let landscape_images = cover_images
            .iter()
            .filter(|c| c.orientation() == ImageOrientation::Landscape)
            .cloned()
            .collect::<Vec<_>>();

        if let Some(first) = landscape_images.first() {
            interests.push(Interest {
                name: "Photography".to_string(),
                slug: Slug::new("albums"),
                image: first.clone(),
            });
        } else if let Some(first) = cover_images.first() {
            interests.push(Interest {
                name: "Photography".to_string(),
                slug: page.slug.append("photography"),
                image: first.clone(),
            });
        }
    }

    if let Some(most_recent_movie) = find_all_omni_posts(state, OmniPostFilterFlags::MOVIE_REVIEW)
        .await?
        .first()
        .cloned()
    {
        if let Some(image) = most_recent_movie.side_image() {
            interests.push(Interest {
                name: "Movies".to_string(),
                slug: page.slug.append("movies"),
                image: image.clone(),
            });
        }
    }

    if let Some(most_recent_tv_show) =
        find_all_omni_posts(state, OmniPostFilterFlags::TV_SHOW_REVIEW)
            .await?
            .first()
            .cloned()
    {
        if let Some(image) = most_recent_tv_show.side_image() {
            interests.push(Interest {
                name: "TV".to_string(),
                slug: page.slug.append("tv"),
                image: image.clone(),
            });
        }
    }

    if let Some(most_recent_book) = find_all_omni_posts(state, OmniPostFilterFlags::BOOK_REVIEW)
        .await?
        .first()
        .cloned()
    {
        if let Some(image) = most_recent_book.side_image() {
            interests.push(Interest {
                name: "Books".to_string(),
                slug: page.slug.append("books"),
                image: image.clone(),
            });
        }
    }

    let template = InterestListTemplate { page, interests };

    state
        .page_rendering_service()
        .add_page(state, template.page.slug.clone(), template, None)
        .await
}
