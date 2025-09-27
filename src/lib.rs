//! # docx-lite
//!
//! A lightweight, fast DOCX text extraction library with minimal dependencies.
//!
//! ## Features
//! - Zero unsafe code
//! - Minimal dependencies (only zip, quick-xml, thiserror)
//! - Fast text extraction
//! - Support for paragraphs, tables, and basic formatting
//! - Graceful error handling for malformed documents
//!
//! ## Quick Start
//!
//! ```no_run
//! use docx_lite::extract_text;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let text = extract_text("document.docx")?;
//!     println!("{}", text);
//!     Ok(())
//! }
//! ```
//!
//! ## Advanced Usage
//!
//! ```no_run
//! use docx_lite::parse_document_from_path;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let doc = parse_document_from_path("document.docx")?;
//!
//!     for paragraph in &doc.paragraphs {
//!         println!("Paragraph: {}", paragraph.to_text());
//!     }
//!
//!     for table in &doc.tables {
//!         println!("Table with {} rows", table.rows.len());
//!     }
//!
//!     Ok(())
//! }
//! ```

pub mod error;
pub mod types;
pub mod parser;
pub mod extractor;

// Re-export main types and functions
pub use error::{DocxError, Result};
pub use types::{Document, Paragraph, Run, Table, TableRow, TableCell};
pub use extractor::{
    extract_text,
    extract_text_from_bytes,
    extract_text_from_reader,
    parse_document,
    parse_document_from_path,
};