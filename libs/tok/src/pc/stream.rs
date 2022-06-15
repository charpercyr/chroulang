
#[derive(Clone, Copy, Debug)]
pub struct CharStream<'a> {
    pos: usize,
    rest: &'a str,
}

impl<'a> CharStream<'a> {
    pub fn next(&mut self) -> Option<char> {
        let c = self.rest.chars().next()?;
        self.pos += 1;
        Some(c)
    }
}