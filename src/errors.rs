use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum PdfExtractorError {
    InvalidPages(String),
}

impl Display for PdfExtractorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PdfExtractorError::InvalidPages(e) => write!(f, "Argument error: {}", e)
        }
    }
}

impl Error for PdfExtractorError {
}