use std::{collections::HashSet, error::Error, process::exit};

use lopdf::Document;

use crate::{args::Args, errors::PdfExtractorError};

mod args;
mod errors;

fn main() {
    let args = get_args();
    let pages = parse_page_numbers(&args.pages);
    let old_doc = open_input_file(&args.input);
    let new_doc = delete_unwanted_pages(old_doc, pages);
    output_to_file(new_doc, &args.output);
}

fn exit_error(msg: &str, e: impl Error) {
    println!("{msg}: {}", e);
    exit(1);
}

fn get_args() -> Args {
    args::parse_args().map_err(|e| exit_error("failed to parse args", e)).unwrap()
}

fn open_input_file(input_path: &str) -> Document {
    let file = lopdf::Document::load(input_path).map_err(|e| exit_error("failed to open input file",e)).unwrap();
    return file;
}

fn parse_page_numbers(pages: &str) -> Vec<u32>{
    let mut res = Vec::new();
    res.reserve(pages.len());
    
    let pages_split = pages.split(',');

    for page_str in pages_split {
        let page_str_split = page_str.split('-');
        let split_count = page_str_split.clone().count();
        if split_count == 2 {
            let pages_vec: Vec<&str> = page_str_split.collect();
            let start_page = str::parse::<u32>(pages_vec[0])
                .map_err(|e| exit_error("failed to parse start page to number", e)).unwrap();
            let end_page = str::parse::<u32>(pages_vec[1])
                .map_err(|e| exit_error("failed to parse end page to number", e)).unwrap();

            if end_page <= start_page {
                exit_error(
                    "invalid page range",
                    PdfExtractorError::InvalidPages(
                        format!("{}-{}", start_page, end_page)
                    )
                );
            }
            for i in start_page..=end_page {
                res.push(i);
            }
        } else if split_count > 2 {
            exit_error(
                    "invalid pages specified",
                    PdfExtractorError::InvalidPages(
                        format!("{}", pages)
                )
            );
        } else {
            match str::parse::<u32>(page_str) {
                Ok(nr) => res.push(nr),
                Err(e) => {
                    exit_error("failed to parse page number", e);
                }
            }
        }
    }

    res
}

fn delete_unwanted_pages(mut doc: Document, wanted_pages: Vec<u32>) -> Document {
    let pages = doc.get_pages();
    
    let mut page_nrs = pages.keys().cloned().collect::<HashSet<u32>>();
    for idx in wanted_pages {
        page_nrs.remove(&idx);
    }

    let page_vec = page_nrs.iter().cloned().collect::<Vec<u32>>();

    doc.delete_pages(&page_vec);
    doc.prune_objects();
    doc
}

fn output_to_file(mut doc: Document, output_path: &str) {
    doc.save(output_path).map_err(|e| exit_error("failed to save output", e)).unwrap();
}
