use traits::*;

pub struct ParseFirstOf<SubValue> {
	parsers: Vec<Box<Parser<SubValue>>>,
}

impl<SubValue> Parser<SubValue> for ParseFirstOf<SubValue> {
	fn parse<'rest>(&self, source: &'rest str) -> Option<(SubValue, &'rest str)> {
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

pub fn first_of<SubValue>(parsers: Vec<Box<Parser<SubValue>>>) -> ParseFirstOf<SubValue> {
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