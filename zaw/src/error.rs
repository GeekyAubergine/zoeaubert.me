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
}

impl NetworkError {
    pub fn fetch_error(error: reqwest::Error) -> Error {
        Error::NetworkError(Self::FetchError(error))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MicroPostError {
    #[error("Unable to parse front matter: {0}")]
    UnparsableFrontMatter(serde_yaml::Error),

    #[error("Post has no content {0}")]
    PostHasNoContent(String),

    #[error("Post has not front matter {0}")]
    PostHasNoFrontMatter(String),

    #[error("Post has invalid file path {0}")]
    InvalidFilePath(String),

    #[error("Post has invalid file name {0}")]
    InvalidFileName(String),
}

impl MicroPostError {
    pub fn unable_to_parse_front_matter(error: serde_yaml::Error) -> Error {
        Error::MicroPostError(Self::UnparsableFrontMatter(error))
    }

    pub fn no_content(post: String) -> Error {
        Error::MicroPostError(Self::PostHasNoContent(post))
    }

    pub fn no_front_matter(post: String) -> Error {
        Error::MicroPostError(Self::PostHasNoFrontMatter(post))
    }

    pub fn invalid_file_path(post: String) -> Error {
        Error::MicroPostError(Self::InvalidFilePath(post))
    }

    pub fn invalid_file_name(post: String) -> Error {
        Error::MicroPostError(Self::InvalidFileName(post))
    }
}
