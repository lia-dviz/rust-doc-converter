# Doc Converter

A hybrid document conversion engine built with a Python CLI interface and a native Rust core. 

Following the "Read once, export anywhere" approach, it converts text-based documents into a unified format, skipping any heavy OCR or VLM overhead. Plus, custom .bundle archives allow you to re-export your data anytime without the source file.

---

## Architecture

* **Core Engine (Rust):** Handles high-speed file parsing, structural node mapping, bundle serialization, and text export generation via `PyO3` and `maturin`.
* **Interface (Python):** Provides a clean command-line interface using `Click` for user interaction and workflow orchestration.

---

## Supported Formats

| Category | Source Formats (Inputs) | Target Formats (Outputs) |
| :--- | :--- | :--- |
| **Documents** | PDF, DOC, DOCX, RTF, ODT | Markdown, HTML, LaTeX, DOCX, EPUB, PDF, JSON |
| **Spreadsheets** | XLS, XLSX, CSV, TSV, ODS | Markdown, HTML, LaTeX, DOCX, EPUB, PDF, JSON |
| **Presentations** | PPT, PPTX, ODP | Markdown, HTML, LaTeX, DOCX, EPUB, PDF, JSON |
| **Web & E-books** | EPUB, HTML, MHTML, OFD | Markdown, HTML, LaTeX, DOCX, EPUB, PDF, JSON |
| **Archives** | `.bundle` (Pre-parsed state) | Markdown, HTML, LaTeX, DOCX, EPUB, PDF, JSON |

---
