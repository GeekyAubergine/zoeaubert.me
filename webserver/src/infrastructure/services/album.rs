use bitflags::iter;

use crate::domain::models::album::{Album, AlbumPhoto};

pub fn ordered_photos_for_album(album: &Album) -> Vec<&AlbumPhoto> {
    album
        .photos_order()
        .iter()
        .filter_map(|file_name| album.photos_map().get(file_name))
        .collect()
}

pub fn cover_photos_for_album(album: &Album) -> Vec<&AlbumPhoto> {
    let (featured, non_featured): (Vec<&AlbumPhoto>, Vec<&AlbumPhoto>) =
        ordered_photos_for_album(album)
            .iter()
            .partition(|photo| photo.featured());

    let (featured_portrait, featured_landscape): (Vec<&AlbumPhoto>, Vec<&AlbumPhoto>) = featured
        .iter()
        .partition(|photo| !photo.small_image().is_landscape());
    let (non_featured_portrait, non_featured_landscape): (Vec<&AlbumPhoto>, Vec<&AlbumPhoto>) =
        non_featured
            .iter()
            .partition(|photo| !photo.small_image().is_landscape());

    // If featured landscape
    if let Some(photo) = featured_landscape.first() {
        return vec![photo];
    }

    // If two featured portrait
    if let (Some(photo), Some(photo2)) = (featured_portrait.first(), featured_portrait.get(1)) {
        return vec![photo, photo2];
    }

    // If 1 featured portrait, use that and 1 non-featured portrait
    if let (Some(photo), Some(photo2)) = (featured_portrait.first(), non_featured_portrait.first())
    {
        return vec![photo, photo2];
    }

    // If non-featured landscape
    if let Some(photo) = non_featured_landscape.first() {
        return vec![photo];
    }

    // If 2 non-featured portrait
    if let (Some(photo), Some(photo2)) =
        (non_featured_portrait.first(), non_featured_portrait.get(1))
    {
        return vec![photo, photo2];
    }

    if let Some(photo) = featured_portrait.first() {
        return vec![photo];
    }

    if let Some(photo) = non_featured_portrait.first() {
        return vec![photo];
    }

    vec![]
}

#[cfg(test)]
mod test {
    use chrono::Utc;
    use uuid::Uuid;

    use crate::{
        domain::models::{
            album::{Album, AlbumPhoto},
            media::image::Image,
        },
        infrastructure::services::album::cover_photos_for_album,
    };

    #[test]
    fn it_should_return_empty_vec_when_no_photos() {
        let album = Album::new(&Uuid::new_v4(), "title".to_string(), None, Utc::now());
        let photos = cover_photos_for_album(&album);
        assert_eq!(photos.len(), 0);
    }

