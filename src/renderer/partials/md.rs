use hypertext::{prelude::*, Raw};

pub fn md<'l>(md: &'l str) -> impl Renderable + 'l {
    maud! {
        (Raw(&md))
    }
}
