pub fn maybe<'source, Value, T>(parser: T) -> impl Fn(&'source str) -> Option<(Option<Value>, &'source str)>
	where T: Fn(&'source str) -> Option<(Value, &'source str)> {

	move |source: &'source str| {
		match parser(source) {
			Some((value, rest)) => Some((Some(value), rest)),
			None => Some((None, source)),
		}
	}
}

#[test]
fn it_matches_literal_maybe() {
	use parsers::literal;

	let parser = maybe(literal("hello"));
	let result = parser("hello, world!");

	assert!(result.is_some());

	let (value, rest) = result.unwrap();

	assert!(value == Some("hello"));
	assert!(rest == ", world!");
}

#[test]
fn it_matches_no_literal() {
	use parsers::literal;

	let parser = maybe(literal("hello"));
	let result = parser("world, hello!");

	assert!(result.is_some());

	let (value, rest) = result.unwrap();

	assert!(value == None);
	assert!(rest == "world, hello!");
}