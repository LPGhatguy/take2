pub fn literal<'rest>(literal: &'rest str) -> impl Fn(&'rest str) -> Option<(&'rest str, &'rest str)> {
	move |source: &'rest str| {
		let zip = source.char_indices().zip(literal.chars());

		for ((_, source_char), literal_char) in zip {
			if source_char != literal_char {
				return None;
			}
		}

		Some((literal, &source[literal.len()..]))
	}
}

#[test]
fn it_matches_literal() {
	let parser = literal("hello");
	let result = parser("hello, world!");

	assert!(result.is_some());

	let (value, rest) = result.unwrap();

	assert!(value == "hello");
	assert!(rest == ", world!");
}

#[test]
fn it_fails_no_literal() {
	let parser = literal("world!");
	let result = parser("hello, world!");

	assert!(result.is_none());
}