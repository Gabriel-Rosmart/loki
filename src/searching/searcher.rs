use crate::{fetching::Indexer, parsing::Lexer};
use std::{cmp::Ordering, collections::HashMap};

pub struct Searcher;

type DocumentIndexMap = HashMap<String, HashMap<String, usize>>;

impl Searcher {
    pub fn search_term(query: &str, document_map: &DocumentIndexMap) {
        let mut ranks = Vec::<(String, f32)>::new();

        for (path, freq_table) in document_map {
            let mut total_rank = 0f32;

            for token in Lexer::new(&query.chars().collect::<Vec<char>>()) {
                total_rank += Indexer::term_frequency(&token, &freq_table)
                    * Indexer::inverse_document_frequency(&token, &document_map);
            }

            if total_rank.partial_cmp(&0f32).unwrap() == Ordering::Greater {
                ranks.push((path.to_string(), total_rank));
            }
        }

        ranks.sort_by(|(_, rank1), (_, rank2)| rank1.partial_cmp(rank2).unwrap());
        ranks.reverse();

        for (filepath, rank) in ranks.iter().take(10) {
            println!("{filepath} => {rank}");
        }
    }
}
