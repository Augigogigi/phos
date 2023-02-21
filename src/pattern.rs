use super::Matchable;

/// TODO: DOCUMENT
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum QuantifierRange {
	Exactly(usize),
	AtLeast(usize),
	AtMost(usize),
	Between(usize, usize),
}

/// TODO: DOCUMENT
#[derive(Clone, PartialEq, Debug)]
enum PatternToken<T: Matchable> {
	Unit(T),
	Or(Vec<PatternToken<T>>),
	And(Vec<PatternToken<T>>),
	Quantifier(Box<PatternToken<T>>, QuantifierRange),
}

/// TODO: DOCUMENT
#[derive(Clone, PartialEq, Debug)]
pub struct Pattern<T: Matchable> {
	spooky: std::marker::PhantomData<T>,
}

impl<T: Matchable> Pattern<T> {
	/// TODO: DOCUMENT
	pub fn does_match(vec: Vec<T>) -> bool {
		todo!()
	}

	/// TODO: DOCUMENT
	pub fn match_indices(vec: Vec<T>) -> Vec<(usize, usize)> {
		todo!()
	}

	/// TODO: DOCUMENT
	pub fn matches(vec: Vec<T>) -> Vec<Vec<T>> {
		todo!()
	}

	/// TODO: DOCUMENT
	pub fn non_overlapping_match_indices(vec: Vec<T>) -> Vec<(usize, usize)> {
		todo!()
	}

	/// TODO: DOCUMENT
	pub fn non_overlapping_matches(vec: Vec<T>) -> Vec<Vec<T>> {
		todo!()
	}
}