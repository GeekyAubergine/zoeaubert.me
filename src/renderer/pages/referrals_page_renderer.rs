use crate::domain::models::data::Data;
use crate::domain::models::page::Page;
use crate::domain::models::referral::Referrals;
use crate::domain::models::slug::Slug;
use crate::prelude::*;
use crate::renderer::RenderTasks;
use crate::renderer::RenderTask;
use crate::services::page_renderer::PageRenderer;
use hypertext::prelude::*;

use crate::renderer::partials::page::PageOptions;
use crate::renderer::partials::page::render_page;

pub fn render_referrals_page<'d>(data: &'d Data, tasks: &mut RenderTasks<'d>) {
    tasks.add(RenderReferralsPageTask {
        referrals: &data.referrals,
    });
}

struct RenderReferralsPageTask<'r> {
    referrals: &'r Referrals,
}

impl<'r> RenderTask for RenderReferralsPageTask<'r> {
    fn render(self: Box<Self>, renderer: &PageRenderer) -> Result<()> {
        let page = Page::new(Slug::new("/save"), Some("Referrals".to_string()), None);
        let slug = page.slug.clone();

        let content = maud! {
            article {
                ul {
                   @for referral in &self.referrals.referrals {
                       li {
                        h2 {
                            (referral.name)
                            a href=(referral.url.as_str()) {
                                p { (referral.url.as_str()) }
                            }
                        }
                        p { (referral.description) }
                       }
                   }
                }
            }
        };

        let options = PageOptions::new().with_main_class("referrals-page");

        let render = render_page(&page, &options, &content, maud! {});

        renderer.render_page(&slug, &render, None)
    }
}
