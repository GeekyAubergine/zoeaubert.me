use once_cell::sync::Lazy;
use regex::Regex;

use crate::domain::models::slug::Slug;

pub const MARKDOWN_LINK_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?i)\[([^\]]+)\]\(([^)]+)\)"#).unwrap());

fn replace_relative_links_wtih_absolute_links(markdown: &str) -> String {
    let mut out = markdown.to_string();

    for cap in MARKDOWN_LINK_REGEX.captures_iter(markdown) {
        let full_match = cap.get(0).unwrap().as_str();
        let alt_text = cap.get(1).unwrap().as_str();
        let url = cap.get(2).map_or("", |m| m.as_str());

        if !url.starts_with("http") {
            let slug = Slug::new(url);

            out = out.replace(full_match, &format!("[{}]({})", alt_text, slug.permalink_string()));
        }
    }

    out
}

pub trait FormatRelativeToAbsoluteUrls {
    fn format_relative_to_absolute_urls(&self) -> String;
}

impl FormatRelativeToAbsoluteUrls for str {
    fn format_relative_to_absolute_urls(&self) -> String {
        replace_relative_links_wtih_absolute_links(self)
    }
}

impl FormatRelativeToAbsoluteUrls for String {
    fn format_relative_to_absolute_urls(&self) -> String {
        self.as_str().format_relative_to_absolute_urls()
    }
}
