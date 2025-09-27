# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2024-09-27

### Added
- **Lists Support**: Full support for bullet lists and numbered lists
  - Automatic detection of list items from paragraph numbering
  - Support for multi-level lists with indentation
  - Configurable list markers in text extraction
- **Headers and Footers**: Extract content from document headers and footers
  - Support for multiple header/footer types (default, first, odd, even)
  - Separate extraction control via `ExtractOptions`
- **Footnotes and Endnotes**: Full support for footnotes and endnotes
  - Parse footnote/endnote references with IDs
  - Extract note content with proper formatting
- **ExtractOptions**: New configuration system for text extraction
  - `ExtractOptions::all()` - Include all additional content
  - `ExtractOptions::none()` - Main content only (default)
  - Fine-grained control over what to include
- **extract_text_with_options()**: New method for customized text extraction

### Changed
- `Document` struct now includes `lists`, `headers`, `footers`, `footnotes`, and `endnotes` fields
- `Paragraph` struct enhanced with `numbering_id` and `numbering_level` for list detection
- Improved parser to handle multiple DOCX parts (numbering.xml, headers, footers, notes)

### Fixed
- Better handling of complex document structures
- More graceful handling of missing optional document parts

## [0.1.0] - 2024-09-26

### Added
- Initial release
- Basic DOCX text extraction
- Support for paragraphs, runs, and tables
- Minimal dependencies (zip, quick-xml, thiserror)
- Streaming XML parsing for efficiency
- Support for bold, italic, and underline formatting

[0.2.0]: https://github.com/v-lawyer/docx-lite/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/v-lawyer/docx-lite/releases/tag/v0.1.0