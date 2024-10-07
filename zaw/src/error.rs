#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("File System Error: {0}")]
    FileSystemError(#[from] FileSystemError),

    #[error("Markdown Error: {0}")]
    MarkdownError(#[from] MarkdownError),
}


#[derive(Debug, thiserror::Error)]
pub enum FileSystemError {
    #[error("Unable to write to file: {0}")]
    WriteError(std::io::Error),
}

impl FileSystemError {
    pub fn write_error(error: std::io::Error) -> Error {
        Error::FileSystemError(Self::WriteError(error))
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
