use crate::{fetching::indexer::TermMapThroughDocuments, fetching::Indexer, parsing::Lexer};
use std::cmp::Ordering;

pub struct Searcher;

impl Searcher {
    pub fn search_term(query: &str, documents_term_map: &TermMapThroughDocuments) {
        let mut ranks = Vec::<(String, f32)>::new();

        for (path, (freq_table, entries)) in &documents_term_map.term_frequency_per_document {
            let mut total_rank = 0f32;

            for token in Lexer::new(&query.chars().collect::<Vec<char>>()) {
                total_rank += Indexer::term_frequency(&token, &freq_table, *entries)
                    * Indexer::inverse_document_frequency(
                        &token,
                        &documents_term_map.document_frequency,
                    );
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
