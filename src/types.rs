
#[derive(Debug, Clone, Default)]
pub struct Document {
    pub paragraphs: Vec<Paragraph>,
    pub tables: Vec<Table>,
}

#[derive(Debug, Clone, Default)]
pub struct Paragraph {
    pub runs: Vec<Run>,
    pub style: Option<String>,
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

impl Document {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn extract_text(&self) -> String {
        let mut text = String::new();

        // Extract text from paragraphs
        for paragraph in &self.paragraphs {
            let para_text = paragraph.to_text();
            if !para_text.is_empty() {
                text.push_str(&para_text);
                text.push('\n');
            }
        }

        // Extract text from tables
        for table in &self.tables {
            for row in &table.rows {
                for cell in &row.cells {
                    for paragraph in &cell.paragraphs {
                        let para_text = paragraph.to_text();
                        if !para_text.is_empty() {
                            text.push_str(&para_text);
                            text.push('\t'); // Separate cells with tabs
                        }
                    }
                }
                text.push('\n'); // New line after each row
            }
            text.push('\n'); // Extra line after table
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