use clap::Parser;

use crate::errors::PdfExtractorError;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The path to the PDF file to extract pages from
    #[arg(short, long)]
    pub input: String,

    /// The page numbers to extract (e.g., "1,3-5")
    #[arg(short, long)]
    pub pages: String,

    /// The output directory for the extracted pages
    #[arg(short, long)]
    pub output: String,
}

pub fn parse_args() -> Result<Args, PdfExtractorError> {
    let args = Args::parse();
    if args.input.is_empty() {

    }

    Ok(args)
}

