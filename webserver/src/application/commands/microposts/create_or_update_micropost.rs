use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::application::events::Event::MicroPostUpdated;
use crate::domain::models::media::MediaUuid;
use crate::domain::models::micro_post::{MicroPost, MicroPostUuid};
use crate::infrastructure::query_services::micro_posts_query_service::commit_micropost;
use crate::infrastructure::query_services::tags_query_service::commit_tags_for_entity;
use crate::{domain::models::tag::Tag, infrastructure::app_state::AppState};

use crate::prelude::Result;

pub async fn create_or_update_micropost(
    uuid: MicroPostUuid,
    slug: String,
    date: DateTime<Utc>,
    content: String,
    tags: Vec<Tag>,
    media: Vec<MediaUuid>,
    state: &AppState,
) -> Result<MicroPost> {
    let micro_post = MicroPost::new(uuid.clone(), slug, date, content, media);

    commit_micropost(&micro_post, state.micro_posts_repo()).await?;

    commit_tags_for_entity(&micro_post, &tags, state.tags_repo()).await?;

    state
        .dispatch_event(MicroPostUpdated { uuid })
        .await?;

    Ok(micro_post)
}
