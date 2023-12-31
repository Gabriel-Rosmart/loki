use rust_stemmers::{Algorithm, Stemmer};

pub struct Lexer<'a> {
    content: &'a [char],
    stemmer: Stemmer,
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a [char]) -> Self {
        Self {
            content,
            stemmer: Stemmer::create(Algorithm::English),
        }
    }

    pub fn next_token(&mut self) -> Option<String> {
        self.trim_left();

        if self.content.is_empty() {
            return None;
        }

        if self.content[0].is_alphabetic() {
            let token = self
                .chop_while(|ch| ch.is_alphanumeric())
                .iter()
                .map(|ch| ch.to_ascii_lowercase())
                .collect::<String>();
            return Some(self.stemmer.stem(&token).to_string());
        }

        if self.content[0].is_numeric() {
            return Some(self.chop_while(|ch| ch.is_numeric()).iter().collect());
        }

        Some(self.chop_and_extract_token(1).iter().collect())
    }

    fn trim_left(&mut self) {
        while !self.content.is_empty() && self.content[0].is_whitespace() {
            self.content = &self.content[1..];
        }
    }

    fn chop_and_extract_token(&mut self, index: usize) -> &'a [char] {
        let token = &self.content[0..index];
        self.content = &self.content[index..];
        token
    }

    fn chop_while<P>(&mut self, mut predicate: P) -> &'a [char]
    where
        P: FnMut(&char) -> bool,
    {
        let mut index: usize = 0;
        while index < self.content.len() && predicate(&self.content[index]) {
            index += 1;
        }

        self.chop_and_extract_token(index)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
