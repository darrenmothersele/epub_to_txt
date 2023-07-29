use clap::Parser;
use epub::doc::{DocError, EpubDoc};
use html2md::parse_html;
use scraper::{Html, Selector};
use std::process::exit;

#[derive(Parser)]
struct Args {
    /// Name of the file to convert
    #[arg()]
    name: String,
}

fn main() {
    let args = Args::parse();

    match convert_epub(&args.name) {
        Ok(content) => {
            println!("{content}");
        }
        Err(err) => {
            eprintln!("{err}");
            exit(1);
        }
    }
}

fn convert_epub(filename: &str) -> Result<String, DocError> {
    let mut doc = EpubDoc::new(filename)?;

    let mut collected_content = String::new();

    let body_selector = Selector::parse("body").unwrap();
    while doc.go_next() {
        match doc.get_current_str() {
            Some((current, _)) => {
                let document = Html::parse_document(current.as_str());
                if let Some(body_element) = document.select(&body_selector).next() {
                    let body_html = body_element.inner_html();
                    let content = parse_html(body_html.as_str());
                    collected_content.push_str(&content);
                }
            }
            None => return Err(DocError::InvalidEpub),
        }
    }

    Ok(collected_content)
}
