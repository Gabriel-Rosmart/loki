use super::lexer::Lexer;
use std::collections::HashMap;

type FrequencyMap = HashMap<String, usize>;

pub struct Parser;

impl Parser {
    pub fn index(file_contents: Vec<char>) -> FrequencyMap {
        // let contents_slice = file_contents.chars().collect::<Vec<char>>();
        let mut lexer = Lexer::new(&file_contents);
        let mut term_frequencies = FrequencyMap::new();

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
            }
        }

        term_frequencies
    }
}
