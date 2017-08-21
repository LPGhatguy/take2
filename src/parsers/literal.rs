use traits::*;

pub struct ParseLiteral<'internal> {
	literal: &'internal str,
}

impl<'internal, 'rest> Parser<'rest, &'internal str> for ParseLiteral<'internal> {
	fn parse(&self, source: &'rest str) -> Option<(&'internal str, &'rest str)> {
		let zip = source.char_indices().zip(self.literal.chars());

		for ((_, source_char), literal_char) in zip {
			if source_char != literal_char {
				return None;
			}
		}

		Some((self.literal, &source[self.literal.len()..]))
	}
}

pub fn literal<'internal>(literal: &'internal str) -> ParseLiteral<'internal> {
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