use askama::Template;

use crate::domain::models::image::Image;
use crate::domain::models::lego::{LegoMinifig, LegoSet};
use crate::domain::models::media::{Media, MediaOrientation};
use crate::domain::models::post::PostFilter;
use crate::domain::models::slug::Slug;
use crate::domain::{models::page::Page};

use crate::renderers::RendererContext;
use crate::prelude::*;

use crate::renderers::formatters::format_date::FormatDate;
use crate::renderers::formatters::format_markdown::FormatMarkdown;
use crate::renderers::formatters::format_number::FormatNumber;

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

pub async fn render_interests_list_page(context: &RendererContext) -> Result<()> {
    let page = Page::new(
        Slug::new("/interests"),
        Some("Interests"),
        Some("My Interests".to_string()),
    );

    let mut interests = vec![];

    if let Some(most_recent_game) = context.data.games.find_by_most_recently_played().first() {
        interests.push(Interest {
            name: "Games".to_string(),
            slug: page.slug.append("games"),
            image: most_recent_game.image().clone(),
        });
    }

    if let Some(biggest_lego_set) = context.data.lego.find_all_sets().first() {
        interests.push(Interest {
            name: "Lego".to_string(),
            slug: page.slug.append("lego"),
            image: biggest_lego_set.image.clone(),
        });
    }

    if let Some(latest_album) = context.data.albums.find_all_by_date().first().cloned() {
        let cover_images = latest_album.cover_images();

        let landscape_images = cover_images
            .iter()
            .filter(|c| c.orientation() == MediaOrientation::Landscape)
            .cloned()
            .collect::<Vec<_>>();

        if let Some(first) = landscape_images.first() {
            interests.push(Interest {
                name: "Photography".to_string(),
                slug: Slug::new("albums"),
                image: first.clone().clone(),
            });
        } else if let Some(first) = cover_images.first() {
            interests.push(Interest {
                name: "Photography".to_string(),
                slug: page.slug.append("photography"),
                image: first.clone().clone(),
            });
        }
    }

    if let Some(most_recent_movie) = context
        .data
        .posts
        .find_all_by_filter(PostFilter::MOVIE_REVIEW)
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

    if let Some(most_recent_tv_show) = context
        .data
        .posts
        .find_all_by_filter(PostFilter::TV_SHOW_REVIEW)
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

    if let Some(most_recent_book) = context
        .data
        .posts
        .find_all_by_filter(PostFilter::BOOK_REVIEW)
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

    context
        .renderer
        .render_page(&template.page.slug, &template, None)
        .await
}
