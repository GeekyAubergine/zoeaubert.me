use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, RwLock};

use htmlentity::entity::{decode, ICodedDataTrait};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, instrument, warn};
use url::Url;

use crate::domain::models::book::{Book, BookID, BookReview};
use crate::domain::models::post::Post;
use crate::domain::models::source_post::SourcePost;
use crate::error::BookError;
use crate::prelude::*;

use crate::services::cdn_service::CdnFile;
use crate::services::file_service::{ArchiveFile, FileService, ReadableFile, WritableFile};
use crate::services::media_service::MediaService;
use crate::utils::parse_content_into_book_review::parse_content_into_book_review;
use crate::{domain::models::tag::Tag, services::ServiceContext};

const FILE_NAME: &str = "book_cache.json";

const BLACK_LIBRARY_PUBLISHER: &str = "Black Library";
const BLACK_LIBRARY_THE_PUBLISHER: &str = "Black Library, The";
const GAMES_WORKSHOP_PUBLISHER: &str = "Games Workshop";
const GAMES_WORKSHOP_LTD_PUBLISHER: &str = "Games Workshop, Limited";
const WARHAMMER_TAG: &str = "Warhammer";

#[derive(Debug)]
pub struct BookService {
    file: ArchiveFile,
    books: Arc<RwLock<HashMap<String, Option<Book>>>>,
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

fn make_search_url(title: &str) -> Url {
    let title = decode(title.as_bytes()).to_string().unwrap();

    let title = title
        .to_lowercase()
        .replace("(warhammer 40,000)", "")
        .replace('&', "")
        .replace(' ', "+");

    format!("https://openlibrary.org/search.json?q={}", title)
        .parse()
        .unwrap()
}

#[instrument(err, skip_all, fields(book.title=%title, book.author=&author))]
async fn query_book_api(
    ctx: &ServiceContext,
    title: &str,
    author: &str,
    tags: &[Tag],
) -> Result<Option<OpenLibraryBook>> {
    let url = &make_search_url(title);

    info!("Querying OpenLibrary [{url}]");

    let response = ctx
        .network
        .download_json::<OpenLibrarySearchResponse>(url)
        .await?;

    let docs = &response.docs;

    if (tags.contains(&Tag::from_string(WARHAMMER_TAG))) {
        let books = docs
            .clone()
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

        if !books.is_empty() {
            return Ok(books.first().cloned());
        }
    }

    let first_author = author.split(',').next().unwrap().trim();

    let books = docs
        .clone()
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

impl BookService {
    pub fn new() -> Result<Self> {
        let file = FileService::archive(FILE_NAME.into());

        let data = file.read_json_or_default()?;

        Ok(Self {
            file,
            books: Arc::new(RwLock::new(data)),
        })
    }

    #[instrument(err, skip_all, fields(book.title=%title, book.author=&author))]
    pub async fn find_book(
        &self,
        ctx: &ServiceContext,
        title: &str,
        author: &str,
        tags: &[Tag],
    ) -> Result<Option<Book>> {
        let mut books = self.books.write().unwrap();

        if let Some(book) = books.get(title) {
            match book {
                Some(book) => return Ok(Some(book.clone())),
                None => {
                    warn!("Did not find cover for book [{title}]");
                    return Ok(None);
                }
            }
        }

        let book = query_book_api(ctx, title, author, tags).await?;

        if let Some(cover_i) = book.map(|doc| doc.cover_i).flatten() {
            debug!("Found cover [{cover_i}] for book [{title}]");
            let image_url = &format!("https://covers.openlibrary.org/b/id/{:?}-L.jpg", cover_i)
                .parse()
                .unwrap();

            let cdn_file = CdnFile::from_str(&format!("books/{}-cover-400.jpg", cover_i));

            let image = MediaService::image_from_url(
                ctx,
                image_url,
                &cdn_file,
                &format!("Cover for book {}", title),
                None,
                None,
            )
            .await?;

            let book = Book {
                title: title.to_string(),
                cover: image,
                id: BookID::OpenLibrary { id: cover_i },
            };

            books.insert(title.to_string(), Some(book.clone()));

            self.file.write_json(&books.clone())?;

            return Ok(Some(book));
        } else {
            warn!("Did not find cover for book [{title}]");
            books.insert(title.to_string(), None);

            self.file.write_json(&books.clone())?;
        }

        Ok(None)
    }
}
