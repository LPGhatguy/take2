use traits::*;

pub struct ParseInsideBalanced {
	start: char,
	end: char,
}

impl<'rest> Parser<'rest, &'rest str> for ParseInsideBalanced {
	fn parse(&self, source: &'rest str) -> Option<(&'rest str, &'rest str)> {
		let mut start_pos = 0;
		let mut end_pos = 0;

		let mut iter = source.char_indices();

		if let Some((_, first_char)) = iter.next() {
			if first_char != self.start {
				return None;
			}

			start_pos = first_char.len_utf8();
		} else {
			return None;
		}

		for (pos, char) in iter {
			end_pos = pos;

			if char == self.end {
				break;
			}
		}

		if end_pos == 0 {
			return None;
		}

		let value = &source[start_pos..end_pos];
		let rest = &source[(end_pos + 1)..];

		Some((value, rest))
	}
}

pub fn inside_balanced(start: char, end: char) -> ParseInsideBalanced {
	ParseInsideBalanced {
		start,
		end,
	}
}

#[test]
fn it_matches_inside_same() {
	let parser = inside_balanced('a', 'a');

	let result = parser.parse("ahello!ayep");

	assert!(result.is_some());

	let (value, rest) = result.unwrap();

	assert_eq!(value, "hello!");
	assert_eq!(rest, "yep");
}

#[test]
fn it_is_ok_at_end() {
	let parser = inside_balanced('a', 'b');

	let result = parser.parse("ab");

	assert!(result.is_some());

	let (value, rest) = result.unwrap();

	assert_eq!(value, "");
	assert_eq!(rest, "");
}

#[test]
fn it_fails_with_no_match() {
	let parser = inside_balanced('(', ')');

	let result = parser.parse("hey");

	assert!(result.is_none());
}