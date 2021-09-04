//! Assertor makes test assertions and failure messages more human-readable.
//!
//! Assertor is heavy affected by [Java Truth](https://github.com/google/truth) in terms of API
//! design and error messages.
//!
//! # Example
//! ```
//! use assertor::*;
//!
//! assert_that!("foobarbaz").contains("bar");
//! assert_that!("foobarbaz").ends_with("baz");
//!
//! assert_that!(0.5).with_abs_tol(0.2).is_approx_equal_to(0.6);
//!
//! assert_that!(vec!["a", "b"]).contains("a");
//! assert_that!(vec!["a", "b"]).has_length(2);
//! assert_that!(vec!["a", "b"]).contains_exactly(vec!["a", "b"]);
//!
//! assert_that!(Option::Some("Foo")).has_value("Foo");
//! ```
//! ## Failure cases
//! ```should_panic
//! use assertor::*;
//! assert_that!(vec!["a", "b", "c"]).contains_exactly(vec!["b", "c", "d"]);
//! // missing (1)   : ["d"]
//! // unexpected (1): ["a"]
//! // ---
//! // expected      : ["b", "c", "d"]
//! // actual        : ["a", "b", "c"]
//! ```
#![warn(missing_docs)]

#[cfg(feature = "float")]
extern crate num_traits;

pub use assertions::basic::{ComparableAssertion, EqualityAssertion};
#[cfg(feature = "float")]
pub use assertions::float::FloatAssertion;
pub use assertions::iterator::IteratorAssertion;
pub use assertions::map::MapAssertion;
pub use assertions::option::OptionAssertion;
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
