use crate::{build_data::BUILD_DATE, prelude::*};
use hypertext::prelude::*;
use maud::DOCTYPE;

use crate::domain::models::page::Page;

pub fn render_page_base(page: &Page, body: &str) -> Rendered<String> {
    maud! {
        !DOCTYPE
        html lang={ (page.language) } {
            head {
                meta charset="utf-8";
                link rel="preconnect" href="https://cdn.geekyaubergine.com";

                meta name="viewport" content="width=device-width, initial-scale=1";
                meta name="theme-color" media="(prefers-color-scheme: light" content="#FFFFFF";
                meta name="theme-color" media="(prefers-color-scheme: dark" content="#0C0C0E";

                link rel="icon" type="image/x-icon" href="/assets/img/icon.png";
                link rel="apple-touch-icon" sizes="256x256" href="/assets/img/icon.png";

                link rel="alternate" type="application/rss+xml" title="Zoe Aubert's RSS Feed" href="https://zoeaubert.me/feeds/blog-rss.xml";

                title {(page.title)}

                meta name="title" content=(page.title);
                meta name="og:title" content=(page.title);
                meta name="twitter:title" content=(page.title);

                meta name="description" content=(page.description);
                meta name="og:description" content=(page.description);
                meta name="twitter:description" content=(page.description);

                meta name="og:url" content=(page.permalink());
                meta name="twitter:url" content=(page.permalink());

                meta name="og:image" content=(page.image_url());
                meta name="twitter:image" content=(page.image_url());

                meta name="og:image:alt" content=(page.image_alt());
                meta name="twitter:image:alt" content=(page.image_alt());

                meta name="fediverse:creator" content="@geekyaubergine@social.lol";

                link rel="stylesheet" href={"/assets/css/styles-" BUILD_DATE ".css"};

                script src="https://cdn.usefathom.com/script.js" data-site="XPKVFMEO" defer {}
            }
            body {
                (body)
            }
        }
    }
    .render()
}
