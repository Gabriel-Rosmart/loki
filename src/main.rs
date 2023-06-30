use std::{collections::HashMap, fs};

use loki::{fetching::Fetcher, parsing::parser::Parser};

type DocumentIndexMap = HashMap<String, HashMap<String, usize>>;

fn main() {
    let mut path = std::env!("CARGO_MANIFEST_DIR").to_string();

    path.push_str("/assets");

    let mut document_map = DocumentIndexMap::new();

    for file in Fetcher::fetch_directory(&path) {
        let terms_map = Parser::index(fs::read_to_string(&file).unwrap());

        document_map.insert(
            file.clone().into_os_string().into_string().unwrap(),
            terms_map,
        );
    }

    print!("{document_map:#?}");
}
