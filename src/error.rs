use aws_sdk_s3::error::SdkError;

use crate::{application::events::Event, infrastructure::bus::job_runner::Job};

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
    #[error("CND Head check {0}")]
    CdnHeadCheck(
        SdkError<
            aws_sdk_s3::operation::head_object::HeadObjectError,
            aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    ),
    #[error("CND Upload {0}")]
    CdnUpload(aws_sdk_s3::error::SdkError<aws_sdk_s3::operation::put_object::PutObjectError>),
    #[error("File system unreadable {0}")]
    FileSystemUnreadable(std::io::Error),
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
    #[error("Couldn't parse blog post {0}")]
    ParseBlogPost(String),
    #[error("Could not parse date {0}")]
    ParseDate(String),
    #[error("Could not find langauge for code block")]
    CouldNotFindLangaugeForCodeBlock(),
    #[error("Could not find body for code block")]
    CouldNotFindBodyForCodeBlock(),
}
