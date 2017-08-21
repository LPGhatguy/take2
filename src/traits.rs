pub trait Parser<'rest, Value> {
	fn parse(&self, source: &'rest str) -> Option<(Value, &'rest str)>;
}