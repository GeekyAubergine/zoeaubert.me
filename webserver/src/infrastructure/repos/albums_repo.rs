use std::{collections::HashMap, sync::Arc};

use chrono::Datelike;
use tokio::sync::RwLock;

use crate::domain::models::album::Album;

#[derive(Debug, Clone, Default)]
pub struct AlbumsRepo {
    albums: Arc<RwLock<HashMap<String, Album>>>,
}

impl AlbumsRepo {
    pub async fn commit(&self, album: Album) {
        let mut albums_ref = self.albums.write().await;
        albums_ref.insert(album.id(), album);
    }

    pub async fn get_all(&self) -> HashMap<String, Album> {
        self.albums.read().await.clone()
    }

    pub async fn get_all_by_date(&self) -> Vec<Album> {
        let mut albums = self.albums.read().await.clone();

        let mut albums = albums
            .drain()
            .map(|(_, album)| album)
            .collect::<Vec<Album>>();

        albums.sort_by(|a, b| b.date().cmp(&a.date()));

        albums
    }

    pub async fn group_by_year(&self) -> Vec<(i32, Vec<Album>)> {
        let albums = self.albums.read().await.clone();
        let mut grouped_albums = HashMap::new();

        for album in albums.values() {
            let year = album.date().year();
            grouped_albums
                .entry(year)
                .or_insert_with(Vec::new)
                .push(album.clone());
        }

        let mut grouped_albums: Vec<_> = grouped_albums.into_iter().collect();
        grouped_albums.sort_by(|a, b| b.0.cmp(&a.0));

        for (_, albums) in &mut grouped_albums {
            albums.sort_by(|a, b| b.date().cmp(a.date()));
        }

        grouped_albums
    }
}
