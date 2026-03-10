use crate::domain::models::data::Data;
use crate::domain::models::games::Game;
use crate::domain::models::page::Page;
use crate::domain::models::slug::Slug;
use crate::domain::models::timeline_event::TimelineEvent;
use crate::renderer::{RenderTask, RenderTasks};

use crate::prelude::*;
use crate::renderer::formatters::format_number::FormatNumber;
use crate::renderer::partials::date::render_date;
use crate::renderer::partials::page::{PageOptions, render_page};
use crate::renderer::partials::timeline_events_list::RenderTimelineEventsListTask;
use crate::services::page_renderer::PageRenderer;
use crate::utils::paginator::Paginator;
use hypertext::prelude::*;

pub fn render_games_pages<'d>(data: &'d Data, tasks: &mut RenderTasks<'d>) {
    tasks.add(RenderGamesListPageTask {
        recently_played: data
            .games
            .find_by_most_recently_played()
            .into_iter()
            .take(6)
            .collect(),
        most_played: data
            .games
            .find_by_most_most_played()
            .into_iter()
            .filter(|g| g.playtime_hours() >= 0.1)
            .collect(),
        most_acheived: data
            .games
            .find_by_most_highest_achievement_unlocked_percentage()
            .into_iter()
            .filter(|g| g.unlocked_achievement_count() > 0)
            .collect(),
    });

    data.games
        .find_by_most_recently_played()
        .iter()
        .for_each(|game| tasks.add(RenderGamePageTask { game }));

    render_activity_pages(data, tasks);
}

struct RenderGamesListPageTask<'g> {
    recently_played: Vec<&'g Game>,
    most_played: Vec<&'g Game>,
    most_acheived: Vec<&'g Game>,
}

impl<'g> RenderTask for RenderGamesListPageTask<'g> {
    fn render(self: Box<Self>, renderer: &PageRenderer) -> Result<()> {
        let recently_played = self.recently_played;
        let most_played = self.most_played;
        let most_achieved = self.most_acheived;

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
                    @for game in &recently_played {
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
                h2 { "Highest Achievments"}
                ul {
                    @for game in &most_achieved {
                        li {
                            a href=(game.slug().relative_string()) {
                                (game.image().render_small())
                                p { (game.name()) }
                                p { (format!("{} / {}", game.unlocked_achievement_count(), game.achievments_count())) }
                            }
                        }
                    }
                }
            }
        };

        let options = PageOptions::new().with_main_class("games-list-page");

        let page = Page::new(
            Slug::new("/interests/games"),
            Some("Games".to_string()),
            None,
        );

        let slug = page.slug.clone();

        let rendered = render_page(&page, &options, &content, maud! {});

        renderer.render_page(&slug, &rendered, None)
    }
}

struct RenderGamePageTask<'g> {
    game: &'g Game,
}

impl<'g> RenderTask for RenderGamePageTask<'g> {
    fn render(self: Box<Self>, renderer: &PageRenderer) -> Result<()> {
        let game = self.game;

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

        let rendered = render_page(&page, &options, &content, maud! {});

        renderer.render_page(&slug, &rendered, None)
    }
}

const PAGINATION_SIZE: usize = 25;

pub fn render_activity_pages<'d>(data: &'d Data, tasks: &mut RenderTasks<'d>) {
    let page = Page::new(
        Slug::new("/interests/games/activity/"),
        Some("Gaming Activity".to_string()),
        None,
    );

    let options = PageOptions::new().with_main_class("gaming-activity-page");

    data.timeline_events
        .all_by_date()
        .iter()
        .filter(|event| matches!(event, TimelineEvent::GameAchievementUnlock(_)))
        .paginate(PAGINATION_SIZE)
        .for_each(|paginator_page| {
            tasks.add(RenderTimelineEventsListTask::new(
                paginator_page,
                page.clone(),
                options.clone(),
            ))
        });
}
