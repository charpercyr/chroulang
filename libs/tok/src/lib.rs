
mod pc;

pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    
}

pub enum TokenType {
    Whitespace,
}

pub struct Token {
    pub typ: TokenType,
    pub span: Span,
}
