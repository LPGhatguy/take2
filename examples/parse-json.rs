#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate try_opt;

extern crate take2;
use take2::*;

static TEST_JSON: &'static str = "
5
";

lazy_static!(
	static ref MAYBE_WHITESPACE: ParseMaybe<ParseWhitespace, &'static str> = maybe(whitespace());
);

#[derive(Debug)]
enum JsonValue {
	Null,
	Number(f64),
}

struct NumberParser;

impl<'rest> Parser<'rest, JsonValue> for NumberParser {
	fn parse(&self, source: &'rest str) -> Option<(JsonValue, &'rest str)> {
		None
	}
}

struct JsonParser;

impl<'a> Parser<'a, JsonValue> for JsonParser {
	fn parse(&self, source: &'a str) -> Option<(JsonValue, &'a str)> {
		let (_, source) = try_opt!(MAYBE_WHITESPACE.parse(source));
		None
	}
}

fn json() -> JsonParser {
	JsonParser
}

fn main() {
	let result = json().parse(TEST_JSON);

	println!("{:?}", result);
}