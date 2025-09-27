use thiserror::Error;

#[derive(Error, Debug)]
pub enum DocxError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("ZIP error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("XML parsing error: {0}")]
    Xml(#[from] quick_xml::Error),

    #[error("UTF-8 decoding error: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    #[error("Document structure error: {0}")]
    Structure(String),

    #[error("Required file not found in DOCX: {0}")]
    FileNotFound(String),

    #[error("Unsupported DOCX format: {0}")]
    UnsupportedFormat(String),
}

pub type Result<T> = std::result::Result<T, DocxError>;