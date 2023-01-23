//! A crate for pattern matching and pattern permutation generation.
//!
//! It offers a different approach to pattern matching by providing a `Pattern` or `Matcher` struct (similar to a regex expression) that is generic, allowing you to match across any type, not just Strings.

// Imports:
use std::fmt::Formatter;
use std::ops::Range;

/// CRATE INTERNALS | An enum with some variants not meant to be accessible by the user. The `pattern!` macro will generate these for you.
#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) enum Fragment<T: Clone + PartialEq + Eq> {
	None,
	Any,
	// TODO: better thing than Vec<MFrag<T>>? box?
	Quantifier(Vec<Fragment<T>>, Range<usize>, bool),
	// lazy expands the quantity until match, greedy starts at pat.len() and shrinks until match
	// TODO: improve
	Except(Vec<Fragment<T>>, Vec<Fragment<T>>),
	Unit(T),
	Class(usize),
	Or(Vec<Fragment<T>>),
	And(Vec<Fragment<T>>),
	Start,
	End,
}

/// A pattern struct used to match against other patterns. This is NOT used to generate permutations.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Matcher<T: Clone + PartialEq + Eq>(Vec<Fragment<T>>);

impl<T: Clone + PartialEq + Eq> Matcher<T> {
	fn new() -> Self {
		Matcher::default()
	}
}

impl<T: Clone + PartialEq + Eq> Default for Matcher<T> {
	fn default() -> Self {
		Self {
			0: vec![]
		}
	}
}

/// The main difference between `Pattern` and `Matcher` is the lack of the `Any` and `Quantifier` fragments. `Pattern`s can always be promoted to `Matcher`s, but not vice-versa.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Pattern<T: Clone + PartialEq + Eq>(Vec<Fragment<T>>);

impl<T: Clone + PartialEq + Eq> Pattern<T> {
	fn new() -> Self {
		Pattern::default()
	}
}

impl<T: Clone + PartialEq + Eq> Default for Pattern<T> {
	fn default() -> Self {
		Self {
			0: vec![]
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::{Matcher, Pattern};

	#[test]
	fn new_is_default() {
		assert_eq!(Matcher::<u8>::new(), Matcher::<u8>::default());
		assert_eq!(Pattern::<u8>::new(), Pattern::<u8>::default());
	}
}