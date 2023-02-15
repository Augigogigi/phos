use std::fmt::Debug;

pub trait Matchable: Clone + PartialEq + Debug {}

impl<T: Sized + Clone + PartialEq + Debug> Matchable for T {}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum QuantifierMode {
	Lazy,
	Greedy,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum QuantifierRange {
	Exactly(usize),
	AtLeast(usize),
	AtMost(usize),
	Between(usize, usize),
}

#[derive(Clone, Debug)]
enum PatternFragment<T: Matchable> {
	Any,
	// '*'
	Unit(T),
	Not(Box<PatternFragment<T>>),
	// '!'
	Or(Vec<PatternFragment<T>>),
	// '|'
	And(Vec<PatternFragment<T>>),
	// '&'
	Join(Vec<PatternFragment<T>>),
	// '+'?
	Quantifier(Box<PatternFragment<T>>, QuantifierRange, QuantifierMode),
}

// Base Functions
pub fn any<T: Matchable>() -> Pattern<T> {
	Pattern::from_frag(PatternFragment::Any)
}

pub fn unit<T: Matchable>(value: T) -> Pattern<T> {
	Pattern::from_frag(PatternFragment::Unit(value))
}

impl<T: Matchable> std::ops::Not for Pattern<T> {
	type Output = Pattern<T>;

	fn not(self) -> Self::Output {
		Pattern::from_frag(PatternFragment::Not(Box::new(self.0)))
	}
}

impl<T: Matchable> std::ops::BitOr for Pattern<T> {
	type Output = Pattern<T>;

	fn bitor(self, rhs: Self) -> Self::Output {
		Pattern::from_frag(match (&self.0, &rhs.0) {
			(PatternFragment::Or(frags), PatternFragment::Or(frags2)) => PatternFragment::Or(frags.iter().chain(frags2).cloned().collect()),
			(PatternFragment::Or(frags), frag) => PatternFragment::Or(frags.iter().chain(&vec![frag.clone()]).cloned().collect()),
			(frag, PatternFragment::Or(frags)) => PatternFragment::Or(vec![frag.clone()].iter().chain(frags).cloned().collect()),
			_ => PatternFragment::Or(vec![self.0, rhs.0]),
		})
	}
}

impl<T: Matchable> std::ops::BitAnd for Pattern<T> {
	type Output = Pattern<T>;

	fn bitand(self, rhs: Self) -> Self::Output {
		Pattern::from_frag(match (&self.0, &rhs.0) {
			(PatternFragment::And(frags), PatternFragment::And(frags2)) => PatternFragment::And(frags.iter().chain(frags2).cloned().collect()),
			(PatternFragment::And(frags), frag) => PatternFragment::And(frags.iter().chain(&vec![frag.clone()]).cloned().collect()),
			(frag, PatternFragment::And(frags)) => PatternFragment::And(vec![frag.clone()].iter().chain(frags).cloned().collect()),
			_ => PatternFragment::And(vec![self.0, rhs.0]),
		})
	}
}

impl<T: Matchable> std::ops::Add for Pattern<T> {
	type Output = Pattern<T>;

	fn add(self, rhs: Self) -> Self::Output {
		Pattern::from_frag(match (&self.0, &rhs.0) {
			(PatternFragment::Join(frags), PatternFragment::Join(frags2)) => PatternFragment::Join(frags.iter().chain(frags2).cloned().collect()),
			(PatternFragment::Join(frags), frag) => PatternFragment::Join(frags.iter().chain(&vec![frag.clone()]).cloned().collect()),
			(frag, PatternFragment::Join(frags)) => PatternFragment::Join(vec![frag.clone()].iter().chain(frags).cloned().collect()),
			_ => PatternFragment::Join(vec![self.0, rhs.0]),
		})
	}
}

impl<T: Matchable> Pattern<T> {
	pub fn optional(self) -> Pattern<T> {
		Pattern::from_frag(PatternFragment::Quantifier(Box::new(self.0), QuantifierRange::AtMost(1), QuantifierMode::Lazy))
	}
	pub fn exactly(self, amount: usize) -> Pattern<T> {
		Pattern::from_frag(PatternFragment::Quantifier(Box::new(self.0), QuantifierRange::Exactly(amount), QuantifierMode::Greedy))
	}
	pub fn at_least(self, amount: usize) -> Pattern<T> {
		Pattern::from_frag(PatternFragment::Quantifier(Box::new(self.0), QuantifierRange::AtLeast(amount), QuantifierMode::Greedy))
	}
	pub fn at_most(self, amount: usize) -> Pattern<T> {
		Pattern::from_frag(PatternFragment::Quantifier(Box::new(self.0), QuantifierRange::AtMost(amount), QuantifierMode::Greedy))
	}
	pub fn between(self, minimum_amount: usize, maximum_amount: usize) -> Pattern<T> {
		Pattern::from_frag(PatternFragment::Quantifier(Box::new(self.0), QuantifierRange::Between(minimum_amount, maximum_amount), QuantifierMode::Greedy))
	}
	pub fn lazy(self) -> Pattern<T> {
		if let PatternFragment::Quantifier(frag, range, _) = self.0 {
			Pattern::from_frag(PatternFragment::Quantifier(frag, range, QuantifierMode::Greedy))
		} else {
			panic!(".lazy() should only be applied to Quantifiers!")
		}
	}
}

impl<T: Matchable> PartialEq for Pattern<T> {
	fn eq(&self, other: &Self) -> bool {
		todo!()
	}
}

impl<T: Matchable> PartialOrd for Pattern<T> {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		todo!()
	}
}

#[derive(Clone, Debug)]
pub struct Pattern<T: Matchable>(PatternFragment<T>);

impl<T: Matchable> Pattern<T> {
	fn from_frag(frag: PatternFragment<T>) -> Self {
		Self(frag)
	}
	pub fn new(frag: Pattern<T>) -> Self {
		Self(frag.0)
	}
}