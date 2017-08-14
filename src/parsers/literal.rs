use traits::*;

pub struct ParseLiteral<'a> {
	literal: &'a str,
}

impl<'a> Parser<'a, &'a str> for ParseLiteral<'a> {
	fn parse(&self, source: &'a str) -> Option<(&'a str, &'a str)> {
		let zip = source.char_indices().zip(self.literal.chars());

		for ((_, source_char), literal_char) in zip {
			if source_char != literal_char {
				return None;
			}
		}

		Some((self.literal, &source[self.literal.len()..]))
	}
}

pub fn literal<'a>(literal: &'a str) -> ParseLiteral<'a> {
	ParseLiteral {
		literal,
	}
}

#[test]
fn it_matches_literal() {
	let parser = literal("hello");
	let result = parser.parse("hello, world!");

	assert!(result.is_some());

	let (value, rest) = result.unwrap();

	assert!(value == "hello");
	assert!(rest == ", world!");
}

#[test]
fn it_fails_no_literal() {
	let parser = literal("world!");
	let result = parser.parse("hello, world!");

	assert!(result.is_none());
}