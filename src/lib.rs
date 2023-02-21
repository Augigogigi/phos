//! Regex-like pattern matching on things other than strings.

pub mod pattern_builder;
pub mod pattern;

#[cfg(test)]
pub mod tests;

pub trait Matchable: Clone + PartialEq {}

impl<T: Clone + PartialEq + Sized> Matchable for T {}