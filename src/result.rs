use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    CError(&'static str),
    ImageLoadError(String),
    ImageOperationError(String),
    ImageSaveError(String),
    ImageMetadataError(String),
    UnknownError(&'static str),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CError(s) => write!(f, "C error: {}", s),
            Error::ImageLoadError(s) => write!(f, "Failed to load image: {}", s),
            Error::ImageOperationError(s) => write!(f, "Failed to process image: {}", s),
            Error::ImageSaveError(s) => write!(f, "Failed to save image: {}", s),
            Error::ImageMetadataError(s) => write!(f, "Failed to retrieve image metadata: {}", s),
            Error::UnknownError(s) => write!(f, "Unknown error: {}", s),
        }
    }
}