#[inline(always)]
pub fn take_while<'source, T>(condition: T) -> impl Fn(&'source str) -> Option<(&'source str, &'source str)>
	where T: Fn(char) -> bool {
	move |source: &'source str| {
		let mut start_pos = 0;

		for (pos, char) in source.char_indices() {
			if condition(char) {
				start_pos = pos + char.len_utf8();
			} else {
				break;
			}
		}

		Some((&source[0..start_pos], &source[start_pos..]))
	}
}

#[test]
fn it_takes_characters() {
	fn is_bad(c: char) -> bool {
		c == ' ' || c == 'h'
	}

	let skip_bad = take_while(is_bad);

	let result = skip_bad("  hello, world!");

	assert!(result.is_some());

	let (value, rest) = result.unwrap();

	assert_eq!(value, "  h");
	assert_eq!(rest, "ello, world!");
}

#[test]
fn it_takes_to_end() {
	fn should_take(c: char) -> bool {
		c == 'ぁ'
	}

	let skip_a = take_while(should_take);

	let result = skip_a("ぁぁぁ");

	assert!(result.is_some());

	let (value, rest) = result.unwrap();

	assert_eq!(value, "ぁぁぁ");
	assert_eq!(rest, "");
}

#[test]
fn it_takes_no_whitespace() {
	fn is_whitespace(c: char) -> bool {
		c == ' ' || c == '\n' || c == '\r' || c == '\t'
	}

	let skip_whitespace = take_while(is_whitespace);

	let result = skip_whitespace("ayy");

	assert!(result.is_some());

	let (value, rest) = result.unwrap();

	assert_eq!(value, "");
	assert_eq!(rest, "ayy");
}