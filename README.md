# PDF Page Extractor

A simple command-line tool to extract specific pages from a PDF file.

## 🛠️ Usage

```bash
pdf-page-extractor.exe --input <INPUT> --pages <PAGES> --output <OUTPUT>
```

## 📌 Options

| Flag                      | Description                                       |
| ------------------------- | ------------------------------------------------- |
| `-i`, `--input <INPUT>`   | Path to the PDF file to extract pages from        |
| `-p`, `--pages <PAGES>`   | Page numbers or ranges to extract (e.g., `1,3-5`) |
| `-o`, `--output <OUTPUT>` | Directory where extracted pages will be saved     |
| `-h`, `--help`            | Show help information                             |
| `-V`, `--version`         | Show version information                          |

## 💡 Example

Extract pages 1 and 3–5 from `document.pdf`:

```bash
pdf-page-extractor.exe -i document.pdf -p 1,3-5 -o ./extracted
```

## 📦 Dependencies

This project uses the [`lopdf`](https://crates.io/crates/lopdf) Rust crate to parse and manipulate PDF files.