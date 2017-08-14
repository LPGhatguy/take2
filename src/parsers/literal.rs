use traits::*;

pub struct MatchLiteral<'a> {
	literal: &'a str,
}

impl<'a> Parser<'a> for MatchLiteral<'a> {
	type Value = &'a str;

	fn parse(&self, source: &'a str) -> Option<(Self::Value, &'a str)> {
		let zip = source.char_indices().zip(self.literal.chars());

		for ((_, source_char), literal_char) in zip {
			if source_char != literal_char {
				return None;
			}
		}

		Some((self.literal, &source[self.literal.len()..]))
	}
}

impl<'a> MatchLiteral<'a> {
	pub fn new(literal: &'a str) -> MatchLiteral<'a> {
		MatchLiteral {
			literal,
		}
	}
}

#[test]
fn it_matches_literal() {
	let source = "hello, world!";
	let parser = MatchLiteral::new("hello");

	let result = parser.parse(source);

	assert!(result.is_some());

	let (value, rest) = result.unwrap();

	assert!(value == "hello");
	assert!(rest == ", world!");
}