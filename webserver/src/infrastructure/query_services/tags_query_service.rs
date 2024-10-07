use std::{collections::HashMap, sync::Arc};

use uuid::Uuid;

use crate::{
    domain::models::{tag::Tag, UuidIdentifiable},
    infrastructure::{
        app_state::AppState,
        repos::tags_repo::{TagRepoEntity, TagsRepo, TagsRepoDb},
    },
    prelude::Result,
};

fn process_tags_to_delete_for_entity(new_tags: &[Tag], old_tags: &[Tag]) -> Vec<Tag> {
    let mut tags_to_delete = vec![];

    for old_tag in old_tags {
        if !new_tags.contains(&old_tag) {
            tags_to_delete.push(old_tag.clone());
        }
    }

    tags_to_delete
}

pub async fn find_tags_for_entity<E>(entity: &E, tags_repo: &impl TagsRepo) -> Result<Vec<Tag>>
where
    E: UuidIdentifiable,
{
    tags_repo
        .find_by_entity_uuid(entity.uuid())
        .await
        .map(|tags| tags.into_iter().map(|tag| tag.tag).collect())
}

pub async fn find_tags_for_entities<E>(
    entities: &[E],
    tags_repo: &impl TagsRepo,
) -> Result<HashMap<Uuid, Vec<Tag>>>
where
    E: UuidIdentifiable,
{
    let mut tags = HashMap::new();

    for entity in entities {
        let entity_tags = find_tags_for_entity(entity, tags_repo).await?;
        tags.insert(entity.uuid().clone(), entity_tags);
    }

    Ok(tags)
}


pub async fn commit_tags_for_entity<E>(
    entity: &E,
    tags: &[Tag],
    tags_repo: &impl TagsRepo,
) -> Result<()>
where E: UuidIdentifiable
{
    let mut existing_tags = find_tags_for_entity(entity, tags_repo).await?;

    let tags_to_delete = process_tags_to_delete_for_entity(tags, &existing_tags);

    for tag in tags_to_delete {
        tags_repo.delete(entity.uuid(), &tag.tag()).await?;
    }

    for tag in tags {
        tags_repo.commit(entity.uuid(), &tag.tag()).await?;
    }

    Ok(())
}
