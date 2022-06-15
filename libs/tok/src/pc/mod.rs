mod stream;

pub type Result<'a, T> = std::result::Result<(&'a str, T), &'a str>;

pub trait Parser<'a> {
    type Output;

    fn parse(&mut self, input: &'a str) -> Result<'a, Self::Output>;
}

fn parse<'a, P: Parser<'a>>(mut p: impl AsMut<P>, input: &'a str) -> Result<'a, P::Output> {
    let p = p.as_mut();
    p.parse(input)
}
