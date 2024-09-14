use serde::Serialize;

pub trait Parser<T> {
    type Item: Serialize;
    type Error: std::fmt::Display;
    fn parse(args: T) -> Result<Self::Item, Self::Error>;
}
