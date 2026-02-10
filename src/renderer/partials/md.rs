use hypertext::{prelude::*, Raw};

use crate::renderer::formatters::format_markdown::FormatMarkdown;

pub enum MarkdownMediaOption {
    WithMedia,
    NoMedia,
}

pub fn md<'l>(md: &'l impl FormatMarkdown, media_option: MarkdownMediaOption) -> impl Renderable + 'l {
    let md = match media_option {
        MarkdownMediaOption::WithMedia => md.to_html(),
        MarkdownMediaOption::NoMedia => md.remove_media().to_html(),
    };

    maud! {
        (Raw::dangerously_create(&md))
    }
}
