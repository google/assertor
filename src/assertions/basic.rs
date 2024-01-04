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

use std::borrow::Borrow;
use std::fmt::Debug;

use crate::base::{AssertionApi, AssertionResult, AssertionStrategy, Subject};

/// Trait for equality assertion.
/// # Example
/// ```
/// use assertor::*;
/// assert_that!(1).is_equal_to(1);
/// assert_that!(1).is_not_equal_to(2);
/// ```
pub trait EqualityAssertion<S, R> {
    /// Checks if the subject is equal to `expected`.
    fn is_equal_to<B: Borrow<S>>(&self, expected: B) -> R;

    /// Checks if the subject value is NOT equal to `expected`.
    fn is_not_equal_to<B: Borrow<S>>(&self, expected: B) -> R;
}

impl<S: PartialEq + Debug, R> EqualityAssertion<S, R> for Subject<'_, S, (), R>
where
    AssertionResult: AssertionStrategy<R>,
{
    #[track_caller]
    fn is_equal_to<B: Borrow<S>>(&self, expected: B) -> R {
        if self.actual().eq(expected.borrow()) {
            self.new_result().do_ok()
        } else {
            self.new_result()
                .add_fact("expected", format!("{:?}", expected.borrow()))
                .add_fact("actual", format!("{:?}", self.actual()))
                .do_fail()
        }
    }

    #[track_caller]
    fn is_not_equal_to<B: Borrow<S>>(&self, expected: B) -> R {
        if !self.actual().ne(expected.borrow()) {
            self.new_result().do_fail()
        } else {
            self.new_result().do_ok()
        }
    }
}

/// Trait for comparison assertions.
pub trait ComparableAssertion<S, R> {
    /// Checks that the subject is greater than or equal to `expected`.
    fn is_at_least<B: Borrow<S>>(&self, expected: B) -> R;

    /// Checks that the subject is less than or equal to `expected`.
    fn is_at_most<B: Borrow<S>>(&self, expected: B) -> R;

    /// Checks that the subject is greater than `expected`.
    fn is_greater_than<B: Borrow<S>>(&self, expected: B) -> R;

    /// Checks that the subject is less than `expected`.
    fn is_less_than<B: Borrow<S>>(&self, expected: B) -> R;
}

impl<S: PartialOrd + Debug, R> ComparableAssertion<S, R> for Subject<'_, S, (), R>
where
    AssertionResult: AssertionStrategy<R>,
{
    #[track_caller]
    fn is_at_least<B: Borrow<S>>(&self, expected: B) -> R {
        if self.actual().ge(expected.borrow()) {
            self.new_result().do_ok()
        } else {
            // TODO: write error message
            self.new_result().do_fail()
        }
    }

    #[track_caller]
    fn is_at_most<B: Borrow<S>>(&self, expected: B) -> R {
        if self.actual().le(expected.borrow()) {
            self.new_result().do_ok()
        } else {
            // TODO: write error message
            self.new_result().do_fail()
        }
    }

    #[track_caller]
    fn is_greater_than<B: Borrow<S>>(&self, expected: B) -> R {
        if self.actual().gt(expected.borrow()) {
            self.new_result().do_ok()
        } else {
            // TODO: write error message
            self.new_result().do_fail()
        }
    }

    #[track_caller]
    fn is_less_than<B: Borrow<S>>(&self, expected: B) -> R {
        if self.actual().lt(expected.borrow()) {
            self.new_result().do_ok()
        } else {
            // TODO: write error message
            self.new_result().do_fail()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;

    use super::*;

    #[test]
    fn is_equal_to() {
        assert_that!(1).is_equal_to(1);
        assert_that!(2).is_equal_to(2);
        assert_that!(vec![1]).is_equal_to(vec![1]);

        // failures
    }

    #[test]
    fn is_equal_to_error_message() {
        let result = check_that!(1).is_equal_to(3);

        assert_that!(result).facts_are(vec![Fact::new("expected", "3"), Fact::new("actual", "1")])
    }

    #[test]
    fn is_not_equal_to() {
        assert_that!(1).is_not_equal_to(2);
        assert_that!(2).is_not_equal_to(1);
        assert_that!(vec![1]).is_not_equal_to(vec![]);
        assert_that!(vec![1]).is_not_equal_to(vec![2]);
    }

    #[test]
    fn is_at_least() {
        assert_that!(2).is_at_least(1);
        assert_that!(2).is_at_least(2);
        assert_that!(2_f32).is_at_least(1.);
    }
}
