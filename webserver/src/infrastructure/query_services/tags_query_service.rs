use std::{collections::HashMap, sync::Arc};

use uuid::Uuid;

use crate::{
    domain::models::tag::Tag,
    infrastructure::{
        app_state::AppState,
        repos::tags_repo::{TagRepoEntity, TagsRepo},
    },
    prelude::Result,
};

fn process_tags_to_delete_for_entity(new_tags: Vec<Tag>, old_tags: Vec<TagRepoEntity>) -> Vec<Tag> {
    let mut tags_to_delete = vec![];

    for old_tag in old_tags {
        let tag = Tag::from_string(old_tag.tag());

        if !new_tags.contains(&tag) {
            tags_to_delete.push(tag);
        }
    }

    tags_to_delete
}

pub struct TagsQueryService {}

impl TagsQueryService {
    pub async fn find_tags_for_entity(
        state: &AppState,
        entity_uuid: &Uuid,
        include_deleted: bool,
    ) -> Result<Vec<Tag>> {
        let rows = match include_deleted {
            false => state.tags_repo().find_by_entity_uuid(entity_uuid).await?,
            true => {
                state
                    .tags_repo()
                    .find_by_entity_uuid_including_deleted(entity_uuid)
                    .await?
            }
        };

        let tags = rows
            .into_iter()
            .map(|r| Tag::from_string(r.tag()))
            .collect();

        Ok(tags)
    }

    pub async fn find_tags_for_entities(
        state: &AppState,
        entity_uuids: &[Uuid],
    ) -> Result<HashMap<Uuid, Vec<Tag>>> {
        let rows = state.tags_repo().find_by_entity_uuids(entity_uuids).await?;

        let mut tags_by_entity = HashMap::new();

        for (uuid, tags) in rows {
            let tags = tags
                .into_iter()
                .map(|r| Tag::from_string(r.tag()))
                .collect();
            tags_by_entity.insert(uuid, tags);
        }

        Ok(tags_by_entity)
    }

    pub async fn find_tag_counts(state: &AppState) -> Result<HashMap<Tag, i64>> {
        let tag_counts = state.tags_repo().find_tag_counts().await?;

        let tags = tag_counts
            .into_iter()
            .map(|(tag, count)| (Tag::from_string(&tag), count))
            .collect::<HashMap<Tag, i64>>();

        Ok(tags)
    }

    pub async fn update_tags_for_entity(
        state: &AppState,
        entity_uuid: &Uuid,
        new_tags: Vec<Tag>,
    ) -> Result<()> {
        let old_tags = state.tags_repo().find_by_entity_uuid(entity_uuid).await?;

        let tags_to_delete = process_tags_to_delete_for_entity(new_tags.clone(), old_tags);

        for tag in tags_to_delete {
            state.tags_repo().delete(entity_uuid, &tag.tag()).await?;
        }

        for tag in new_tags {
            state.tags_repo().commit(entity_uuid, &tag.tag()).await?;
        }

        Ok(())
    }
}
