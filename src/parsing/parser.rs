use super::lexer::Lexer;
use crate::fetching::indexer::TfIdfModel;
use std::collections::HashMap;

type FrequencyMap = HashMap<String, usize>;

pub struct Parser;

impl Parser {
    pub fn index(
        file_contents: Vec<char>,
        documents_term_map: &mut TfIdfModel,
    ) -> (FrequencyMap, usize) {
        let mut lexer = Lexer::new(&file_contents);
        let mut term_frequencies = FrequencyMap::new();
        let mut total_entries: usize = 0;

        while let Some(token_slice) = lexer.next_token() {
            let token = token_slice
                .chars()
                .filter(|ch| ch.is_alphanumeric())
                .collect::<String>();

            if !token.is_empty() {
                term_frequencies
                    .entry(token)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);

                total_entries += 1;
            }
        }

        for term in term_frequencies.keys() {
            documents_term_map
                .term_frequency_across_documents
                .entry(term.to_string())
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        (term_frequencies, total_entries)
    }
}
