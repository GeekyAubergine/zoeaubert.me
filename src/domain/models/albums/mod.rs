use std::collections::HashMap;

use chrono::Datelike;

use crate::{
    domain::{
        models::{
            albums::{album::Album, album_photo::AlbumPhoto},
            slug::Slug,
        },
    },
    prelude::Result,
};

pub mod album;
pub mod album_photo;

#[derive(Clone, Default)]
pub struct Albums {
    albums: HashMap<Slug, Album>,
}

impl Albums {
    pub fn find_all_by_date(&self) -> Vec<&Album> {
        let mut albums = self.albums.values().collect::<Vec<&Album>>();

        albums.sort_by(|a, b| b.date.cmp(&a.date));

        albums
    }

    pub fn find_by_slug(&self, slug: &Slug) -> Option<&Album> {
        self.albums.get(slug)
    }

    pub fn find_grouped_by_year(&self) -> Vec<(u16, Vec<&Album>)> {
        let years: HashMap<u16, Vec<&Album>> =
            self.albums
                .values()
                .into_iter()
                .fold(HashMap::new(), |mut acc, album| {
                    acc.entry(album.date.year() as u16)
                        .or_insert_with(Vec::new)
                        .push(album);
                    acc
                });

        let mut years = years.into_iter().collect::<Vec<(u16, Vec<&Album>)>>();

        for (_, albums) in &mut years {
            albums.sort_by(|a, b| b.date.cmp(&a.date));
        }

        years.sort_by(|a, b| b.0.cmp(&a.0));

        years
    }

    pub fn find_all_album_photos(&self) -> Vec<AlbumPhoto> {
        let mut photos = Vec::new();

        for album in self.albums.values() {
            photos.extend(album.photos.iter().cloned());
        }

        photos
    }

    pub fn commit(&mut self, album: &Album) {
        self.albums.insert(album.slug.clone(), album.clone());
    }
}
