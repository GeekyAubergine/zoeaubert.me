use games_listener::GamesListener;
use lego_listener::LegoListener;
use mastodon_posts_listener::MastodonListener;
use status_lol_listener::StatusLolListener;
use tracing::debug;

use super::bus::Bus;

pub mod games_listener;
pub mod lego_listener;
pub mod mastodon_posts_listener;
pub mod status_lol_listener;

pub fn register_listeners(mut bus: Bus) -> Bus {
    debug!("Registering listeners");
    bus.add_event_listener(Box::new(GamesListener::new()));
    bus.add_event_listener(Box::new(LegoListener::new()));
    bus.add_event_listener(Box::new(MastodonListener::new()));
    bus.add_event_listener(Box::new(StatusLolListener::new()));
    bus
}