    #[test]
    fn it_should_return_first_featured_landscaped_photo() {
        let mut album = Album::new(&Uuid::new_v4(), "title".to_string(), None, Utc::now());
        album.add_photo(AlbumPhoto::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Image::new(&Uuid::new_v4(), "file_1", "", 30, 20),
            Image::new(&Uuid::new_v4(), "file_1", "", 30, 20),
            Image::new(&Uuid::new_v4(), "file_1", "", 30, 20),
            "file_name1".to_string(),
            vec![],
            true,
        ));
        album.add_photo(AlbumPhoto::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Image::new(&Uuid::new_v4(), "file_2", "", 20, 30),
            Image::new(&Uuid::new_v4(), "file_2", "", 20, 30),
            Image::new(&Uuid::new_v4(), "file_2", "", 20, 30),
            "file_name2".to_string(),
            vec![],
            true,
        ));
        album.add_photo(AlbumPhoto::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Image::new(&Uuid::new_v4(), "file_3", "", 30, 20),
            Image::new(&Uuid::new_v4(), "file_3", "", 30, 20),
            Image::new(&Uuid::new_v4(), "file_3", "", 30, 20),
            "file_name3".to_string(),
            vec![],
            false,
        ));
        let photos = cover_photos_for_album(&album);
        assert_eq!(photos.len(), 1);
        assert_eq!(photos[0].file_name(), "file_name1");
    }

    #[test]
    fn it_should_return_first_two_featured_portrait_photos() {
        let mut album = Album::new(&Uuid::new_v4(), "title".to_string(), None, Utc::now());
        album.add_photo(AlbumPhoto::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Image::new(&Uuid::new_v4(), "file_1", "", 20, 30),
            Image::new(&Uuid::new_v4(), "file_1", "", 20, 30),
            Image::new(&Uuid::new_v4(), "file_1", "", 20, 30),
            "file_name1".to_string(),
            vec![],
            true,
        ));
        album.add_photo(AlbumPhoto::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Image::new(&Uuid::new_v4(), "file_2", "", 20, 30),
            Image::new(&Uuid::new_v4(), "file_2", "", 20, 30),
            Image::new(&Uuid::new_v4(), "file_2", "", 20, 30),
            "file_name2".to_string(),
            vec![],
            true,
        ));
        album.add_photo(AlbumPhoto::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Image::new(&Uuid::new_v4(), "file_3", "", 20, 30),
            Image::new(&Uuid::new_v4(), "file_3", "", 20, 30),
            Image::new(&Uuid::new_v4(), "file_3", "", 20, 30),
            "file_name3".to_string(),
            vec![],
            false,
        ));
        let photos = cover_photos_for_album(&album);
        assert_eq!(photos.len(), 2);
        assert_eq!(photos[0].file_name(), "file_name1");
        assert_eq!(photos[1].file_name(), "file_name2");
    }

    #[test]
    fn it_should_return_first_featured_portrait_and_first_non_featured_portrait() {
        let mut album = Album::new(&Uuid::new_v4(), "title".to_string(), None, Utc::now());
        album.add_photo(AlbumPhoto::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Image::new(&Uuid::new_v4(), "file_1", "", 20, 30),
            Image::new(&Uuid::new_v4(), "file_1", "", 20, 30),
            Image::new(&Uuid::new_v4(), "file_1", "", 20, 30),
            "file_name1".to_string(),
            vec![],
            true,
        ));
        album.add_photo(AlbumPhoto::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Image::new(&Uuid::new_v4(), "file_2", "", 20, 30),
            Image::new(&Uuid::new_v4(), "file_2", "", 20, 30),
            Image::new(&Uuid::new_v4(), "file_2", "", 20, 30),
            "file_name2".to_string(),
            vec![],
            false,
        ));
        album.add_photo(AlbumPhoto::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Image::new(&Uuid::new_v4(), "file_3", "", 30, 20),
            Image::new(&Uuid::new_v4(), "file_3", "", 30, 20),
            Image::new(&Uuid::new_v4(), "file_3", "", 30, 20),
            "file_name3".to_string(),
            vec![],
            false,
        ));
        let photos = cover_photos_for_album(&album);
        assert_eq!(photos.len(), 2);
        assert_eq!(photos[0].file_name(), "file_name1");
        assert_eq!(photos[1].file_name(), "file_name2");
    }

    #[test]
    fn it_should_return_first_non_featured_landscaped_photo() {
        let mut album = Album::new(&Uuid::new_v4(), "title".to_string(), None, Utc::now());
        album.add_photo(AlbumPhoto::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Image::new(&Uuid::new_v4(), "file_1", "", 20, 30),
            Image::new(&Uuid::new_v4(), "file_1", "", 20, 30),
            Image::new(&Uuid::new_v4(), "file_1", "", 20, 30),
            "file_name1".to_string(),
            vec![],
            false,
        ));
        album.add_photo(AlbumPhoto::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Image::new(&Uuid::new_v4(), "file_2", "", 20, 30),
            Image::new(&Uuid::new_v4(), "file_2", "", 20, 30),
            Image::new(&Uuid::new_v4(), "file_2", "", 20, 30),
            "file_name2".to_string(),
            vec![],
            false,
        ));
        album.add_photo(AlbumPhoto::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Image::new(&Uuid::new_v4(), "file_3", "", 30, 20),
            Image::new(&Uuid::new_v4(), "file_3", "", 30, 20),
            Image::new(&Uuid::new_v4(), "file_3", "", 30, 20),
            "file_name3".to_string(),
            vec![],
            false,
        ));
        let photos = cover_photos_for_album(&album);
        assert_eq!(photos.len(), 1);
        assert_eq!(photos[0].file_name(), "file_name3");
    }

    #[test]
    fn it_should_return_first_two_non_featured_portrait_photos() {
        let mut album = Album::new(&Uuid::new_v4(), "title".to_string(), None, Utc::now());
        album.add_photo(AlbumPhoto::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Image::new(&Uuid::new_v4(), "file_1", "", 20, 30),
            Image::new(&Uuid::new_v4(), "file_1", "", 20, 30),
            Image::new(&Uuid::new_v4(), "file_1", "", 20, 30),
            "file_name1".to_string(),
            vec![],
            false,
        ));
        album.add_photo(AlbumPhoto::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Image::new(&Uuid::new_v4(), "file_2", "", 20, 30),
            Image::new(&Uuid::new_v4(), "file_2", "", 20, 30),
            Image::new(&Uuid::new_v4(), "file_2", "", 20, 30),
            "file_name2".to_string(),
            vec![],
            false,
        ));
        album.add_photo(AlbumPhoto::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Image::new(&Uuid::new_v4(), "file_3", "", 20, 30),
            Image::new(&Uuid::new_v4(), "file_3", "", 20, 30),
            Image::new(&Uuid::new_v4(), "file_3", "", 20, 30),
            "file_name3".to_string(),
            vec![],
            false,
        ));
        let photos = cover_photos_for_album(&album);
        assert_eq!(photos.len(), 2);
        assert_eq!(photos[0].file_name(), "file_name1");
        assert_eq!(photos[1].file_name(), "file_name2");
    }

    #[test]
    fn it_should_return_first_featured_photo_if_no_previous_rule_met() {
        let mut album = Album::new(&Uuid::new_v4(), "title".to_string(), None, Utc::now());
        album.add_photo(AlbumPhoto::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Image::new(&Uuid::new_v4(), "file_1", "", 20, 30),
            Image::new(&Uuid::new_v4(), "file_1", "", 20, 30),
            Image::new(&Uuid::new_v4(), "file_1", "", 20, 30),
            "file_name1".to_string(),
            vec![],
            true,
        ));
        let photos = cover_photos_for_album(&album);
        assert_eq!(photos.len(), 1);
        assert_eq!(photos[0].file_name(), "file_name1");
    }

    #[test]
    fn it_should_return_first_non_featured_photo_if_no_previous_rule_met() {
        let mut album = Album::new(&Uuid::new_v4(), "title".to_string(), None, Utc::now());
        album.add_photo(AlbumPhoto::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Image::new(&Uuid::new_v4(), "file_1", "", 20, 30),
            Image::new(&Uuid::new_v4(), "file_1", "", 20, 30),
            Image::new(&Uuid::new_v4(), "file_1", "", 20, 30),
            "file_name1".to_string(),
            vec![],
            false,
        ));
        let photos = cover_photos_for_album(&album);
        assert_eq!(photos.len(), 1);
        assert_eq!(photos[0].file_name(), "file_name1");
    }
}
