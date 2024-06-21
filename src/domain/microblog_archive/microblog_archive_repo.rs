use std::{collections::HashMap, hash::Hash, sync::Arc};

use tokio::sync::RwLock;

use super::microblog_archive_models::MicroblogArchivePost;

#[derive(Debug, Clone, Default)]
pub struct MicroblogArchiveRepo {
    microblog_archive: Arc<RwLock<HashMap<String, MicroblogArchivePost>>>,
}

impl MicroblogArchiveRepo {
    pub async fn commit(&self, microblog_archive_post: MicroblogArchivePost) {
        let mut microblog_archive_ref = self.microblog_archive.write().await;
        microblog_archive_ref.insert(microblog_archive_post.slug().to_owned(), microblog_archive_post);
    }

    pub async fn get_all(&self) -> HashMap<String, MicroblogArchivePost> {
        self.microblog_archive.read().await.clone()
    }

    pub async fn get_by_slug(&self, slug: &str) -> Option<MicroblogArchivePost> {
        self.microblog_archive.read().await.get(slug).cloned()
    }

    pub async fn get_all_by_published_date(&self) -> Vec<MicroblogArchivePost> {
        let mut microblog_archive = self.microblog_archive.read().await.clone();

        let mut microblog_archive = microblog_archive
            .drain()
            .map(|(_, microblog_archive_post)| microblog_archive_post)
            .collect::<Vec<MicroblogArchivePost>>();

        microblog_archive.sort_by_key(|b| std::cmp::Reverse(*b.date()));

        microblog_archive
    }

    pub async fn get_all_by_tag(&self, tag: &str) -> Vec<MicroblogArchivePost> {
        let microblog_archive = self.microblog_archive.read().await.clone();

        microblog_archive
            .values()
            .filter(|microblog_archive_post| {
                microblog_archive_post
                    .tags()
                    .iter()
                    .any(|microblog_archive_post_tag| microblog_archive_post_tag.tag() == tag)
            })
            .cloned()
            .collect()
    }
}