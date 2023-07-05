use crate::{fetching::indexer::TfIdfModel, fetching::Indexer, parsing::Lexer};
use std::{
    cmp::Ordering,
    sync::{Arc, Mutex},
};

pub struct Searcher;

impl Searcher {
    pub fn search_term(query: &str, tf_idf_model: Arc<Mutex<TfIdfModel>>) -> Vec<(String, f32)> {
        let mut ranks = Vec::<(String, f32)>::new();

        let model = tf_idf_model.lock().unwrap();

        for (path, document) in &model.term_frequency_per_document {
            let mut total_rank = 0f32;

            for token in Lexer::new(&query.chars().collect::<Vec<char>>()) {
                total_rank +=
                    Indexer::term_frequency(&token, &document.frequency_map, document.total_terms)
                        * Indexer::inverse_document_frequency(
                            &token,
                            &model.term_frequency_across_documents,
                        );
            }

            if total_rank.partial_cmp(&0f32).unwrap() == Ordering::Greater {
                ranks.push((path.to_string(), total_rank));
            }
        }

        ranks.sort_by(|(_, rank1), (_, rank2)| rank1.partial_cmp(rank2).unwrap());
        ranks.reverse();

        ranks
    }
}
