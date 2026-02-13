use crate::{error::MarkdownError, prelude::*};

use comrak::{Options, options::ListStyleType};
use once_cell::sync::Lazy;
use regex::Regex;
use syntect::{easy::HighlightLines, parsing::SyntaxSet};
use tracing::{error, warn};

static MEDIA_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"\!\[.*?\]\(.*?\)"#).unwrap());
static MEDIA_HTML_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"<img[^>]*src="(?P<src>[^"]+)"[^>]*alt="(?P<alt>[^"]+)"[^>]*>"#).unwrap()
});

static OPTIONS: Lazy<Options> = Lazy::new(|| {
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
    options.extension.front_matter_delimiter = None;
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
    options.render.r#unsafe = true;
    options.render.list_style = ListStyleType::Dash;
    options.render.sourcepos = false;

    options
});

static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(SyntaxSet::load_defaults_newlines);

// static ref SYNTAX_HIGHLIGHTER: Highlighter = {
//     let mut highlighter = Highlighter::new();

//     let typescript_language = tree_sitter_typescript::language_typescript();

//     let typescript_config = HighlightConfiguration::new(
//         typescript_language,
//         "ts",
//         tree_sitter_typescript::HIGHLIGHT_QUERY,
//         "",
//         tree_sitter_typescript::LOCALS_QUERY,
//     ).unwrap();

//     highlighter
// };

fn correct_codeblock_language_name_to_extension(s: &str) -> &str {
    match s {
        "typescript" => "js",
        "tsx" => "js",
        "ts" => "js",
        "javascript" => "js",
        "bash" => "sh",
        "yaml" => "yml",
        "toml" => "toml",
        "json" => "json",
        "html" => "html",
        "css" => "css",
        "scss" => "scss",
        "markdown" => "md",
        "rust" => "rs",
        "python" => "py",
        "ruby" => "rb",
        "java" => "java",
        "go" => "go",
        "php" => "php",
        "sql" => "sql",
        "xml" => "xml",
        "dockerfile" => "dockerfile",
        "plaintext" => "plaintext",
        _ => s,
    }
}

fn highlight_code_block_capture(caps: &regex::Captures) -> Result<String> {
    let original_lang = caps
        .name("lang")
        .ok_or(MarkdownError::could_not_find_language_for_code_block())?
        .as_str();
    let code = caps
        .name("body")
        .ok_or(MarkdownError::could_not_find_body_for_code_block())?
        .as_str();

    let lang = correct_codeblock_language_name_to_extension(original_lang);

    let syntax = SYNTAX_SET
        .find_syntax_by_extension(lang)
        .unwrap_or_else(|| {
            warn!("Syntax not found for {}", lang);
            SYNTAX_SET.find_syntax_plain_text()
        });

    let theme_set = syntect::highlighting::ThemeSet::load_defaults();

    let theme = &theme_set.themes["base16-ocean.dark"];

    let mut highlighter = HighlightLines::new(syntax, theme);

    let mut lines = vec![];

    for line in code.lines() {
        match highlighter.highlight_line(line, &SYNTAX_SET) {
            Ok(ranges) => {
                match syntect::html::styled_line_to_highlighted_html(
                    &ranges,
                    syntect::html::IncludeBackground::No,
                ) {
                    Ok(html) => {
                        lines.push(html);
                    }
                    Err(e) => {
                        error!("Error highlighting line: {}", e);
                    }
                }
            }
            Err(e) => {
                error!("Error highlighting line: {}", e);
            }
        }
    }

    let highlighted = lines.join("\n");

    Ok(format!(
        "<pre lang=\"{}\">{}</pre>",
        original_lang, highlighted
    ))
}

fn highligh_codeblocks(markdown: &str) -> String {
    let markdown = html_escape::decode_html_entities(markdown).to_string();
    let re = Regex::new(
        r#"<pre lang="(?P<lang>\S+?)">\s*?<code>(?P<body>(?s:(.*?)))(</code>\s*?</pre>)"#,
    )
    .unwrap();
    re.replace_all(
        &markdown,
        |caps: &regex::Captures| match highlight_code_block_capture(caps) {
            Ok(highlighted) => highlighted,
            Err(e) => {
                error!("Error highlighting code block: {}", e);
                markdown.to_string()
            }
        },
    )
    .into_owned()
}

fn markdown_to_html(s: &str) -> String {
    let html = comrak::markdown_to_html(s, &OPTIONS);

    highligh_codeblocks(&html)
}

fn remove_media_from_markdown(markdown: &str) -> String {
    let s = MEDIA_REGEX.replace_all(markdown, "");
    let s = MEDIA_HTML_REGEX.replace_all(&s, "");
    s.to_string()
}

pub trait FormatMarkdown {
    fn to_html(&self) -> String;

    fn remove_media(&self) -> String;
}

impl FormatMarkdown for String {
    fn to_html(&self) -> String {
        markdown_to_html(self).to_string()
    }

    fn remove_media(&self) -> String {
        remove_media_from_markdown(self)
    }
}

impl FormatMarkdown for &str {
    fn to_html(&self) -> String {
        markdown_to_html(self).to_string()
    }

    fn remove_media(&self) -> String {
        remove_media_from_markdown(self)
    }
}
