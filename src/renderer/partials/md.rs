use hypertext::{prelude::*, Raw};

use crate::renderer::formatters::format_markdown::FormatMarkdown;

pub fn md<'l>(md: &'l impl FormatMarkdown) -> impl Renderable + 'l {
    maud! {
        (Raw(md.to_html()))
    }
}
