use crate::{error::Error, prelude::*};

use askama::{filters::safe, Html, MarkupDisplay, Text};
use askama_escape::Escaper;
use comrak::{
    markdown_to_html, ExtensionOptions, ListStyleType, Options, ParseOptions, RenderOptions,
};
use regex::Regex;
use syntect::{easy::HighlightLines, parsing::SyntaxSet};
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

    static ref SYNTAX_SET: SyntaxSet = {
        let set = SyntaxSet::load_defaults_newlines();

        set
    };


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
}

fn correct_codeblock_language_name(s: &str) -> &str {
    match s {
        "ts" => "tsx",
        "js" => "javascript",
        "sh" => "bash",
        "yml" => "yaml",
        "toml" => "toml",
        "json" => "json",
        "html" => "html",
        "css" => "css",
        "scss" => "scss",
        "md" => "markdown",
        "rs" => "rust",
        "py" => "python",
        "rb" => "ruby",
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
        .ok_or(Error::CouldNotFindLangaugeForCodeBlock())?
        .as_str();
    let code = caps
        .name("body")
        .ok_or(Error::CouldNotFindBodyForCodeBlock())?
        .as_str();

    let lang = correct_codeblock_language_name(original_lang);

    println!("lang: {}", lang);

    let syntax = SYNTAX_SET
        .find_syntax_by_extension(lang)
        .unwrap_or_else(|| {
            info!("Syntax not found for {}", lang);
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
                        info!("Error highlighting line: {}", e);
                    }
                }
            }
            Err(e) => {
                info!("Error highlighting line: {}", e);
            }
        }
    }

    let highlighted = lines.join("\n");

    Ok(format!(
        "<pre lang=\"{}\"><code>{}</code></pre>",
        original_lang, highlighted
    ))
}

fn highligh_codeblocks(markdown: &str) -> String {
    let re = Regex::new(
        r#"<pre lang="(?P<lang>\S+?)">\s*?<code>(?P<body>(?s:(.*?)))(</code>\s*?</pre>)"#,
    )
    .unwrap();
    re.replace_all(
        markdown,
        |caps: &regex::Captures| match highlight_code_block_capture(caps) {
            Ok(highlighted) => highlighted,
            Err(e) => {
                info!("Error highlighting code block: {}", e);
                markdown.to_string()
            }
        },
    )
    .into_owned()
}

fn replace_codeblock_language_old_style_with_new(s: &str) -> String {
    let re = Regex::new("<pre lang=\"(.+)\">(.*?)<code>").unwrap();
    re.replace_all(
        s,
        "<pre class=\"language-$1\">$2<code class=\"language-$1\">",
    )
    .to_string()
}

fn string_to_html(s: &str) -> String {
    let html = comrak::markdown_to_html(s, &OPTIONS);

    let html = highligh_codeblocks(&html);

    html
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
