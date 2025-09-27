use docx_lite::{extract_text, parse_document_from_path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check if a file path was provided
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <docx_file>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    println!("Extracting text from: {}\n", file_path);
    println!("{}", "=".repeat(50));

    // Simple text extraction
    let text = extract_text(file_path)?;
    println!("Extracted Text:\n{}", text);

    println!("\n{}", "=" .repeat(50));
    println!("Detailed Document Structure:\n");

    // Parse document for structured access
    let doc = parse_document_from_path(file_path)?;

    println!("Paragraphs: {}", doc.paragraphs.len());
    println!("Tables: {}", doc.tables.len());

    // Show first few paragraphs
    for (i, para) in doc.paragraphs.iter().take(3).enumerate() {
        println!("\nParagraph {}:", i + 1);
        println!("  Text: {}", para.to_text());
        println!("  Runs: {}", para.runs.len());
    }

    // Show table information
    for (i, table) in doc.tables.iter().enumerate() {
        println!("\nTable {}:", i + 1);
        println!("  Rows: {}", table.rows.len());
        if let Some(first_row) = table.rows.first() {
            println!("  Columns: {}", first_row.cells.len());
        }
    }

    Ok(())
}