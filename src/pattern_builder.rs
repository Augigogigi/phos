use super::Matchable;
use super::pattern::Pattern;

/// TODO: DOCUMENT
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PatternBuilderConfig {}

/// TODO: DOCUMENT
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PatternBuilderError {
	LazyCalledOnNonQuantifier,
}

/// A configurable builder for a pattern.
#[derive(Clone, PartialEq, Debug)]
pub struct PatternBuilder<T> {
	config: PatternBuilderConfig,
	spooky: std::marker::PhantomData<T>,
}

impl<T: Matchable> PatternBuilder<T> {
	/// TODO: DOCUMENT
	pub fn from() -> PatternBuilder<T> {
		todo!()
	}

	/// Compile the `PatternBuilder` into a `Pattern`.
	/// Returns an error if the compilation fails for any reason.
	///
	/// # Examples
	///
	/// This fails, as `build()` returned `PatternBuilderError::LazyCalledOnNonQuantifier`.
	/// ```
	/// use phos::pattern_builder::*;
	///
	/// let builder = unit('b') + unit('o') + unit('o').lazy();
	/// let pattern = builder.build().unwrap(); // PANIC
	/// ```
	///
	/// Do this instead:
	/// ```
	/// use phos::pattern_builder::*;
	///
	/// let builder = unit('b') + unit('o') + unit('o').zero_or_more().lazy();
	/// let pattern = builder.build().unwrap(); // OK
	/// ```
	pub fn build(&self) -> Result<Pattern<T>, PatternBuilderError> {
		todo!()
	}
}

impl<T: Matchable> PatternBuilder<T> {
	/// Matches a single unit token:
	/// ```
	/// use phos::pattern_builder::*;
	///
	/// let builder = unit('a') + unit('b');
	/// let pattern = builder.build().unwrap();
	///
	/// assert!(pattern.does_match("ab".chars())); // OK
	/// assert!(pattern.does_match("ba".chars())); // PANIC
	/// ```
	#[inline]
	pub fn unit(unit: T) -> PatternBuilder<T> {
		todo!()
	}

	/// Matches several consecutive unit tokens:
	/// ```
	/// use phos::pattern_builder::*;
	///
	/// let builder = units("ab".chars()) + units("cd".chars());
	/// let pattern = builder.build().unwrap();
	///
	/// assert!(pattern.does_match("abcd".chars())); // OK
	/// assert!(pattern.does_match("cdab".chars())); // PANIC
	/// ```
	#[inline]
	pub fn units<I: IntoIterator<Item=T>>(unit: I) -> PatternBuilder<T> {
		todo!()
	}

	/// Matches the start of a pattern:
	/// ```
	/// use phos::pattern_builder::*;
	///
	/// let builder = start() + units([1, 2, 3]);
	/// let pattern = builder.build().unwrap();
	///
	/// assert!(pattern.does_match([1, 2, 3, 4])); // OK
	/// assert!(pattern.does_match([0, 1, 2, 3])); // PANIC
	/// ```
	#[inline]
	pub fn start() -> PatternBuilder<T> {
		todo!()
	}

	/// Matches the end of a pattern:
	/// ```
	/// use phos::pattern_builder::*;
	///
	/// let builder = units([1, 2, 3]) + end();
	/// let pattern = builder.build().unwrap();
	///
	/// assert!(pattern.does_match([0, 1, 2, 3])); // OK
	/// assert!(pattern.does_match([1, 2, 3, 4])); // PANIC
	/// ```
	#[inline]
	pub fn end() -> PatternBuilder<T> {
		todo!()
	}

	/// Matches exactly `amount` of a token:
	/// ```
	/// use phos::pattern_builder::*;
	///
	/// let builder = (unit('a') + unit('b')).exactly(2);
	/// let pattern = builder.build().unwrap();
	///
	/// assert!(pattern.does_match("abab".chars())); // OK
	/// assert!(pattern.does_match("ababab".chars())); // PANIC
	/// ```
	#[inline]
	pub fn exactly(self, amount: usize) -> PatternBuilder<T> {
		todo!()
	}

	/// Matches at least `amount` of a token:
	/// ```
	/// use phos::pattern_builder::*;
	///
	/// let builder = (unit('a') + unit('b')).at_least(3);
	/// let pattern = builder.build().unwrap();
	///
	/// assert!(pattern.does_match("ababab".chars())); // OK
	/// assert!(pattern.does_match("abab".chars())); // PANIC
	/// ```
	#[inline]
	pub fn at_least(self, amount: usize) -> PatternBuilder<T> {
		todo!()
	}

	/// Matches at most `amount` of a token:
	/// ```
	/// use phos::pattern_builder::*;
	///
	/// let builder = (unit('a') + unit('b')).at_most(2);
	/// let pattern = builder.build().unwrap();
	///
	/// assert!(pattern.does_match("abab".chars())); // OK
	/// assert!(!pattern.does_match("ababab".chars())) // PANIC
	/// ```
	#[inline]
	pub fn at_most(self, amount: usize) -> PatternBuilder<T> {
		todo!()
	}

