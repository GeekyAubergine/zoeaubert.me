use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use crate::domain::models::album::Album;

#[derive(Debug, Clone, Default)]
pub struct AlbumsRepo {
    albums: Arc<RwLock<HashMap<String, Album>>>
}

impl AlbumsRepo {
    pub async fn commit(&self, album: Album) {
        let mut albums_ref = self.albums.write().await;
        albums_ref.insert(album.id(), album);
    }

    pub async fn get_all(&self) -> HashMap<String, Album> {
        self.albums.read().await.clone()
    }

    pub async fn get_by_slug(&self, slug: &str) -> Option<Album> {
        self.albums.read().await.get(slug).cloned()
    }
}

