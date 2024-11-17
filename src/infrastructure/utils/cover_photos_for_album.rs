use bitflags::iter;

use crate::domain::models::album::{Album, AlbumPhoto};

pub fn cover_photos_for_album(album: &Album) -> Vec<&AlbumPhoto> {
    let (featured, non_featured): (Vec<&AlbumPhoto>, Vec<&AlbumPhoto>) =
        album.photos.iter().partition(|photo| photo.featured);

    let (featured_portrait, featured_landscape): (Vec<&AlbumPhoto>, Vec<&AlbumPhoto>) = featured
        .iter()
        .partition(|photo| !photo.small_image.dimensions.orientation().is_landscape());
    let (non_featured_portrait, non_featured_landscape): (Vec<&AlbumPhoto>, Vec<&AlbumPhoto>) =
        non_featured
            .iter()
            .partition(|photo| !photo.small_image.dimensions.orientation().is_landscape());

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
    use std::path::Path;

    use chrono::Utc;
    use uuid::Uuid;

    use crate::{
        domain::models::{
            album::{Album, AlbumPhoto},
            image::{Image, ImageDimensions},
            slug::Slug,
        },
        infrastructure::utils::cover_photos_for_album::cover_photos_for_album,
    };

    #[test]
    fn it_should_return_empty_vec_when_no_photos() {
        let album = Album::new(Slug::new(""), "title".to_string(), None, Utc::now(), 0);
        let photos = cover_photos_for_album(&album);
        assert_eq!(photos.len(), 0);
    }

    #[test]
    fn it_should_return_first_featured_landscaped_photo() {
        let mut album = Album::new(Slug::new(""), "title".to_string(), None, Utc::now(), 0);
        album.add_photo(
            AlbumPhoto::new(
                Slug::new("file_1"),
                "".to_string(),
                Utc::now(),
                vec![],
                Image::new(&Path::new("file_1"), "", &ImageDimensions::new(30, 20)),
                Image::new(&Path::new("file_1"), "", &ImageDimensions::new(30, 20)),
                Image::new(&Path::new("file_1"), "", &ImageDimensions::new(30, 20)),
            )
            .set_featured(true),
        );
        album.add_photo(
            AlbumPhoto::new(
                Slug::new("file_2"),
                "".to_string(),
                Utc::now(),
                vec![],
                Image::new(&Path::new("file_2"), "", &ImageDimensions::new(20, 30)),
                Image::new(&Path::new("file_2"), "", &ImageDimensions::new(20, 30)),
                Image::new(&Path::new("file_2"), "", &ImageDimensions::new(20, 30)),
            )
            .set_featured(true),
        );
        album.add_photo(
            AlbumPhoto::new(
                Slug::new("file_3"),
                "".to_string(),
                Utc::now(),
                vec![],
                Image::new(&Path::new("file_3"), "", &ImageDimensions::new(30, 20)),
                Image::new(&Path::new("file_3"), "", &ImageDimensions::new(30, 20)),
                Image::new(&Path::new("file_3"), "", &ImageDimensions::new(30, 20)),
            )
            .set_featured(false),
        );
        let photos = cover_photos_for_album(&album);
        assert_eq!(photos.len(), 1);
        assert_eq!(photos[0].slug, Slug::new("file_1"));
    }

    #[test]
    fn it_should_return_first_two_featured_portrait_photos() {
        let mut album = Album::new(Slug::new(""), "title".to_string(), None, Utc::now(), 0);
        album.add_photo(
            AlbumPhoto::new(
                Slug::new("file_1"),
                "".to_string(),
                Utc::now(),
                vec![],
                Image::new(&Path::new("file_1"), "", &ImageDimensions::new(20, 30)),
                Image::new(&Path::new("file_1"), "", &ImageDimensions::new(20, 30)),
                Image::new(&Path::new("file_1"), "", &ImageDimensions::new(20, 30)),
            )
            .set_featured(true),
        );
        album.add_photo(
            AlbumPhoto::new(
                Slug::new("file_2"),
                "".to_string(),
                Utc::now(),
                vec![],
                Image::new(&Path::new("file_2"), "", &ImageDimensions::new(20, 30)),
                Image::new(&Path::new("file_2"), "", &ImageDimensions::new(20, 30)),
                Image::new(&Path::new("file_2"), "", &ImageDimensions::new(20, 30)),
            )
            .set_featured(true),
        );
        album.add_photo(
            AlbumPhoto::new(
                Slug::new("file_3"),
                "".to_string(),
                Utc::now(),
                vec![],
                Image::new(&Path::new("file_3"), "", &ImageDimensions::new(20, 30)),
                Image::new(&Path::new("file_3"), "", &ImageDimensions::new(20, 30)),
                Image::new(&Path::new("file_3"), "", &ImageDimensions::new(20, 30)),
            )
            .set_featured(false),
        );
        let photos = cover_photos_for_album(&album);
        assert_eq!(photos.len(), 2);
        assert_eq!(photos[0].slug, Slug::new("file_1"));
        assert_eq!(photos[1].slug, Slug::new("file_2"));
    }

    #[test]
    fn it_should_return_first_featured_portrait_and_first_non_featured_portrait() {
        let mut album = Album::new(Slug::new(""), "title".to_string(), None, Utc::now(), 0);
        album.add_photo(
            AlbumPhoto::new(
                Slug::new("file_1"),
                "".to_string(),
                Utc::now(),
                vec![],
                Image::new(&Path::new("file_1"), "", &ImageDimensions::new(20, 30)),
                Image::new(&Path::new("file_1"), "", &ImageDimensions::new(20, 30)),
                Image::new(&Path::new("file_1"), "", &ImageDimensions::new(20, 30)),
            )
            .set_featured(true),
        );
        album.add_photo(
            AlbumPhoto::new(
                Slug::new("file_2"),
                "".to_string(),
                Utc::now(),
                vec![],
                Image::new(&Path::new("file_1"), "", &ImageDimensions::new(20, 30)),
                Image::new(&Path::new("file_1"), "", &ImageDimensions::new(20, 30)),
                Image::new(&Path::new("file_1"), "", &ImageDimensions::new(20, 30)),
            )
            .set_featured(false),
        );
        album.add_photo(
            AlbumPhoto::new(
                Slug::new("file_3"),
                "".to_string(),
                Utc::now(),
                vec![],
                Image::new(&Path::new("file_1"), "", &ImageDimensions::new(30, 20)),
                Image::new(&Path::new("file_1"), "", &ImageDimensions::new(30, 20)),
                Image::new(&Path::new("file_1"), "", &ImageDimensions::new(30, 20)),
            )
            .set_featured(false),
        );
        let photos = cover_photos_for_album(&album);
        assert_eq!(photos.len(), 2);
        assert_eq!(photos[0].slug, Slug::new("file_1"));
        assert_eq!(photos[1].slug, Slug::new("file_2"));
    }

    #[test]
    fn it_should_return_first_non_featured_landscaped_photo() {
        let mut album = Album::new(Slug::new(""), "title".to_string(), None, Utc::now(), 0);
        album.add_photo(
            AlbumPhoto::new(
                Slug::new("file_1"),
                "".to_string(),
                Utc::now(),
                vec![],
                Image::new(&Path::new("file_1"), "", &ImageDimensions::new(20, 30)),
                Image::new(&Path::new("file_1"), "", &ImageDimensions::new(20, 30)),
                Image::new(&Path::new("file_1"), "", &ImageDimensions::new(20, 30)),
            )
            .set_featured(false),
        );
        album.add_photo(
            AlbumPhoto::new(
                Slug::new("file_2"),
                "".to_string(),
                Utc::now(),
                vec![],
                Image::new(&Path::new("file_2"), "", &ImageDimensions::new(20, 30)),
                Image::new(&Path::new("file_2"), "", &ImageDimensions::new(20, 30)),
                Image::new(&Path::new("file_2"), "", &ImageDimensions::new(20, 30)),
            )
            .set_featured(false),
        );
        album.add_photo(
            AlbumPhoto::new(
                Slug::new("file_3"),
                "".to_string(),
                Utc::now(),
                vec![],
                Image::new(&Path::new("file_3"), "", &ImageDimensions::new(30, 20)),
                Image::new(&Path::new("file_3"), "", &ImageDimensions::new(30, 20)),
                Image::new(&Path::new("file_3"), "", &ImageDimensions::new(30, 20)),
            )
            .set_featured(false),
        );
        let photos = cover_photos_for_album(&album);
        assert_eq!(photos.len(), 1);
        assert_eq!(photos[0].slug, Slug::new("file_3"));
    }

    #[test]
    fn it_should_return_first_two_non_featured_portrait_photos() {
        let mut album = Album::new(Slug::new(""), "title".to_string(), None, Utc::now(), 0);

        album.add_photo(
            AlbumPhoto::new(
                Slug::new("file_1"),
                "".to_string(),
                Utc::now(),
                vec![],
                Image::new(&Path::new("file_1"), "", &ImageDimensions::new(20, 30)),
                Image::new(&Path::new("file_1"), "", &ImageDimensions::new(20, 30)),
                Image::new(&Path::new("file_1"), "", &ImageDimensions::new(20, 30)),
            )
            .set_featured(false),
        );
        album.add_photo(
            AlbumPhoto::new(
                Slug::new("file_2"),
                "".to_string(),
                Utc::now(),
                vec![],
                Image::new(&Path::new("file_2"), "", &ImageDimensions::new(20, 30)),
                Image::new(&Path::new("file_2"), "", &ImageDimensions::new(20, 30)),
                Image::new(&Path::new("file_2"), "", &ImageDimensions::new(20, 30)),
            )
            .set_featured(false),
        );
        album.add_photo(
            AlbumPhoto::new(
                Slug::new("file_3"),
                "".to_string(),
                Utc::now(),
                vec![],
                Image::new(&Path::new("file_3"), "", &ImageDimensions::new(20, 30)),
                Image::new(&Path::new("file_3"), "", &ImageDimensions::new(20, 30)),
                Image::new(&Path::new("file_3"), "", &ImageDimensions::new(20, 30)),
            )
            .set_featured(false),
        );
        let photos = cover_photos_for_album(&album);
        assert_eq!(photos.len(), 2);
        assert_eq!(photos[0].slug, Slug::new("file_1"));
        assert_eq!(photos[1].slug, Slug::new("file_2"));
    }

    #[test]
    fn it_should_return_first_featured_photo_if_no_previous_rule_met() {
        let mut album = Album::new(Slug::new(""), "title".to_string(), None, Utc::now(), 0);
        album.add_photo(
            AlbumPhoto::new(
                Slug::new("file_1"),
                "".to_string(),
                Utc::now(),
                vec![],
                Image::new(&Path::new("file_1"), "", &ImageDimensions::new(20, 30)),
                Image::new(&Path::new("file_1"), "", &ImageDimensions::new(20, 30)),
                Image::new(&Path::new("file_1"), "", &ImageDimensions::new(20, 30)),
            )
            .set_featured(true),
        );
        let photos = cover_photos_for_album(&album);
        assert_eq!(photos.len(), 1);
        assert_eq!(photos[0].slug, Slug::new("file_1"));
    }

    #[test]
    fn it_should_return_first_non_featured_photo_if_no_previous_rule_met() {
        let mut album = Album::new(Slug::new(""), "title".to_string(), None, Utc::now(), 0);
        album.add_photo(
            AlbumPhoto::new(
                Slug::new("file_1"),
                "".to_string(),
                Utc::now(),
                vec![],
                Image::new(&Path::new("file_1"), "", &ImageDimensions::new(20, 30)),
                Image::new(&Path::new("file_1"), "", &ImageDimensions::new(20, 30)),
                Image::new(&Path::new("file_1"), "", &ImageDimensions::new(20, 30)),
            )
            .set_featured(false),
        );
        let photos = cover_photos_for_album(&album);
        assert_eq!(photos.len(), 1);
        assert_eq!(photos[0].slug, Slug::new("file_1"));
    }
}
