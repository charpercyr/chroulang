mod stream;

pub type Result<'a, T> = std::result::Result<(&'a str, T), &'a str>;

pub trait Parser<'a> {
    type Output;

    fn parse(&mut self, input: &'a str) -> Result<'a, Self::Output>;
}

pub trait ParserExt<'a>: Parser<'a> + Sized {
    fn map<F>(self, f: F) -> ParserMap<Self, F> {
        ParserMap { p: self, f }
    }
}
impl<'a, P: Parser<'a>> ParserExt<'a> for P {}

pub struct ParserMap<P, F> {
    p: P,
    f: F,
}
impl<'a, R, P: Parser<'a>, F: FnMut(P::Output) -> R> Parser<'a> for ParserMap<P, F> {
    type Output = R;
    fn parse(&mut self, input: &'a str) -> Result<'a, Self::Output> {
        let (rest, r) = self.p.parse(input)?;
        Ok((rest, (self.f)(r)))
    }
}

pub struct ParserFn<F>(F);
impl<'a, R, F: FnMut(&'a str) -> Result<'a, R>> Parser<'a> for ParserFn<F> {
    type Output = R;
    fn parse(&mut self, input: &'a str) -> Result<'a, Self::Output> {
        (self.0)(input)
    }
}
pub const fn parser_fn<F>(f: F) -> ParserFn<F> {
    ParserFn(f)
}

pub struct Whitespace;
impl<'a> Parser<'a> for Whitespace {
    type Output = char;
    fn parse(&mut self, input: &'a str) -> Result<'a, Self::Output> {
        let c = input.chars().next().ok_or(input)?;
        if c.is_whitespace() {
            Ok((&input[c.len_utf8()..], c))
        } else {
            Err(input)
        }
    }
}
pub const fn whitespace() -> Whitespace {
    Whitespace
}

pub struct Whitespaces;
impl<'a> Parser<'a> for Whitespaces {
    type Output = &'a str;
    fn parse(&mut self, input: &'a str) -> Result<'a, Self::Output> {
        for (i, c) in input.char_indices() {
            if !c.is_whitespace() {
                return Ok(())
            }
        }
        Ok(("", input))
    }
}

fn parse<'a, P: Parser<'a>>(mut p: impl AsMut<P>, input: &'a str) -> Result<'a, P::Output> {
    let p = p.as_mut();
    p.parse(input)
}
