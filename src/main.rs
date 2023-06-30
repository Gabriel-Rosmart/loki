use std::fs;

use loki::{fetching::Fetcher, parsing::parser::Parser};

fn main() {
    let mut path = std::env!("CARGO_MANIFEST_DIR").to_string();

    path.push_str("/assets");

    for file in Fetcher::fetch_directory(&path) {
        println!("Dir entry: {file:?}");
        let terms_map = Parser::index(fs::read_to_string(file).unwrap());
        let mut top_terms = terms_map.iter().collect::<Vec<_>>();
        top_terms.sort_by_key(|(_, frequency)| *frequency);
        top_terms.reverse();

        for (term, frequency) in top_terms.iter().take(10) {
            println!("  [{term} => {frequency}]");
        }
    }
}
