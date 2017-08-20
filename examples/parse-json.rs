extern crate take2;
use take2::*;

static TEST_JSON: &'static str = "
5
";

#[derive(Debug)]
enum JsonValue {
	Null,
	Number(f64),
}

struct JsonParser;

impl<'a, JsonValue> Parser<'a, JsonValue> for JsonParser {
	fn parse(&self, source: &'a str) -> Option<(JsonValue, &'a str)> {
		None
	}
}

fn json() -> JsonParser {
	JsonParser
}

fn main() {
	let parser = json();

	// I want lifetime elision here; refactoring lifetimes now.
	let result: Option<(JsonValue, &'static str)> = parser.parse(TEST_JSON);

	println!("{:?}", result);
}