#![feature(conservative_impl_trait)]

pub type Parser<'source, Value> = Fn(&'source str) -> Option<(Value, &'source str)>;

mod parsers;

pub use parsers::*;