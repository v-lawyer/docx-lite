use std::io::{Read, Seek};
use quick_xml::events::Event;
use quick_xml::Reader;
use zip::ZipArchive;

use crate::error::{DocxError, Result};
use crate::types::{Document, Paragraph, Run, Table, TableRow, TableCell};

pub struct DocxParser<R: Read + Seek> {
    archive: ZipArchive<R>,
}

impl<R: Read + Seek> DocxParser<R> {
    pub fn new(reader: R) -> Result<Self> {
        let archive = ZipArchive::new(reader)?;
        Ok(Self { archive })
    }

    pub fn parse(mut self) -> Result<Document> {
        let mut document = Document::new();

        // Extract main document content
        let document_xml = self.read_document_xml()?;
        self.parse_document_xml(&document_xml, &mut document)?;

        Ok(document)
    }

    fn read_document_xml(&mut self) -> Result<String> {
        let mut file = self.archive
            .by_name("word/document.xml")
            .map_err(|_| DocxError::FileNotFound("word/document.xml".to_string()))?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    fn parse_document_xml(&self, xml: &str, document: &mut Document) -> Result<()> {
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);

        let mut buf = Vec::new();
        let mut current_paragraph: Option<Paragraph> = None;
        let mut current_run: Option<Run> = None;
        let mut current_table: Option<Table> = None;
        let mut current_row: Option<TableRow> = None;
        let mut current_cell: Option<TableCell> = None;
        let mut in_text = false;
        let mut in_table = false;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name().as_ref() {
                        b"w:p" => {
                            // Start of a paragraph
                            if in_table {
                                // Paragraph inside a table cell
                                if current_cell.is_none() {
                                    current_cell = Some(TableCell::default());
                                }
                            } else {
                                current_paragraph = Some(Paragraph::new());
                            }
                        }
                        b"w:r" => {
                            // Start of a run
                            current_run = Some(Run::default());
                        }
                        b"w:t" => {
                            // Text element
                            in_text = true;
                        }
                        b"w:tbl" => {
                            // Start of a table
                            in_table = true;
                            current_table = Some(Table::new());
                        }
                        b"w:tr" => {
                            // Table row
                            current_row = Some(TableRow::default());
                        }
                        b"w:tc" => {
                            // Table cell
                            current_cell = Some(TableCell::default());
                        }
                        b"w:b" => {
                            // Bold formatting
                            if let Some(ref mut run) = current_run {
                                run.bold = true;
                            }
                        }
                        b"w:i" => {
                            // Italic formatting
                            if let Some(ref mut run) = current_run {
                                run.italic = true;
                            }
                        }
                        b"w:u" => {
                            // Underline formatting
                            if let Some(ref mut run) = current_run {
                                run.underline = true;
                            }
                        }
                        _ => {}
                    }
                }
                Ok(Event::Text(e)) => {
                    if in_text {
                        if let Some(ref mut run) = current_run {
                            let text = e.unescape()?.into_owned();
                            run.text.push_str(&text);
                        }
                    }
                }
                Ok(Event::End(ref e)) => {
                    match e.name().as_ref() {
                        b"w:t" => {
                            in_text = false;
                        }
                        b"w:r" => {
                            // End of a run
                            if let Some(run) = current_run.take() {
                                if in_table {
                                    // Add run to table cell paragraph
                                    if let Some(ref mut cell) = current_cell {
                                        if cell.paragraphs.is_empty() {
                                            cell.paragraphs.push(Paragraph::new());
                                        }
                                        if let Some(para) = cell.paragraphs.last_mut() {
                                            para.add_run(run);
                                        }
                                    }
                                } else if let Some(ref mut para) = current_paragraph {
                                    para.add_run(run);
                                }
                            }
                        }
                        b"w:p" => {
                            // End of a paragraph
                            if in_table {
                                // Paragraph inside table cell already handled
                            } else if let Some(para) = current_paragraph.take() {
                                document.paragraphs.push(para);
                            }
                        }
                        b"w:tc" => {
                            // End of table cell
                            if let Some(cell) = current_cell.take() {
                                if let Some(ref mut row) = current_row {
                                    row.cells.push(cell);
                                }
                            }
                        }
                        b"w:tr" => {
                            // End of table row
                            if let Some(row) = current_row.take() {
                                if let Some(ref mut table) = current_table {
                                    table.rows.push(row);
                                }
                            }
                        }
                        b"w:tbl" => {
                            // End of table
                            in_table = false;
                            if let Some(table) = current_table.take() {
                                document.tables.push(table);
                            }
                        }
                        _ => {}
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(e.into()),
                _ => {}
            }
            buf.clear();
        }

        Ok(())
    }
}