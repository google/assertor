//! Assertor makes test assertions and failure messages more human-readable.
//!
//! Assertor is heavy affected by [Java Truth](https://github.com/google/truth) in terms of API
//! design and error messages.

#![warn(missing_docs)]

#[cfg(feature = "float")]
extern crate num_traits;

pub use assertions::basic::{ComparableAssertion, EqualityAssertion};
#[cfg(feature = "float")]
pub use assertions::float::FloatAssertion;
pub use assertions::iterator::IteratorAssertion;
pub use assertions::map::MapAssertion;
pub use assertions::result::ResultAssertion;
pub use assertions::set::SetAssertion;
pub use assertions::string::StringAssertion;
pub use assertions::vec::VecAssertion;
pub use base::{AssertionResult, AssertionStrategy, Fact, Location, Subject};

mod assertions;
mod base;

// TODO: make this public once API gets stable.
#[cfg(any(test, doc, feature = "testing"))]
pub(crate) mod testing;
