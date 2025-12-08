use std::path::PathBuf;

use tracing::Value;

use crate::{domain::models::slug::Slug, services::file_service::ContentFile};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("File System Error: {0}")]
    FileSystemError(#[from] FileSystemError),

    #[error("Markdown Error: {0}")]
    MarkdownError(#[from] MarkdownError),

    #[error("JSON Error: {0}")]
    JsonError(#[from] JsonError),

    #[error("CSV Error: {0}")]
    CsvError(#[from] CsvError),

    #[error("Yaml Error: {0}")]
    YamlError(#[from] YamlError),

    #[error("Blog post error: {0}")]
    BlogPostError(#[from] BlogPostError),

    #[error("Date parse error: {0}")]
    DateParseError(#[from] DateParseError),

    #[error("Template error: {0}")]
    TemplateError(#[from] TemplateError),

    #[error("Network error: {0}")]
    NetworkError(#[from] NetworkError),

    #[error("Micro post error: {0}")]
    MicroPostError(#[from] MicroPostError),

    #[error("Cdn error: {0}")]
    CdnError(#[from] CdnError),

    #[error("Game error: {0}")]
    GameError(#[from] GameError),

    #[error("Image error: {0}")]
    ImageError(#[from] ImageError),

    #[error("Movie error: {0}")]
    MovieError(#[from] MovieError),

    #[error("Tv shows error: {0}")]
    TvShowsError(#[from] TvShowsError),

    #[error("Album error: {0}")]
    AlbumError(#[from] AlbumError),

    #[error("Book error: {0}")]
    BookError(#[from] BookError),

    #[error("Site Build Error: {0}")]
    SiteBuildError(#[from] SiteBuildError),

    #[error("Inquire Error: {0}")]
    InquireError(#[from] inquire::error::InquireError),

    #[error("Unknown")]
    Unknown(),
}

#[derive(Debug, thiserror::Error)]
pub enum FileSystemError {
    #[error("Unable to write to file: {0}")]
    WriteError(std::io::Error),

    #[error("Unable to read file: {0}")]
    ReadError(std::io::Error),

    #[error("Unable to create directory: {0}")]
    CreateDirError(std::io::Error),

    #[error("Unable to read directory: {0}")]
    ReadDirError(std::io::Error),

    #[error("Unable to copy file: {0}")]
    CopyFileError(std::io::Error),

    #[error("Unable to delete directory: {0}")]
    DeleteDirError(std::io::Error),

    #[error("Unable to copy directory: {0}")]
    CopyDirError(std::io::Error),

    #[error("Unable to delete file: {0}")]
    DeleteFileError(std::io::Error),

    #[error("Path is not representable as URL: {0}")]
    PathIsNotUrl(url::ParseError),

    #[error("Invalid path: [{0}]")]
    InvalidPath(PathBuf),
}

impl FileSystemError {
    pub fn write_error(error: std::io::Error) -> Error {
        Error::FileSystemError(Self::WriteError(error))
    }

    pub fn read_error(error: std::io::Error) -> Error {
        Error::FileSystemError(Self::ReadError(error))
    }

    pub fn create_dir_error(error: std::io::Error) -> Error {
        Error::FileSystemError(Self::CreateDirError(error))
    }

    pub fn read_dir_error(error: std::io::Error) -> Error {
        Error::FileSystemError(Self::ReadDirError(error))
    }

    pub fn copy_file_error(error: std::io::Error) -> Error {
        Error::FileSystemError(Self::CopyFileError(error))
    }

    pub fn delete_dir_error(error: std::io::Error) -> Error {
        Error::FileSystemError(Self::DeleteDirError(error))
    }

    pub fn copy_dir_error(error: std::io::Error) -> Error {
        Error::FileSystemError(Self::CopyDirError(error))
    }

    pub fn delete_file_error(error: std::io::Error) -> Error {
        Error::FileSystemError(Self::DeleteFileError(error))
    }

    pub fn path_is_not_url(error: url::ParseError) -> Error {
        Error::FileSystemError(Self::PathIsNotUrl(error))
    }

    pub fn invalid_path(path: PathBuf) -> Error {
        Error::FileSystemError(Self::InvalidPath(path))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MarkdownError {
    #[error("Could not find langauge for code block")]
    CouldNotFindLangaugeForCodeBlock(),

    #[error("Could not find body for code block")]
    CouldNotFindBodyForCodeBlock(),
}

impl MarkdownError {
    pub fn could_not_find_language_for_code_block() -> Error {
        Error::MarkdownError(Self::CouldNotFindLangaugeForCodeBlock())
    }

    pub fn could_not_find_body_for_code_block() -> Error {
        Error::MarkdownError(Self::CouldNotFindBodyForCodeBlock())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum JsonError {
    #[error("Unable to parse JSON: {0}")]
    ParseError(serde_json::Error),

    #[error("Unable to stringify to JSON: {0}")]
    StringifyError(serde_json::Error),
}

impl JsonError {
    pub fn parse_error(error: serde_json::Error) -> Error {
        Error::JsonError(Self::ParseError(error))
    }

    pub fn stringify_error(error: serde_json::Error) -> Error {
        Error::JsonError(Self::StringifyError(error))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CsvError {
    #[error("Unable to read csv {0}")]
    ReadError(csv::Error),

    #[error("Unable to parse csv {0}")]
    ParseError(csv::Error),
}

impl CsvError {
    pub fn read_error(error: csv::Error) -> Error {
        Error::CsvError(Self::ReadError(error))
    }

    pub fn parse_error(error: csv::Error) -> Error {
        Error::CsvError(Self::ParseError(error))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum YamlError {
    #[error("Unable to parse yaml: {0}")]
    ParseError(serde_yaml::Error),

    #[error("Unable to stringify to yaml: {0}")]
    StringifyError(serde_yaml::Error),
}

impl YamlError {
    pub fn parse_error(error: serde_yaml::Error) -> Error {
        Error::YamlError(Self::ParseError(error))
    }

    pub fn stringify_error(error: serde_yaml::Error) -> Error {
        Error::YamlError(Self::StringifyError(error))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BlogPostError {
    #[error("Unable to parse front matter: {0}")]
    UnparsableFrontMatter(serde_yaml::Error),

    #[error("Unable to parse blog post")]
    UnparsableBlogPost(),
}

impl BlogPostError {
    pub fn unparsable_front_matter(error: serde_yaml::Error) -> Error {
        Error::BlogPostError(Self::UnparsableFrontMatter(error))
    }

    pub fn unparsable_blog_post() -> Error {
        Error::BlogPostError(Self::UnparsableBlogPost())
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum DateParseError {
    #[error("Unable to parse date: {0}")]
    UnableToParseDate(String),
}

impl DateParseError {
    pub fn unable_to_parse_date(date: String) -> Error {
        Error::DateParseError(Self::UnableToParseDate(date))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TemplateError {
    #[error("Unable to render template: {0}")]
    RenderError(askama::Error),
}

impl TemplateError {
    pub fn render_error(error: askama::Error) -> Error {
        Error::TemplateError(Self::RenderError(error))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum NetworkError {
    #[error("Unable to fetch url: {0}")]
    FetchError(reqwest::Error),

    #[error("Unable to send to url: {0}")]
    SendError(reqwest::Error),
}

impl NetworkError {
    pub fn fetch_error(error: reqwest::Error) -> Error {
        Error::NetworkError(Self::FetchError(error))
    }

    pub fn send_error(error: reqwest::Error) -> Error {
        Error::NetworkError(Self::SendError(error))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MicroPostError {
    #[error("Unable to parse front matter: {0}")]
    UnparsableFrontMatter(serde_yaml::Error),

    #[error("Post has no content {0}")]
    PostHasNoContent(ContentFile),

    #[error("Post has not front matter {0}")]
    PostHasNoFrontMatter(ContentFile),

    #[error("Post has invalid file path {0}")]
    InvalidFilePath(ContentFile),

    #[error("Post has invalid file name {0}")]
    InvalidFileName(ContentFile),
}

impl MicroPostError {
    pub fn unable_to_parse_front_matter(error: serde_yaml::Error) -> Error {
        Error::MicroPostError(Self::UnparsableFrontMatter(error))
    }

    pub fn no_content(post: ContentFile) -> Error {
        Error::MicroPostError(Self::PostHasNoContent(post))
    }

    pub fn no_front_matter(post: ContentFile) -> Error {
        Error::MicroPostError(Self::PostHasNoFrontMatter(post))
    }

    pub fn invalid_file_path(post: ContentFile) -> Error {
        Error::MicroPostError(Self::InvalidFilePath(post))
    }

    pub fn invalid_file_name(post: ContentFile) -> Error {
        Error::MicroPostError(Self::InvalidFileName(post))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CdnError {
    #[error("Unable to upload file: {0}")]
    UploadError(reqwest::Error),

    #[error("Bad status from CDN: {0}")]
    BadStatus(u16),
}

impl CdnError {
    pub fn upload_error(error: reqwest::Error) -> Error {
        Error::CdnError(Self::UploadError(error))
    }

    pub fn base_status(status: u16) -> Error {
        Error::CdnError(Self::BadStatus(status))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GameError {
    #[error("Unable to find game: {0}")]
    NotFound(u32),
}

impl GameError {
    pub fn not_found(id: u32) -> Error {
        Error::GameError(Self::NotFound(id))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ImageError {
    #[error("Unable to get image size: {0}")]
    SizeError(imagesize::ImageError),

    #[error("Unable to parse image format {0}")]
    ParseError(std::io::Error),

    #[error("Unable to decode image {0}")]
    DecodeError(image::ImageError),

    #[error("Unable to encode image {0}")]
    EncodeError(image::ImageError),
}

impl ImageError {
    pub fn size_error(error: imagesize::ImageError) -> Error {
        Error::ImageError(Self::SizeError(error))
    }

    pub fn parse_format_error(error: std::io::Error) -> Error {
        Error::ImageError(Self::ParseError(error))
    }

    pub fn decode_error(error: image::ImageError) -> Error {
        Error::ImageError(Self::DecodeError(error))
    }

    pub fn encode_error(error: image::ImageError) -> Error {
        Error::ImageError(Self::EncodeError(error))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MovieError {
    #[error("Unable to parse movie: {0}")]
    UnableToParseMovie(String),

    #[error("Unable to parse and find movie title: {0}")]
    UnableToParseAndFindMovieTitle(String),

    #[error("Unable to parse and find movie year: {0}")]
    UnableToParseAndFindMovieYear(String),

    #[error("Unable to parse and find movie review: {0}")]
    UnableToParseAndFindMovieReview(String),

    #[error("Unable to parse and find movie score: {0}")]
    UnableToParseAndFindMovieScore(String),

    #[error("Movie not found")]
    MovieNotFound(String),

    #[error("Movie has no poster {0}")]
    MovieHasNoPoster(u32),
}

impl MovieError {
    pub fn unable_to_parse_movie(error: String) -> Error {
        Error::MovieError(Self::UnableToParseMovie(error))
    }

    pub fn unable_to_parse_and_find_movie_title(error: String) -> Error {
        Error::MovieError(Self::UnableToParseAndFindMovieTitle(error))
    }

    pub fn unable_to_parse_and_find_movie_year(error: String) -> Error {
        Error::MovieError(Self::UnableToParseAndFindMovieYear(error))
    }

    pub fn unable_to_parse_and_find_movie_review(error: String) -> Error {
        Error::MovieError(Self::UnableToParseAndFindMovieReview(error))
    }

    pub fn unable_to_parse_and_find_movie_score(error: String) -> Error {
        Error::MovieError(Self::UnableToParseAndFindMovieScore(error))
    }

    pub fn movie_not_found(error: String) -> Error {
        Error::MovieError(Self::MovieNotFound(error))
    }

    pub fn movie_has_no_poster(id: u32) -> Error {
        Error::MovieError(Self::MovieHasNoPoster(id))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TvShowsError {
    #[error("Unable to parse tv show: {0}")]
    UnableToParseTvShow(String),

    #[error("Unable to parse and find tv show title: {0}")]
    UnableToParseAndFindTvShowTitle(String),

    #[error("Unable to parse and find tv show season: {0}")]
    UnableToParseAndFindTvShowSeason(String),

    #[error("Unable to parse and find tv show review: {0}")]
    UnableToParseAndFindTvShowReview(String),

    #[error("Unable to parse and find tv show score: {0}")]
    UnableToParseAndFindTvShowScore(String),

    #[error("Tv show not found")]
    TvShowNotFound(String),

    #[error("Tv show has no poster {0}")]
    TvShowHasNoPoster(u32),
}

impl TvShowsError {
    pub fn unable_to_parse_tv_show(error: String) -> Error {
        Error::TvShowsError(Self::UnableToParseTvShow(error))
    }

    pub fn unable_to_parse_and_find_tv_show_title(error: String) -> Error {
        Error::TvShowsError(Self::UnableToParseAndFindTvShowTitle(error))
    }

    pub fn unable_to_parse_and_find_tv_show_season(error: String) -> Error {
        Error::TvShowsError(Self::UnableToParseAndFindTvShowSeason(error))
    }

    pub fn unable_to_parse_and_find_tv_show_review(error: String) -> Error {
        Error::TvShowsError(Self::UnableToParseAndFindTvShowReview(error))
    }

    pub fn unable_to_parse_and_find_tv_show_score(error: String) -> Error {
        Error::TvShowsError(Self::UnableToParseAndFindTvShowScore(error))
    }

    pub fn tv_show_not_found(error: String) -> Error {
        Error::TvShowsError(Self::TvShowNotFound(error))
    }

    pub fn tv_show_has_no_poster(id: u32) -> Error {
        Error::TvShowsError(Self::TvShowHasNoPoster(id))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AlbumError {
    #[error("Invalid file name {0}")]
    InvalidFileName(ContentFile),
}

impl AlbumError {
    pub fn invalid_file_name(file_name: ContentFile) -> Error {
        Error::AlbumError(Self::InvalidFileName(file_name))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BookError {
    #[error("Unable to parse book: {0}")]
    UnableToParseBook(String),

    #[error("Unable to find book title: {0}")]
    UnableToParseAndFindBookTitle(String),
}

impl BookError {
    pub fn unable_to_parse_book(error: String) -> Error {
        Error::BookError(Self::UnableToParseBook(error))
    }

    pub fn unable_to_parse_and_find_book_title(error: String) -> Error {
        Error::BookError(Self::UnableToParseAndFindBookTitle(error))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SiteBuildError {
    #[error("Unable to compile Tailwind CSS")]
    UnableToCompileTailwindCss(),

    #[error("Unable to compile Lightning CSS")]
    UnableToCompileLightningCss(),

    #[error("Unable to create _assets/css directory")]
    UnableToCreateAssetsCssDirectory(),
}

impl SiteBuildError {
    pub fn unable_to_compile_tailwind_css() -> Error {
        Error::SiteBuildError(Self::UnableToCompileTailwindCss())
    }

    pub fn unable_to_compile_lightning_css() -> Error {
        Error::SiteBuildError(Self::UnableToCompileLightningCss())
    }

    pub fn unable_to_create_assets_css_directory() -> Error {
        Error::SiteBuildError(Self::UnableToCreateAssetsCssDirectory())
    }
}
