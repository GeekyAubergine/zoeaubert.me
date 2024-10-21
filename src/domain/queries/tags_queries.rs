use std::collections::HashMap;

use crate::{domain::state::State, prelude::*};

use crate::domain::models::tag::Tag;

use super::omni_post_queries::{find_all_omni_posts, OmniPostFilterFlags};

pub async fn find_tag_counts(state: &impl State) -> Result<HashMap<Tag, usize>> {
    let posts = find_all_omni_posts(state, OmniPostFilterFlags::all()).await?;

    let mut tags = HashMap::new();

    for post in posts {
        for tag in post.tags() {
            *tags.entry(tag.clone()).or_insert(0) += 1;
        }
    }

    Ok(tags)
}
