use std::io::{Read, Seek};
use quick_xml::events::Event;
use quick_xml::Reader;
use zip::ZipArchive;

use crate::error::{DocxError, Result};
use crate::types::{
    Document, Paragraph, Run, Table, TableRow, TableCell,
    ListItem, ListType, HeaderFooter, Note, NoteType
};
use std::collections::HashMap;

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

        // Try to parse numbering definitions for lists
        if let Ok(numbering_xml) = self.read_file("word/numbering.xml") {
            let numbering_defs = self.parse_numbering(&numbering_xml)?;
            self.process_lists(&mut document, &numbering_defs);
        }

        // Try to parse headers and footers
        self.parse_headers_footers(&mut document)?;

        // Try to parse footnotes and endnotes
        if let Ok(footnotes_xml) = self.read_file("word/footnotes.xml") {
            self.parse_notes(&footnotes_xml, &mut document.footnotes, NoteType::Footnote)?;
        }

        if let Ok(endnotes_xml) = self.read_file("word/endnotes.xml") {
            self.parse_notes(&endnotes_xml, &mut document.endnotes, NoteType::Endnote)?;
        }

        Ok(document)
    }

    fn read_document_xml(&mut self) -> Result<String> {
        self.read_file("word/document.xml")
    }

    fn read_file(&mut self, path: &str) -> Result<String> {
        let mut file = self.archive
            .by_name(path)
            .map_err(|_| DocxError::FileNotFound(path.to_string()))?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    fn parse_document_xml(&self, xml: &str, document: &mut Document) -> Result<()> {
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(false);

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
                        b"w:numPr" => {
                            // Numbering properties (list item)
                            if let Some(ref mut para) = current_paragraph {
                                // Parse numId and ilvl from attributes or child elements
                                // For simplicity, we'll set defaults here
                                // In a full implementation, we'd parse the XML attributes
                                para.numbering_id = Some(1);
                                para.numbering_level = Some(0);
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

    fn parse_numbering(&self, xml: &str) -> Result<HashMap<i64, ListType>> {
        let mut numbering_defs = HashMap::new();
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(false);

        let mut buf = Vec::new();
        let mut current_num_id: Option<i64> = None;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    if e.name().as_ref() == b"w:num" {
                        // Try to get numId attribute
                        for attr in e.attributes() {
                            if let Ok(attr) = attr {
                                if attr.key.as_ref() == b"w:numId" {
                                    if let Ok(id_str) = std::str::from_utf8(&attr.value) {
                                        current_num_id = id_str.parse().ok();
                                    }
                                }
                            }
                        }
                    }
                }
                Ok(Event::End(ref e)) => {
                    if e.name().as_ref() == b"w:num" {
                        if let Some(id) = current_num_id {
                            // For simplicity, default to bullet
                            // In a full implementation, we'd check the abstractNum
                            numbering_defs.insert(id, ListType::Bullet);
                        }
                        current_num_id = None;
                    }
                }
                Ok(Event::Eof) => break,
                _ => {}
            }
            buf.clear();
        }

        Ok(numbering_defs)
    }

    fn process_lists(&self, document: &mut Document, numbering_defs: &HashMap<i64, ListType>) {
        for paragraph in &document.paragraphs {
            if let (Some(num_id), Some(level)) = (paragraph.numbering_id, paragraph.numbering_level) {
                let list_type = numbering_defs.get(&num_id)
                    .cloned()
                    .unwrap_or(ListType::Bullet);

                let list_item = ListItem {
                    level: level as u32,
                    list_type,
                    number: None, // Would need counter tracking for numbered lists
                    text: paragraph.to_text(),
                };

                document.lists.push(list_item);
            }
        }
    }

    fn parse_headers_footers(&mut self, document: &mut Document) -> Result<()> {
        // Try to parse headers
        for i in 1..=3 {
            let header_path = format!("word/header{}.xml", i);
            if let Ok(header_xml) = self.read_file(&header_path) {
                let mut header = HeaderFooter::default();
                self.parse_header_footer_content(&header_xml, &mut header)?;
                document.headers.push(header);
            }

            let footer_path = format!("word/footer{}.xml", i);
            if let Ok(footer_xml) = self.read_file(&footer_path) {
                let mut footer = HeaderFooter::default();
                self.parse_header_footer_content(&footer_xml, &mut footer)?;
                document.footers.push(footer);
            }
        }

        Ok(())
    }

    fn parse_header_footer_content(&self, xml: &str, header_footer: &mut HeaderFooter) -> Result<()> {
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(false);

        let mut buf = Vec::new();
        let mut current_paragraph: Option<Paragraph> = None;
        let mut current_run: Option<Run> = None;
        let mut in_text = false;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name().as_ref() {
                        b"w:p" => current_paragraph = Some(Paragraph::new()),
                        b"w:r" => current_run = Some(Run::default()),
                        b"w:t" => in_text = true,
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
                        b"w:t" => in_text = false,
                        b"w:r" => {
                            if let Some(run) = current_run.take() {
                                if let Some(ref mut para) = current_paragraph {
                                    para.add_run(run);
                                }
                            }
                        }
                        b"w:p" => {
                            if let Some(para) = current_paragraph.take() {
                                header_footer.paragraphs.push(para);
                            }
                        }
                        _ => {}
                    }
                }
                Ok(Event::Eof) => break,
                _ => {}
            }
            buf.clear();
        }

        Ok(())
    }

    fn parse_notes(&self, xml: &str, notes: &mut Vec<Note>, note_type: NoteType) -> Result<()> {
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(false);

        let mut buf = Vec::new();
        let mut current_note: Option<Note> = None;
        let mut current_paragraph: Option<Paragraph> = None;
        let mut current_run: Option<Run> = None;
        let mut in_text = false;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name().as_ref() {
                        b"w:footnote" | b"w:endnote" => {
                            let mut id = String::new();
                            for attr in e.attributes() {
                                if let Ok(attr) = attr {
                                    if attr.key.as_ref() == b"w:id" {
                                        id = String::from_utf8_lossy(&attr.value).to_string();
                                    }
                                }
                            }
                            current_note = Some(Note {
                                id,
                                note_type: note_type.clone(),
                                paragraphs: Vec::new(),
                            });
                        }
                        b"w:p" => current_paragraph = Some(Paragraph::new()),
                        b"w:r" => current_run = Some(Run::default()),
                        b"w:t" => in_text = true,
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
                        b"w:t" => in_text = false,
                        b"w:r" => {
                            if let Some(run) = current_run.take() {
                                if let Some(ref mut para) = current_paragraph {
                                    para.add_run(run);
                                }
                            }
                        }
                        b"w:p" => {
                            if let Some(para) = current_paragraph.take() {
                                if let Some(ref mut note) = current_note {
                                    note.paragraphs.push(para);
                                }
                            }
                        }
                        b"w:footnote" | b"w:endnote" => {
                            if let Some(note) = current_note.take() {
                                // Skip separator and continuation notes (ids -1 and 0)
                                if note.id != "-1" && note.id != "0" {
                                    notes.push(note);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Ok(Event::Eof) => break,
                _ => {}
            }
            buf.clear();
        }

        Ok(())
    }
}