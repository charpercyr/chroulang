mod pc;

#[derive(Clone, Copy, Debug)]
pub struct Span {
    pub start: usize,
    pub bytes_start: usize,
    pub end: usize,
    pub bytes_end: usize,
}

impl Span {
    pub fn merge(self, rhs: Self) -> Self {
        Self {
            start: self.start.min(rhs.start),
            bytes_start: self.bytes_start.min(rhs.bytes_start),
            end: self.end.max(rhs.end),
            bytes_end: self.bytes_end.max(rhs.bytes_end),
        }
    }

    pub fn source<'a>(&self, src: &'a str) -> &'a str {
        &src[self.bytes_start..self.bytes_end]
    }
}

pub enum TokenType {
    Whitespace,
}

pub struct Token {
    pub typ: TokenType,
    pub span: Span,
}
