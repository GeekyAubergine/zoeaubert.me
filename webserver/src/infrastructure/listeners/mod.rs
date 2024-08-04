use about_listener::AboutListener;
use albums_listener::AlbumsListener;
use blog_posts_listener::BlogPostsListener;
use faq_listener::FaqListener;
use games_listener::GamesListener;
use lego_listener::LegoListener;
use mastodon_posts_listener::MastodonListener;
use micro_posts_listener::MicroPostsListener;
use microblog_archive_listener::MicroblogArchiveListener;
use status_lol_listener::StatusLolListener;
use tracing::debug;

use super::bus::Bus;

pub mod about_listener;
pub mod albums_listener;
pub mod blog_posts_listener;
pub mod faq_listener;
pub mod games_listener;
pub mod lego_listener;
pub mod mastodon_posts_listener;
pub mod micro_posts_listener;
pub mod microblog_archive_listener;
pub mod status_lol_listener;

pub fn register_listeners(mut bus: Bus) -> Bus {
    debug!("Registering listeners");
    bus.add_event_listener(Box::new(AboutListener::new()));
    bus.add_event_listener(Box::new(AlbumsListener::new()));
    bus.add_event_listener(Box::new(BlogPostsListener::new()));
    bus.add_event_listener(Box::new(FaqListener::new()));
    bus.add_event_listener(Box::new(GamesListener::new()));
    bus.add_event_listener(Box::new(LegoListener::new()));
    bus.add_event_listener(Box::new(MastodonListener::new()));
    bus.add_event_listener(Box::new(MicroPostsListener::new()));
    bus.add_event_listener(Box::new(MicroblogArchiveListener::new()));
    bus.add_event_listener(Box::new(StatusLolListener::new()));
    bus
}
