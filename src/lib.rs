#[cfg(feature = "float")]
extern crate num_traits;

mod assertions;
mod base;

pub use assertions::basic::{ComparableAssertion, EqualityAssertion};
pub use assertions::float::FloatAssertion;
pub use assertions::iterator::IteratorAssertion;
pub use assertions::map::MapAssertion;
pub use assertions::result::ResultAssertion;
pub use assertions::testing::AssertionResultAssertion;
pub use base::{AssertionResult, CheckThatResult, Fact, Location, ReturnStrategy, Subject};
