pub trait Parser<'a, Value> {
	fn parse(&self, source: &'a str) -> Option<(Value, &'a str)>;
}