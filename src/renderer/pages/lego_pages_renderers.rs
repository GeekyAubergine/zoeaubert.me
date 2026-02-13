use crate::domain::models::page::Page;
use crate::domain::models::slug::Slug;
use crate::renderer::RendererContext;

use crate::prelude::*;
use crate::renderer::formatters::format_number::FormatNumber;
use crate::renderer::partials::page::{PageOptions, render_page};
use hypertext::prelude::*;

pub fn render_lego_pages(context: &RendererContext) -> Result<()> {
    render_lego_list_page(context)?;
    Ok(())
}

fn render_lego_list_page(context: &RendererContext) -> Result<()> {
    let sets = context.data.lego.find_all_sets();
    let minifigs = context.data.lego.find_all_minifigs();

    let content = maud! {
        section class="stats" {
            div class="stat" {
                p class="value" { (context.data.lego.find_total_sets().format(0, true)) }
                p class="desc" { ("Sets") }
            }
            div class="stat" {
                p class="value" { (context.data.lego.find_total_pieces().format(0, true)) }
                p class="desc" { ("Pieces") }
            }
            div class="stat" {
                p class="value" { (context.data.lego.find_total_minifigs().format(0, true)) }
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

    let renderer = render_page(&page, &options, &content, maud! {});

    context.renderer.render_page(&slug, &renderer, None)
}
