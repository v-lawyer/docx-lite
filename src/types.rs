#[derive(Debug, Clone, Default)]
pub struct Document {
    pub paragraphs: Vec<Paragraph>,
    pub tables: Vec<Table>,
    pub lists: Vec<ListItem>,
    pub headers: Vec<HeaderFooter>,
    pub footers: Vec<HeaderFooter>,
    pub footnotes: Vec<Note>,
    pub endnotes: Vec<Note>,
}

#[derive(Debug, Clone, Default)]
pub struct Paragraph {
    pub runs: Vec<Run>,
    pub style: Option<String>,
    pub numbering_id: Option<i64>,
    pub numbering_level: Option<i64>,
}

#[derive(Debug, Clone, Default)]
pub struct Run {
    pub text: String,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Table {
    pub rows: Vec<TableRow>,
}

#[derive(Debug, Clone, Default)]
pub struct TableRow {
    pub cells: Vec<TableCell>,
}

#[derive(Debug, Clone, Default)]
pub struct TableCell {
    pub paragraphs: Vec<Paragraph>,
}

// New types for v0.2.0

#[derive(Debug, Clone)]
pub struct ListItem {
    pub level: u32,
    pub list_type: ListType,
    pub number: Option<String>,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ListType {
    Bullet,
    Numbered,
}

#[derive(Debug, Clone, Default)]
pub struct HeaderFooter {
    pub paragraphs: Vec<Paragraph>,
    pub tables: Vec<Table>,
    pub header_type: HeaderFooterType,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum HeaderFooterType {
    #[default]
    Default,
    First,
    Even,
    Odd,
}

#[derive(Debug, Clone)]
pub struct Note {
    pub id: String,
    pub note_type: NoteType,
    pub paragraphs: Vec<Paragraph>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NoteType {
    Footnote,
    Endnote,
}

#[derive(Debug, Clone, Default)]
pub struct ExtractOptions {
    pub include_headers: bool,
    pub include_footers: bool,
    pub include_footnotes: bool,
    pub include_endnotes: bool,
    pub include_list_markers: bool,
}

impl ExtractOptions {
    pub fn all() -> Self {
        Self {
            include_headers: true,
            include_footers: true,
            include_footnotes: true,
            include_endnotes: true,
            include_list_markers: true,
        }
    }

    pub fn none() -> Self {
        Self::default()
    }
}

impl Document {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn extract_text(&self) -> String {
        self.extract_text_with_options(&ExtractOptions::none())
    }

    pub fn extract_text_with_options(&self, options: &ExtractOptions) -> String {
        let mut text = String::new();

        // Headers
        if options.include_headers && !self.headers.is_empty() {
            text.push_str("--- Headers ---\n");
            for header in &self.headers {
                text.push_str(&header.extract_text());
                text.push('\n');
            }
            text.push('\n');
        }

        // Main content - paragraphs and lists interspersed
        let mut list_index = 0;
        for paragraph in &self.paragraphs {
            // Check if this paragraph is a list item
            if let (Some(_num_id), Some(level)) = (paragraph.numbering_id, paragraph.numbering_level) {
                // This is a list item
                if options.include_list_markers && list_index < self.lists.len() {
                    let list_item = &self.lists[list_index];
                    let indent = "  ".repeat(level as usize);
                    let marker = match list_item.list_type {
                        ListType::Bullet => "• ".to_string(),
                        ListType::Numbered => {
                            if let Some(ref num) = list_item.number {
                                format!("{}. ", num)
                            } else {
                                "• ".to_string()
                            }
                        }
                    };
                    text.push_str(&format!("{}{}{}\n", indent, marker, list_item.text));
                    list_index += 1;
                } else {
                    // Include as regular paragraph without marker
                    let para_text = paragraph.to_text();
                    if !para_text.is_empty() {
                        text.push_str(&para_text);
                        text.push('\n');
                    }
                }
            } else {
                // Regular paragraph
                let para_text = paragraph.to_text();
                if !para_text.is_empty() {
                    text.push_str(&para_text);
                    text.push('\n');
                }
            }
        }

        // Tables
        for table in &self.tables {
            for row in &table.rows {
                for cell in &row.cells {
                    for paragraph in &cell.paragraphs {
                        let para_text = paragraph.to_text();
                        if !para_text.is_empty() {
                            text.push_str(&para_text);
                            text.push('\t');
                        }
                    }
                }
                text.push('\n');
            }
            text.push('\n');
        }

        // Footnotes
        if options.include_footnotes && !self.footnotes.is_empty() {
            text.push_str("\n--- Footnotes ---\n");
            for (i, note) in self.footnotes.iter().enumerate() {
                text.push_str(&format!("[{}] ", i + 1));
                for para in &note.paragraphs {
                    text.push_str(&para.to_text());
                }
                text.push('\n');
            }
        }

        // Endnotes
        if options.include_endnotes && !self.endnotes.is_empty() {
            text.push_str("\n--- Endnotes ---\n");
            for (i, note) in self.endnotes.iter().enumerate() {
                text.push_str(&format!("[{}] ", i + 1));
                for para in &note.paragraphs {
                    text.push_str(&para.to_text());
                }
                text.push('\n');
            }
        }

        // Footers
        if options.include_footers && !self.footers.is_empty() {
            text.push_str("\n--- Footers ---\n");
            for footer in &self.footers {
                text.push_str(&footer.extract_text());
                text.push('\n');
            }
        }

        text
    }
}

impl Paragraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_text(&self) -> String {
        self.runs.iter()
            .map(|run| run.text.as_str())
            .collect::<Vec<_>>()
            .join("")
    }

    pub fn add_run(&mut self, run: Run) {
        self.runs.push(run);
    }
}

impl Run {
    pub fn new(text: String) -> Self {
        Self {
            text,
            ..Default::default()
        }
    }
}

impl Table {
    pub fn new() -> Self {
        Self::default()
    }
}

impl HeaderFooter {
    pub fn extract_text(&self) -> String {
        let mut text = String::new();

        for paragraph in &self.paragraphs {
            let para_text = paragraph.to_text();
            if !para_text.is_empty() {
                text.push_str(&para_text);
                text.push('\n');
            }
        }

        for table in &self.tables {
            for row in &table.rows {
                for cell in &row.cells {
                    for paragraph in &cell.paragraphs {
                        let para_text = paragraph.to_text();
                        if !para_text.is_empty() {
                            text.push_str(&para_text);
                            text.push('\t');
                        }
                    }
                }
                text.push('\n');
            }
        }

        text
    }
}