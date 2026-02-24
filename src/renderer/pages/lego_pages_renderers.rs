use crate::domain::models::data::Data;
use crate::domain::models::lego::{LegoMinifig, LegoSet};
use crate::domain::models::page::Page;
use crate::domain::models::slug::Slug;
use crate::renderer::{RenderTask, RenderTasks};

use crate::prelude::*;
use crate::renderer::formatters::format_number::FormatNumber;
use crate::renderer::partials::page::{PageOptions, render_page};
use crate::services::page_renderer::PageRenderer;
use hypertext::prelude::*;

pub fn render_lego_pages<'d>(data: &'d Data, tasks: &mut RenderTasks<'d>) {
    tasks.add(RenderLegoPageTask {
        sets: data.lego.find_all_sets(),
        minifigs: data.lego.find_all_minifigs(),
    });
}

struct RenderLegoPageTask<'l> {
    sets: Vec<&'l LegoSet>,
    minifigs: Vec<&'l LegoMinifig>,
}

impl<'l> RenderTask for RenderLegoPageTask<'l> {
    fn render(self: Box<Self>, renderer: &PageRenderer) -> Result<()> {
        let sets = self.sets;
        let minifigs = self.minifigs;

        let content = maud! {
            section class="stats" {
                div class="stat" {
                    p class="value" { (sets.len().format(0, true)) }
                    p class="desc" { ("Sets") }
                }
                div class="stat" {
                    p class="value" { (sets.iter().map(|set| set.pieces).sum::<u32>().format(0, true)) }
                    p class="desc" { ("Pieces") }
                }
                div class="stat" {
                    p class="value" { (minifigs.len().format(0, true)) }
                    p class="desc" { ("Minfigs") }
                }
            }
            section {
                h2 { "Sets" }
                ul {
                    @for set in &sets {
                        li {
                            a href=(set.link.as_str()) {
                                (set.image.render_large())
                                p { (set.name) }
                                p { (set.id) }
                                @if set.quantity > 1 {
                                    p { (format!("{} pieces x {}", set.pieces.format(0, true), set.quantity))}
                                }
                                @else {
                                    p { (format!("{} pieces", set.pieces.format(0, true)))}
                                }
                            }
                        }
                    }
                }
            }
            section {
                h2 { "Minifigs"}
                ul {
                    @for minifig in &minifigs {
                        li {
                            a href=(&minifig.link()) {
                                (minifig.image.render_large())
                                p { (minifig.name) }
                                @if minifig.total_owned > 1 {
                                    p { (format!("x {}", minifig.total_owned))}
                                }
                            }
                        }
                    }
                }
            }
        };

        let options = PageOptions::new().with_main_class("lego-list-page");

        let page = Page::new(Slug::new("/interests/lego"), Some("Lego".to_string()), None);

        let slug = page.slug.clone();

        let rendered = render_page(&page, &options, &content, maud! {});

        renderer.render_page(&slug, &rendered, None)
    }
}
