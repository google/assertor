// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
pub use assertions::boolean::BooleanAssertion;
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
mod diff;

/// Module for testing the assertor library itself. Expected to be used by library developers.
#[cfg(any(test, doc, feature = "testing"))]
pub mod testing;