	/// Matches between `min_amount` and `max_amount` of a token:
	/// ```
	/// use phos::pattern_builder::*;
	///
	/// let builder = (unit('a') + unit('b')).between(2, 3);
	/// let pattern = builder.build().unwrap();
	///
	/// assert!(pattern.does_match("ababab".chars())); // OK
	/// assert!(pattern.does_match("abababab".chars())); // PANIC
	/// ```
	#[inline]
	pub fn between(self, min_amount: usize, max_amount: usize) -> PatternBuilder<T> {
		todo!()
	}

	/// Coverts a greedy quantifier to a lazy quantifier:
	/// ```
	/// use phos::pattern_builder::*;
	///
	/// let builder = unit('a').at_least(2).lazy();
	/// let pattern = builder.build().unwrap(); // OK
	///
	/// let broken_builder = unit('a').lazy();
	/// let broken_pattern = builder.build().unwrap(); // PANIC
	///
	/// assert!(pattern.does_match("aaa".chars())); // OK
	/// assert!(pattern.does_match("a".chars())); // PANIC
	/// ```
	#[inline]
	pub fn lazy(self, amount: usize) -> PatternBuilder<T> {
		todo!()
	}

	/// Matches 0 or 1 of a token:
	/// ```
	/// use phos::pattern_builder::*;
	///
	/// let builder = unit('a').optional() + unit('b');
	/// let pattern = builder.build().unwrap();
	///
	/// assert!(pattern.does_match("b".chars())); // OK
	/// assert!(pattern.does_match("ab".chars())); // OK
	/// assert!(pattern.does_match("aab".chars())); // PANIC
	/// ```
	/// NOTE: This is functionally equivalent to `.at_most(1)`
	#[inline]
	pub fn optional(self) -> PatternBuilder<T> {
		self.at_most(1)
	}

	/// Matches 0 or more of a token:
	/// ```
	/// use phos::pattern_builder::*;
	///
	/// let builder = unit('a').zero_or_more() + unit('b');
	/// let pattern = builder.build().unwrap();
	///
	/// assert!(pattern.does_match("b".chars())); // OK
	/// assert!(pattern.does_match("ab".chars())); // OK
	/// assert!(pattern.does_match("aab".chars())); // OK
	/// ```
	/// NOTE: This is functionally equivalent to `.at_least(0)`
	#[inline]
	pub fn zero_or_more(self) -> PatternBuilder<T> {
		self.at_least(0)
	}

	/// Matches 1 or more of a token:
	/// ```
	/// use phos::pattern_builder::*;
	///
	/// let builder = unit('a').one_or_more() + unit('b');
	/// let pattern = builder.build().unwrap();
	///
	/// assert!(pattern.does_match("ab".chars())); // OK
	/// assert!(pattern.does_match("aab".chars())); // OK
	/// assert!(pattern.does_match("b".chars())); // PANIC
	/// ```
	/// NOTE: This is functionally equivalent to `.at_least(1)`
	#[inline]
	pub fn one_or_more(self) -> PatternBuilder<T> {
		self.at_least(1)
	}
}

/// You can use the `+` operator to join/concatenate some builders:
/// ```
/// use phos::pattern_builder::*;
///
/// let builder = unit('a') + unit('b');
/// let pattern = builder.build().unwrap();
///
/// assert!(pattern.does_match("ab".chars())); // OK
/// assert!(pattern.does_match("ba".chars())); // PANIC
/// ```
impl<T: Matchable> std::ops::Add for PatternBuilder<T> {
	type Output = PatternBuilder<T>;

	#[inline]
	fn add(self, rhs: PatternBuilder<T>) -> Self::Output {
		todo!()
	}
}

/// You can use the `&` operator to logically AND some builders:
/// ```
/// use phos::pattern_builder::*;
///
/// let builder = !unit('b') & !unit('c');
/// let pattern = builder.build().unwrap();
///
/// assert!(pattern.does_match('a')); // OK
/// assert!(pattern.does_match('b')); // PANIC
/// assert!(pattern.does_match('c')); // PANIC
/// ```
impl<T: Matchable> std::ops::BitAnd for PatternBuilder<T> {
	type Output = PatternBuilder<T>;

	#[inline]
	fn bitand(self, rhs: PatternBuilder<T>) -> Self::Output {
		todo!()
	}
}

/// You can use the `|` operator to logically OR some builders:
/// ```
/// use phos::pattern_builder::*;
///
/// let builder = unit('a') | unit('b');
/// let pattern = builder.build().unwrap();
///
/// assert!(pattern.does_match('a')); // OK
/// assert!(pattern.does_match('b')); // OK
/// assert!(pattern.does_match('c')); // PANIC
/// ```
/// (Note: This is only really useful when using a `UnitClass`, which may or may not be in this crate (undecided))
impl<T: Matchable> std::ops::BitOr for PatternBuilder<T> {
	type Output = PatternBuilder<T>;

	#[inline]
	fn bitor(self, rhs: PatternBuilder<T>) -> Self::Output {
		todo!()
	}
}

/// You can use the `!` operator to logically NOT a builder:
/// ```
/// use phos::pattern_builder::*;
///
/// let builder = !unit('a');
/// let pattern = builder.build().unwrap();
///
/// assert!(pattern.does_match('b')); // OK
/// assert!(pattern.does_match('c')); // OK
/// assert!(pattern.does_match('a')); // PANIC
/// ```
impl<T: Matchable> std::ops::Not for PatternBuilder<T> {
	type Output = PatternBuilder<T>;

	#[inline]
	fn not(self) -> Self::Output {
		todo!()
	}
}