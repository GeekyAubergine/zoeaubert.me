use crate::error::Error;

#[derive(Debug, thiserror::Error)]
pub enum SillyNamesError {
    #[error("Unable to read csv {0}")]
    UnableToReadCsv(csv::Error),

    #[error("Unable to parse csv {0}")]
    UnableToParseCsv(csv::Error),
}

impl SillyNamesError {
    pub fn unable_to_read_csv(error: csv::Error) -> Error {
        Error::SillyNames(Self::UnableToReadCsv(error))
    }

    pub fn unable_to_parse_csv(error: csv::Error) -> Error {
        Error::SillyNames(Self::UnableToParseCsv(error))
    }
}
