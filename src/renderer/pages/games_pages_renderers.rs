use crate::domain::models::games::Game;
use crate::domain::models::page::Page;
use crate::domain::models::slug::Slug;
use crate::domain::models::timeline_event::TimelineEvent;
use crate::renderer::RendererContext;

use crate::prelude::*;
use crate::renderer::formatters::format_number::FormatNumber;
use crate::renderer::partials::date::render_date;
use crate::renderer::partials::page::{PageOptions, render_page};
use crate::renderer::partials::timline_events_list::render_timline_events_list;
use crate::utils::paginator::paginate;
use hypertext::prelude::*;

pub fn render_games_pages(context: &RendererContext) -> Result<()> {
    render_games_list_page(context)?;
    render_games_most_achieved_list_page(context)?;
    render_activity_pages(context)?;

    for game in context.data.games.find_by_most_recently_played() {
        render_game_page(context, &game)?;
    }

    Ok(())
}

fn render_games_list_page(context: &RendererContext) -> Result<()> {
    let recent_games = context
        .data
        .games
        .find_by_most_recently_played()
        .into_iter()
        .take(6)
        .collect::<Vec<&Game>>();

    let most_played = context.data.games.find_by_most_most_played();

    let total_play = most_played.iter().map(|g| g.playtime_hours()).sum::<f32>();

    let achievement_rate = most_played
        .iter()
        .filter_map(|g| {
            if g.playtime_hours() > 0.5 {
                Some(g.achievement_unlocked_percentage())
            } else {
                None
            }
        })
        .sum::<f32>()
        / most_played.len().max(1) as f32;

    let content = maud! {
        section class="stats" {
            div class="stat" {
                p class="value" {
                    (total_play.format(1, true))
                    span class="unit" { ("h") }
                }
                p class="desc" { ("Playtime") }
            }
            div class="stat" {
                p class="value" { (most_played.len().format(0, true)) }
                p class="desc" { ("Games") }
            }
            div class="stat" {
                p class="value" {
                    ((achievement_rate * 100.0).format(1, true))
                    span class="unit" { ("%") }
                }
                p class="desc" { ("Completion") }
            }
        }
        section {
            h2 { "Recently Played" }
            ul {
                @for game in &recent_games {
                    li {
                        a href=(game.slug().relative_string()) {
                            (game.image().render_small())
                            p { (game.name()) }
                            p { (render_date(game.last_played())) }
                        }
                    }
                }
            }
        }
        section {
            h2 { "Most Played"}
            ul {
                @for game in &most_played {
                    li {
                        a href=(game.slug().relative_string()) {
                            (game.image().render_small())
                            p { (game.name()) }
                            p { (format!("{:.1}h", game.playtime_hours())) }
                        }
                    }
                }
            }
        }
        section {
            a href="/interests/games/by-achievements" { ("Sort by Achievments") }
        }
    };

    let options = PageOptions::new().with_main_class("games-list-page");

    let page = Page::new(
        Slug::new("/interests/games"),
        Some("Games".to_string()),
        None,
    );

    let slug = page.slug.clone();

    let renderer = render_page(&page, &options, &content, maud! {});

    context.renderer.render_page(&slug, &renderer, None)
}

fn render_games_most_achieved_list_page(context: &RendererContext) -> Result<()> {
    let games = context
        .data
        .games
        .find_by_most_highest_achievement_unlocked_percentage();

    let content = maud! {
        section {
            h2 { "Highest Achievments"}
            ul {
                @for game in &games {
                    li {
                        a href=(game.slug().relative_string()) {
                            (game.image().render_small())
                            p { (game.name()) }
                            p { (format!("{} / {}", game.unlocked_achievement_count(), game.achievments_count())) }
                            p { (format!("{:.1}h", game.playtime_hours())) }
                        }
                    }
                }
            }
        }
    };

    let options = PageOptions::new().with_main_class("games-list-page");

    let page = Page::new(
        Slug::new("/interests/games/by-achievements"),
        Some("Games by Achievments".to_string()),
        None,
    );

    let slug = page.slug.clone();

    let renderer = render_page(&page, &options, &content, maud! {});

    context.renderer.render_page(&slug, &renderer, None)
}

fn render_game_page(context: &RendererContext, game: &Game) -> Result<()> {
    let content = maud! {
        div class="cover" {
            (game.image().render_original())
            @match game {
                Game::Steam(game) => {
                    a href=(game.game.link_url) rel="noopener noreferrer" {
                        ("Steam Store Page")
                    }
                }
            }
        }
        section class="stats" {
            div class="stat" {
                p class="value" {
                    (format!("{:.1}", game.playtime_hours()))
                    span class="unit" { ("h") }
                }
                p class="desc" { ("Playtime") }
            }
            div class="stat" {
                p class="value" {
                    (format!("{} / {}", game.unlocked_achievement_count(), game.achievments_count()))
                }
                p class="desc" { ("Achievements") }
            }
        }
        @match game {
            Game::Steam(game) => {
                section {
                    h2 { ("Achievements") }
                    ul {
                        @for achievement in game.find_all_unlocked_by_unlocked_date() {
                            li {
                                (achievement.image.render_small())
                                div class="name-and-description" {
                                h3 { (achievement.display_name) }
                                p { (achievement.description) }
                                }
                                (render_date(&achievement.unlocked_date))
                            }
                        }
                    }
                }
            }
        }
    };

    let options = PageOptions::new().with_main_class("game-page");

    let page = Page::new(game.slug(), Some(game.name().to_string()), None);

    let slug = page.slug.clone();

    let renderer = render_page(&page, &options, &content, maud! {});

    context.renderer.render_page(&slug, &renderer, None)
}

const PAGINATION_SIZE: usize = 25;

pub fn render_activity_pages(context: &RendererContext) -> Result<()> {
    let posts = context
        .data
        .timeline_events
        .all_by_date()
        .iter()
        .filter(|event| match event {
            TimelineEvent::GameAchievementUnlock(_) => true,
            _ => false,
        })
        .collect::<Vec<&TimelineEvent>>();

    let paginated = paginate(&posts, PAGINATION_SIZE);

    let page = Page::new(
        Slug::new("/interests/games/activity/"),
        Some("Gaming Activity".to_string()),
        None,
    );
    for paginator_page in paginated {
        let page = Page::from_page_and_pagination_page(&page, &paginator_page);

        let slug = page.slug.clone();

        let content = render_timline_events_list(paginator_page.data);

        let options = PageOptions::new().with_main_class("gaming-activity-page");

        let renderer = render_page(&page, &options, &content, maud! {});

        context.renderer.render_page(&slug, &renderer, None)?;
    }

    Ok(())
}
