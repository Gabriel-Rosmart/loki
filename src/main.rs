use std::{collections::HashMap, fs};

use loki::{
    fetching::{indexer::Indexer, reader::Reader, Fetcher},
    parsing::{parser::Parser, Lexer},
};

type DocumentIndexMap = HashMap<String, HashMap<String, usize>>;

fn main() {
    let query: &str = &std::env::args().collect::<Vec<String>>()[1];

    let mut path = std::env!("CARGO_MANIFEST_DIR").to_string();

    path.push_str("/assets");

    let mut document_map = DocumentIndexMap::new();

    println!("Indexing...");

    for file in Fetcher::fetch_directory(&path) {
        let file_content = Reader::read_file(&file);

        let terms_map = Parser::index(file_content.chars().collect::<Vec<char>>());

        document_map.insert(
            file.clone().into_os_string().into_string().unwrap(),
            terms_map,
        );
    }

    let mut ranks = Vec::<(String, f32)>::new();

    for (path, freq_table) in &document_map {
        let mut total_rank = 0f32;

        for token in Lexer::new(&query.chars().collect::<Vec<char>>()) {
            total_rank += Indexer::term_frequency(&token, &freq_table)
                * Indexer::inverse_document_frequency(&token, &document_map);
        }

        ranks.push((path.to_string(), total_rank));
    }

    ranks.sort_by(|(_, rank1), (_, rank2)| rank1.partial_cmp(rank2).unwrap());
    ranks.reverse();

    for (filepath, rank) in ranks.iter().take(10) {
        println!("{filepath} => {rank}");
    }
}
