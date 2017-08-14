use traits::*;

pub struct ParseWhitespace;

impl<'a> Parser<'a> for ParseWhitespace {
	type Value = &'a str;

	fn parse(&self, source: &'a str) -> Option<(Self::Value, &'a str)> {
		let mut start_pos = 0;

		for (pos, char) in source.char_indices() {
			match char {
				' ' | '\t' | '\r' | '\n' => {},
				_ => {
					start_pos = pos;
					break;
				}
			}
		}

		return Some((&source[0..start_pos], &source[start_pos..]));
	}
}

impl ParseWhitespace {
	pub fn new() -> ParseWhitespace {
		ParseWhitespace {}
	}
}

#[test]
fn it_matches_whitespace() {
	let source = "  test";

	let parser = ParseWhitespace::new();
	let result = parser.parse(source);

	assert!(result.is_some());

	let (value, rest) = result.unwrap();

	assert!(value == "  ");
	assert!(rest == "test");
}