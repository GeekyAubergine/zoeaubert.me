use chrono::{DateTime, Utc};
use hypertext::Renderable;
use hypertext::prelude::*;

use crate::renderer::formatters::format_date::FormatDate;

pub fn render_date<'l>(date: &'l DateTime<Utc>) -> impl Renderable + 'l {
    maud! {
        time class="date" datetime=(date.datetime()) {
            (date.month_as_word())
        }
    }
}
