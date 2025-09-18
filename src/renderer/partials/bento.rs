use hypertext::prelude::*;

pub struct BentoBoxOptions<'l> {
    pub title: &'l str,
    pub width: u8,
    pub height: Option<u8>,
    pub row: u8,
    pub class_name: &'l str,
}

#[component]
pub fn bento_box_component<'l>(
    options: &'l BentoBoxOptions<'l>,
    content: &'l dyn Renderable,
) -> impl Renderable + 'l {
    maud! {
        div class=(options.class_name) {
            h2 { (options.title) }

            (content)
        }
    }
}
