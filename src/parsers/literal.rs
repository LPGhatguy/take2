pub fn literal<'rest>(literal: &'rest str) -> impl Fn(&'rest str) -> Option<(&'rest str, &'rest str)> {
	move |source: &'rest str| {
		if !source.starts_with(literal) {
			return None;
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

#[test]
fn it_fails_end_of_string() {
	let parser = literal("a");
	let result = parser("");

	assert!(result.is_none());
}