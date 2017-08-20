pub trait Parser<Value> {
	fn parse<'a>(&self, source: &'a str) -> Option<(Value, &'a str)>;
}