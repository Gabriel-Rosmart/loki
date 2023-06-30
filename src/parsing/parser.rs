use super::lexer::Lexer;
use std::collections::HashMap;

type FrequencyMap = HashMap<String, usize>;

pub struct Parser;

impl Parser {
    pub fn index(file_contents: String) -> FrequencyMap {
        let contents_slice = file_contents.chars().collect::<Vec<char>>();
        let mut lexer = Lexer::new(&contents_slice);
        let mut term_frequencies = FrequencyMap::new();

        while let Some(token_slice) = lexer.next_token() {
            let token = token_slice
                .iter()
                .map(|ch| ch.to_ascii_uppercase())
                .collect::<String>();

            term_frequencies
                .entry(token)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        term_frequencies
    }
}
