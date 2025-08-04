use crate::prelude::*;
use hypertext::prelude::*;
use maud::DOCTYPE;

use crate::domain::models::page::Page;

pub fn render_page_base(page: &Page, head: Option<&str>, body: Option<&str>) -> Rendered<String> {
    maud! {
        (DOCTYPE.into_string())
        html lang={ (page.language) } {
            head {
                link
                    rel="preconnect"
                    href="https://cdn.geekyaubergine.com"
                ;
            }
        }
    }
    .render()
}
