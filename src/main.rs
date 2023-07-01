use std::{collections::HashMap, fs};

use loki::{
    fetching::{indexer::Indexer, Fetcher},
    parsing::{parser::Parser, Lexer},
};

type DocumentIndexMap = HashMap<String, HashMap<String, usize>>;

fn main() {
    let mut path = std::env!("CARGO_MANIFEST_DIR").to_string();

    path.push_str("/assets");

    let mut document_map = DocumentIndexMap::new();

    for file in Fetcher::fetch_directory(&path) {
        let terms_map = Parser::index(
            fs::read_to_string(&file)
                .unwrap()
                .chars()
                .collect::<Vec<char>>(),
        );

        document_map.insert(
            file.clone().into_os_string().into_string().unwrap(),
            terms_map,
        );
    }

    // println!("{document_map:#?}");
    //
    // let search_query: Vec<char> = "rust programming language docs".chars().collect();
    //
    // for token in Lexer::new(&search_query) {
    //     println!("{token:?}");
    // }

    let mut ranks = Vec::<(String, f32)>::new();

    for (path, freq_table) in &document_map {
        let mut total_rank = 0f32;

        for token in Lexer::new(&"horse".chars().collect::<Vec<char>>()) {
            total_rank += Indexer::term_frequency(&token, &freq_table)
                * Indexer::inverse_document_frequency(&token, &document_map);

            // println!(
            //     "  {token} => {rank}",
            //     rank = Indexer::inverse_document_frequency(&token, &document_map)
            // );
        }

        ranks.push((path.to_string(), total_rank));
        // println!("{path} => {total_rank}");
    }

    ranks.sort_by(|(_, rank1), (_, rank2)| rank1.partial_cmp(rank2).unwrap());
    ranks.reverse();

    println!("{ranks:#?}");
}
