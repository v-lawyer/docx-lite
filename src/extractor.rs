use std::fs::File;
use std::io::{BufReader, Cursor};
use std::path::Path;

use crate::error::Result;
use crate::parser::DocxParser;
use crate::types::Document;

/// Extract text from a DOCX file at the given path
pub fn extract_text<P: AsRef<Path>>(path: P) -> Result<String> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    extract_text_from_reader(reader)
}

/// Extract text from DOCX bytes
pub fn extract_text_from_bytes(bytes: &[u8]) -> Result<String> {
    let cursor = Cursor::new(bytes);
    extract_text_from_reader(cursor)
}

/// Extract text from any reader containing DOCX data
pub fn extract_text_from_reader<R: std::io::Read + std::io::Seek>(reader: R) -> Result<String> {
    let document = parse_document(reader)?;
    Ok(document.extract_text())
}

/// Parse a DOCX file and return the structured Document
pub fn parse_document<R: std::io::Read + std::io::Seek>(reader: R) -> Result<Document> {
    let parser = DocxParser::new(reader)?;
    parser.parse()
}

/// Parse a DOCX file from a path and return the structured Document
pub fn parse_document_from_path<P: AsRef<Path>>(path: P) -> Result<Document> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    parse_document(reader)
}