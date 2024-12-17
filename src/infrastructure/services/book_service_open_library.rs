use chrono::{DateTime, Datelike, Utc};
use dotenvy_macro::dotenv;
use reqwest::Client;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use url::Url;

use serde::{Deserialize, Serialize};

use crate::domain::models::book::{Book, BookID, BookReview};
use crate::domain::models::content::Content;
use crate::domain::models::omni_post::OmniPost;
use crate::domain::models::tag::Tag;
use crate::domain::models::tv_show::{TvShow, TvShowId, TvShowReview};
use crate::domain::services::{
    BookService, FileService, ImageService, NetworkService, TvShowsService,
};
use crate::domain::state::State;

use crate::error::{BookError, TvShowsError};
use crate::infrastructure::utils::date::parse_date;
use crate::infrastructure::utils::parse_content_into_book_review::{
    self, parse_content_into_book_review,
};
use crate::infrastructure::utils::parse_omni_post_content_into_movie_review::parse_content_into_movie_review;
use crate::infrastructure::utils::parse_omni_post_into_tv_show_reviews::{
    self, parse_content_into_tv_show_review,
};
use crate::prelude::*;

use super::file_service_disk::FileServiceDisk;

const FILE_NAME: &str = "book_cache.json";

const BLACK_LIBRARY_PUBLISHER: &str = "Black Library";
const BLACK_LIBRARY_THE_PUBLISHER: &str = "Black Library, The";
const GAMES_WORKSHOP_PUBLISHER: &str = "Games Workshop";
const GAMES_WORKSHOP_LTD_PUBLISHER: &str = "Games Workshop, Limited";
const WARHAMMER_TAG: &str = "Warhammer";

fn make_file_path(file_service: &impl FileService) -> PathBuf {
    file_service.make_archive_file_path(&Path::new(FILE_NAME))
}

fn make_search_url(title: &str) -> Url {
    let title = title
        .to_lowercase()
        .replace("(warhammer 40,000)", "")
        .replace('&', "")
        .replace(' ', "+");

    format!("https://openlibrary.org/search.json?q={}", title)
        .parse()
        .unwrap()
}

async fn find_book(
    state: &impl State,
    title: &str,
    author: &str,
    tags: &[Tag],
) -> Result<Option<OpenLibraryBook>> {
    let response = state
        .network_service()
        .download_json::<OpenLibrarySearchResponse>(&make_search_url(title))
        .await?;

    if (tags.contains(&Tag::from_string(WARHAMMER_TAG))) {
        let books = response
            .docs
            .into_iter()
            .filter(|doc| match &doc.publisher {
                Some(publishers) => {
                    publishers.contains(&BLACK_LIBRARY_PUBLISHER.to_string())
                        || publishers.contains(&BLACK_LIBRARY_THE_PUBLISHER.to_string())
                        || publishers.contains(&GAMES_WORKSHOP_PUBLISHER.to_string())
                        || publishers.contains(&GAMES_WORKSHOP_LTD_PUBLISHER.to_string())
                }
                None => false,
            })
            .filter(|doc| doc.cover_i.is_some())
            .collect::<Vec<OpenLibraryBook>>();

        return Ok(books.first().cloned());
    }

    let first_author = author.split(',').next().unwrap().trim();

    let books = response
        .docs
        .into_iter()
        .filter(|doc| match doc.author_name {
            Some(ref authors) => {
                authors.contains(&author.to_string()) || authors.contains(&first_author.to_string())
            }
            None => false,
        })
        .filter(|doc| doc.cover_i.is_some())
        .collect::<Vec<OpenLibraryBook>>();

    Ok(books.first().cloned())
}

#[derive(Debug, Clone, Deserialize)]
struct OpenLibraryBook {
    cover_i: Option<u32>,
    publisher: Option<Vec<String>>,
    author_name: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
struct OpenLibrarySearchResponse {
    docs: Vec<OpenLibraryBook>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct BookServiceData {
    books: HashMap<String, Option<Book>>,
}

#[derive(Debug, Clone)]
pub struct BookServiceOpenLibrary {
    client: Client,
    data: Arc<RwLock<BookServiceData>>,
    file_service: FileServiceDisk,
}

impl BookServiceOpenLibrary {
    pub async fn new() -> Result<Self> {
        let file_service = FileServiceDisk::new();

        let data = file_service
            .read_json_file_or_default(&make_file_path(&file_service))
            .await?;

        Ok(Self {
            client: Client::new(),
            data: Arc::new(RwLock::new(data)),
            file_service,
        })
    }
}

#[async_trait::async_trait]
impl BookService for BookServiceOpenLibrary {
    async fn find_book(
        &self,
        state: &impl State,
        title: &str,
        author: &str,
        tags: &[Tag],
    ) -> Result<Option<Book>> {
        if let Some(book) = self.data.read().await.books.get(title) {
            match book {
                Some(book) => return Ok(Some(book.clone())),
                None => return Ok(None),
            }
        }

        println!("Searching {}", make_search_url(title));

        let book = find_book(state, title, author, tags).await?;

        if let Some(cover_i) = book.map(|doc| doc.cover_i).flatten() {
            let image_url = format!("https://covers.openlibrary.org/b/id/{:?}-L.jpg", cover_i)
                .parse()
                .unwrap();

            let image_path = &format!("books/{}-cover-400.jpg", cover_i);
            let image_path = Path::new(&image_path);

            let image = state
                .image_service()
                .copy_image_from_url(
                    state,
                    &image_url,
                    &image_path,
                    &format!("Cover for book {}", title),
                )
                .await?;

            let book = Book {
                title: title.to_string(),
                cover: image,
                id: BookID::OpenLibrary { id: cover_i },
            };

            let mut data = self.data.write().await;
            data.books.insert(title.to_string(), Some(book.clone()));

            self.file_service
                .write_json_file(&make_file_path(&self.file_service), &data.clone())
                .await?;

            return Ok(Some(book));
        } else {
            let mut data = self.data.write().await;
            data.books.insert(title.to_string(), None);

            self.file_service
                .write_json_file(&make_file_path(&self.file_service), &data.clone())
                .await?;
        }

        Ok(None)
    }

    async fn book_review_from_content(
        &self,
        state: &impl State,
        post: &Content,
    ) -> Result<BookReview> {
        let review = parse_content_into_book_review(post)?;

        let book = self
            .find_book(state, &review.title, &review.author, &post.tags())
            .await?
            .ok_or(BookError::unable_to_parse_and_find_book_title(review.title))?;

        Ok(BookReview {
            book,
            score: review.score,
            review: review.review,
            source_content: post.clone(),
        })
    }
}
