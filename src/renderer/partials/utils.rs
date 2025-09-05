use crate::{domain::models::slug::Link, prelude::*};
use hypertext::prelude::*;
use url::Url;

// TODO, option to not open in new tab?

pub fn link<'l>(link: &'l Link<'l>, children: &'l dyn Renderable) -> impl Renderable + 'l {
    maud! {
         @match link {
             Link::External(url) => {
                a
                    class="items-center border-hover-accent"
                    href={ (url.as_str()) }
                    target="_blank"
                    rel="noopener noreferrer" {
                        { (children) }
                    }
             },
             Link::Internal(url) => {
                a
                    class="items-center border-hover-accent"
                    href={ (url) }
                    rel="noopener noreferrer" {
                        { (children) }
                    }
            },
        }
    }
}
