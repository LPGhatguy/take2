use traits::*;

pub struct ParseFirstOf<'rest, SubValue> {
	parsers: Vec<Box<Parser<'rest, SubValue>>>,
}

impl<'rest, SubValue> Parser<'rest, SubValue> for ParseFirstOf<'rest, SubValue> {
	fn parse(&self, source: &'rest str) -> Option<(SubValue, &'rest str)> {
		for parser in &self.parsers {
			match parser.parse(source) {
				v@Some(_) => {
					return v;
				},
				None => {},
			}
		}

		None
	}
}

pub fn first_of<'rest, SubValue>(parsers: Vec<Box<Parser<'rest, SubValue>>>) -> ParseFirstOf<'rest, SubValue> {
	ParseFirstOf {
		parsers,
	}
}

#[test]
fn it_returns_first_literal() {
	use parsers::literal;

	let parser = first_of(vec![
		Box::new(literal("a")),
		Box::new(literal("b")),
		Box::new(literal("c")),
	]);

	{
		let result = parser.parse("bac");

		assert!(result.is_some());

		let (value, rest) = result.unwrap();

		assert!(value == "b");
		assert!(rest == "ac");
	}
}

#[test]
fn it_fails_no_literal() {
	use parsers::literal;

	let parser = first_of(vec![
		Box::new(literal("a")),
		Box::new(literal("b")),
		Box::new(literal("c")),
	]);

	let result = parser.parse("dabc");

	assert!(result.is_none());
}