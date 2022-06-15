
pub trait CharStream {
    fn take(&mut self) -> Option<char>;
}

pub struct StrCharStream<'a> {
    s: &'a str,
}
