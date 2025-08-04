use crate::prelude::*;
use hypertext::prelude::*;
use url::Url;

pub struct PageParts {
    pub header: Rendered<String>,
    pub content: Rendered<String>,
}

pub fn link<'a>(url: &Url, children: Rendered<String>) -> Rendered<String> {
    maud! {
        a
            class="items-center border-hover-accent"
            href={ (url.to_string()) }
            target="_blank"
            rel="noopener noreferrer" {
                { (children.as_str()) }
            }
    }.render()
}
