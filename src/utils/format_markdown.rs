use askama::{filters::safe, Html, MarkupDisplay, Text};
use askama_escape::Escaper;
use comrak::{
    markdown_to_html, ExtensionOptions, ListStyleType, Options, ParseOptions, RenderOptions,
};
use tracing::info;

lazy_static! {
    static ref OPTIONS: Options = {
        let mut options = Options::default();
        // Extension
        options.extension.strikethrough = true;
        options.extension.tagfilter = true;
        options.extension.table = true;
        options.extension.autolink = true;
        options.extension.tasklist = false;
        options.extension.superscript = false;
        options.extension.header_ids = None;
        options.extension.footnotes = false;
        options.extension.description_lists = false;
        options.extension.front_matter_delimiter = Some("---".to_owned());
        options.extension.multiline_block_quotes = true;
        options.extension.shortcodes = true;

        // Parse
        options.parse.smart = true;
        options.parse.default_info_string = None;
        options.parse.relaxed_tasklist_matching = true;
        options.parse.relaxed_autolinks = true;

        // Render
        options.render.escape = true;
        options.render.hardbreaks = true;
        options.render.github_pre_lang = true;
        options.render.full_info_string = true;
        options.render.width = 0;
        options.render.unsafe_ = true;
        options.render.list_style = ListStyleType::Dash;
        options.render.sourcepos = false;

        options
    };
}

fn string_to_html(s: &str) -> String {
    comrak::markdown_to_html(s, &OPTIONS)
}

pub trait FormatMarkdown {
    fn to_html(&self) -> String;
}

impl FormatMarkdown for String {
    fn to_html(&self) -> String {
        string_to_html(self).to_string()
    }
}

impl FormatMarkdown for &str {
    fn to_html(&self) -> String {
        string_to_html(self).to_string()
    }
}
