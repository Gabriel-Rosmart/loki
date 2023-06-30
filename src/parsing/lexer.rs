pub struct Lexer<'a> {
    content: &'a [char],
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a [char]) -> Self {
        Self { content }
    }

    pub fn next_token(&mut self) -> Option<&'a [char]> {
        self.trim_left();

        if self.content.is_empty() {
            return None;
        }

        if self.content[0].is_alphabetic() {
            return Some(self.chop_while(|ch| ch.is_alphanumeric()));
        }

        if self.content[0].is_numeric() {
            return Some(self.chop_while(|ch| ch.is_numeric()));
        }

        Some(self.chop_and_extract_token(1))
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
    type Item = &'a [char];

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
