use traits::*;
use std::marker::PhantomData;

pub struct ParseMaybe<SubParser, SubValue> {
	marker: PhantomData<SubValue>,
	sub_parser: SubParser,
}

impl<'a, SubParser, SubValue> Parser<'a, Option<SubValue>> for ParseMaybe<SubParser, SubValue> where
	SubParser: Parser<'a, SubValue> {

	fn parse(&self, source: &'a str) -> Option<(Option<SubValue>, &'a str)> {
		match self.sub_parser.parse(source) {
			Some((value, rest)) => Some((Some(value), rest)),
			None => Some((None, source)),
		}
	}
}

pub fn maybe<'a, SubParser, SubValue>(sub_parser: SubParser) -> ParseMaybe<SubParser, SubValue>
	where SubParser: Parser<'a, SubValue> {
	ParseMaybe {
		sub_parser,
		marker: PhantomData,
	}
}

#[test]
fn it_matches_literal_maybe() {
	use parsers::literal;

	let parser = maybe(literal("hello"));
	let result = parser.parse("hello, world!");

	assert!(result.is_some());

	let (value, rest) = result.unwrap();

	assert!(value == Some("hello"));
	assert!(rest == ", world!");
}

#[test]
fn it_matches_no_literal() {
	use parsers::literal;

	let parser = maybe(literal("hello"));
	let result = parser.parse("world, hello!");

	assert!(result.is_some());

	let (value, rest) = result.unwrap();

	assert!(value == None);
	assert!(rest == "world, hello!");
}