#[inline(always)]
pub fn take_while<'source, T>(condition: T) -> impl Fn(&'source str) -> Option<(&'source str, &'source str)>
	where T: Fn(char) -> bool {
	move |source: &'source str| {
		let mut start_pos = 0;

		for (pos, char) in source.char_indices() {
			start_pos = pos;

			if !condition(char) {
				break;
			}
		}

		Some((&source[0..start_pos], &source[start_pos..]))
	}
}

#[test]
fn it_takes_whitespace() {
	#[inline(always)]
	fn is_whitespace(c: char) -> bool {
		c == ' ' || c == '\n' || c == '\r' || c == '\t'
	}

	let skip_whitespace = take_while(is_whitespace);

	let result = skip_whitespace("\r\n\t  hello, world!");

	assert!(result.is_some());

	let (value, rest) = result.unwrap();

	assert_eq!(value, "\r\n\t  ");
	assert_eq!(rest, "hello, world!");
}

#[test]
fn it_takes_no_whitespace() {
	#[inline(always)]
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