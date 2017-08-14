pub trait Parser<'a> {
	type Value;

	fn parse(&self, source: &'a str) -> Option<(Self::Value, &'a str)>;
}