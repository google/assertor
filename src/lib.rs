//! Assertor makes test assertions and failure messages more human-readable.
//!
//! Assertor is heavy affected by [Java Truth](https://github.com/google/truth) in terms of API
//! design and error messages.

#[cfg(feature = "float")]
extern crate num_traits;

pub use assertions::basic::{ComparableAssertion, EqualityAssertion};
#[cfg(feature = "float")]
pub use assertions::float::FloatAssertion;
pub use assertions::iterator::IteratorAssertion;
pub use assertions::map::MapAssertion;
pub use assertions::result::ResultAssertion;
pub use assertions::set::SetAssertion;
pub use base::{AssertionResult, Fact, Location, ReturnStrategy, Subject};

mod assertions;
mod base;

#[cfg(any(test, doc, feature = "testing"))]
pub mod testing;
