# docx-lite

[![Crates.io](https://img.shields.io/crates/v/docx-lite)](https://crates.io/crates/docx-lite)
[![Documentation](https://docs.rs/docx-lite/badge.svg)](https://docs.rs/docx-lite)
[![License](https://img.shields.io/crates/l/docx-lite)](LICENSE)

A lightweight, fast DOCX text extraction library for Rust with minimal dependencies.

## Features

- 🚀 **Fast** - Optimized for speed with streaming XML parsing
- 🪶 **Lightweight** - Minimal dependencies (only `zip`, `quick-xml`, and `thiserror`)
- 🛡️ **Safe** - Zero unsafe code
- 📊 **Tables** - Full support for table text extraction
- 🎯 **Simple API** - Easy to use with both simple and advanced APIs
- 🔧 **Robust** - Handles malformed documents gracefully

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
docx-lite = "0.2.0"
```

## Quick Start

```rust
use docx_lite::extract_text;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = extract_text("document.docx")?;
    println!("{}", text);
    Ok(())
}
```

## Advanced Usage

```rust
use docx_lite::{parse_document_from_path, ExtractOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let doc = parse_document_from_path("document.docx")?;

    // Extract text with all options enabled
    let options = ExtractOptions::all();
    let text = doc.extract_text_with_options(&options);
    println!("{}", text);

    // Or customize extraction
    let custom_options = ExtractOptions {
        include_headers: true,
        include_footers: true,
        include_footnotes: false,
        include_endnotes: false,
        include_list_markers: true,
    };
    let custom_text = doc.extract_text_with_options(&custom_options);

    // Access specific elements
    for list_item in &doc.lists {
        println!("List item (level {}): {}", list_item.level, list_item.text);
    }

    for footnote in &doc.footnotes {
        println!("Footnote {}: {}",
            footnote.id,
            footnote.paragraphs[0].to_text()
        );
    }

    Ok(())
}
```

## API

### Simple API

- `extract_text(path)` - Extract all text from a DOCX file
- `extract_text_from_bytes(bytes)` - Extract text from DOCX bytes
- `extract_text_from_reader(reader)` - Extract text from any reader

### Advanced API

- `parse_document(reader)` - Parse DOCX into a structured Document
- `parse_document_from_path(path)` - Parse DOCX file into a structured Document

## Supported Elements

- ✅ Paragraphs
- ✅ Runs (with bold, italic, underline formatting)
- ✅ Tables (with rows and cells)
- ✅ Lists (bullets and numbering) - **NEW in v0.2.0**
- ✅ Headers/Footers - **NEW in v0.2.0**
- ✅ Footnotes/Endnotes - **NEW in v0.2.0**
- ✅ Advanced text extraction with options

## Performance

`docx-lite` is designed for speed and efficiency:

- Streaming XML parsing (no full DOM loading)
- Minimal memory allocation
- Zero-copy where possible
- Optimized for text extraction use case

## Why docx-lite?

Unlike other DOCX libraries in the Rust ecosystem, `docx-lite`:

1. **Compiles on modern Rust** - No issues with latest Rust versions
2. **Minimal dependencies** - Reduces compilation time and security surface
3. **Production-ready** - Used in production at V-Lawyer
4. **Focused scope** - Does one thing well: text extraction

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is dual-licensed under MIT OR Apache-2.0.

## Credits

Developed by the V-Lawyer team as part of our commitment to open source.