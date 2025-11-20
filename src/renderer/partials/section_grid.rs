use chrono::{DateTime, Utc};
use hypertext::prelude::*;

use crate::{domain::models::image::Image, renderer::partials::date::render_date};

pub enum SectionGridItemHeader<'l> {
    Title(&'l str),
    TitleAndDate {
        title: &'l str,
        date: &'l DateTime<Utc>
    }
}

pub enum SectionGridItemContent<'l> {
    Image(&'l Image),
    Text(&'l str),
}

pub struct SectionGridItem<'l> {
    header: SectionGridItemHeader<'l>,
    content: SectionGridItemContent<'l>,
    link: &'l str,
}

pub fn render_section_grid<'l>(
    title: &'l str,
    items: &'l[&'l SectionGridItem<'l>],
    more_text: &'l str,
    more_link: &'l str,
) -> impl Renderable + 'l {
    maud! {
        section {
            h2 { (title) }
            ul {
                @for item in items {
                    li {
                        @match item.header {
                            SectionGridItemHeader::Title(title) => h3 {
                                a href=(&item.link) {
                                    h3 class="title" { (title) }
                                }
                            }
                            SectionGridItemHeader::TitleAndDate { title, date } => {
                                a href=(&item.link) {
                                    h3 class="title" { (title) }
                                }
                                (render_date(date))
                            },
                        }
                    }
                }
            }
        }
    }
}
