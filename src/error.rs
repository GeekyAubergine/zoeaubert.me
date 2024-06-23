use crate::{
    application::events::Event,
    infrastructure::{bus::job_runner::Job, cdn::CdnPath},
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Couldn't read dir {0}")]
    ReadDir(std::io::Error),
    #[error("Couldn't read config file {0}")]
    ReadConfigFile(std::io::Error),
    #[error("Couldn't parse config file {0}")]
    ParseConfigFile(serde_json::Error),
    #[error("Couldn't make folder {0}")]
    MakeFolder(std::io::Error),
    #[error("HttpReqwest {0}")]
    HttpReqwest(reqwest::Error),
    #[error("CND Upload {0}")]
    CdnUpload(reqwest::Error),
    #[error("CND Download {0}")]
    CdnDownload(reqwest::Error),
    #[error("Unable to parse cdn response {0}")]
    ParseCdnResponse(serde_json::Error),
    #[error("CDN invalid path {0}")]
    CdnInvalidPath(String),
    #[error("CDN file not found {0}")]
    CdnFileNotFound(String),
    #[error("CDN unable to upload file {0} to {1}")]
    CdnUnableToUploadFile(String, String),
    #[error("File not found {0}")]
    FileNotFound(String),
    #[error("File system unreadable {0}")]
    FileSystemUnreadable(std::io::Error),
    #[error("File system unwritable {0}")]
    FileSystemUnwritable(std::io::Error),
    #[error("Dispatch job {0}")]
    DispatchJob(tokio::sync::mpsc::error::SendError<Box<dyn Job>>),
    #[error("Dispatch event {0}")]
    DispatchEvent(tokio::sync::mpsc::error::SendError<Event>),
    #[error("Couldn't serialize archive {0}")]
    SerializeArchive(serde_json::Error),
    #[error("Couldn't deserialize archive {0}")]
    DeserializeArchive(serde_json::Error),
    #[error("Couldn't parse blog front matter {0}")]
    ParseBlogFrontMatter(serde_yaml::Error),
    #[error("Couldn't parse micro post front matter {0}")]
    ParseMicroPostFrontMatter(serde_yaml::Error),
    #[error("Couldn't parse blog post {0}")]
    ParseBlogPost(String),
    #[error("Couldn't parse micro post {0}")]
    ParseMicroPost(String),
    #[error("Could not parse date {0}")]
    ParseDate(String),
    #[error("Could not find langauge for code block")]
    CouldNotFindLangaugeForCodeBlock(),
    #[error("Could not find body for code block")]
    CouldNotFindBodyForCodeBlock(),
    #[error("Image size {0}")]
    ImageSize(String),
}
