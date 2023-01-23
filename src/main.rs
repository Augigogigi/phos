//! `phos` is a powerful and flexible rust crate for pattern matching and pattern permutation generation.
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
	Unit(T),
	Class(usize),
	Or(Vec<Fragment<T>>),
	And(Vec<Fragment<T>>),
	Start,
	End,
}

/// hi
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Matcher<T>(Box<Fragment<T>>);

impl<T: Clone + PartialEq + Eq> Default for Matcher<T> {
	fn default() -> Self {
		Self {
			0: Box::new(Fragment::None)
		}
	}
}

/// The main difference between `Pattern` and `Matcher` is the lack of the `Any` and `Quantifier` fragments. `Pattern`s can always be promoted to `Matcher`s, but not vice-versa.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Pattern<T>(Box<Fragment<T>>);

impl<T: Clone + PartialEq + Eq> Default for Pattern<T> {
	fn default() -> Self {
		Self {
			0: Box::new(Fragment::None)
		}
	}
}

fn main() {}