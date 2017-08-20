use traits::*;

pub struct ParseWhitespace;

impl<'result> Parser<&'result str> for ParseWhitespace {
	fn parse<'rest>(&self, source: &'rest str) -> Option<(&'rest str, &'rest str)> {
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

		if start_pos == 0 {
			return None;
		}

		return Some((&source[0..start_pos], &source[start_pos..]));
	}
}

pub fn whitespace() -> ParseWhitespace {
	ParseWhitespace
}

#[test]
fn it_matches_whitespace() {
	let parser = whitespace();
	let result = parser.parse("  test");

	assert!(result.is_some());

	let (value, rest) = result.unwrap();

	assert!(value == "  ");
	assert!(rest == "test");
}

#[test]
fn it_fails_no_whitespace() {
	let parser = whitespace();
	let result = parser.parse("hello, world!");

	assert!(result.is_none());
}