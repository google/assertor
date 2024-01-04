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

use crate::base::AssertionApi;
use crate::{AssertionResult, AssertionStrategy, Subject};

/// Trait for option assertion.
///
/// # Example
/// ```
/// use assertor::*;
///
/// assert_that!(Option::Some(1)).has_value(1);
/// assert_that!(Option::Some(1)).is_some();
/// assert_that!(Option::<usize>::None).is_none();
/// ```
pub trait OptionAssertion<'a, T, R>
where
    AssertionResult: AssertionStrategy<R>,
{
    /// Checks the subject is [`Option::None`].
    fn is_none(&self) -> R
    where
        T: PartialEq + Debug;

    /// Checks the subject is [`Option::Some(_)`](`Option::Some`).
    fn is_some(&self) -> R
    where
        T: PartialEq + Debug;

    /// Checks the subject is [`Option::Some(expected)`](`Option::Some`).
    fn has_value<B>(&self, expected: B) -> R
    where
        B: Borrow<T>,
        T: PartialEq + Debug;
}

impl<'a, T, R> OptionAssertion<'a, T, R> for Subject<'a, Option<T>, (), R>
where
    AssertionResult: AssertionStrategy<R>,
{
    #[track_caller]
    fn is_none(&self) -> R
    where
        T: PartialEq + Debug,
    {
        match self.actual() {
            None => self.new_result().do_ok(),
            Some(actual) => self
                .new_result()
                .add_fact("expected", "None")
                .add_fact("actual", format!("Some({:?})", actual))
                .do_fail(),
        }
    }

    #[track_caller]
    fn is_some(&self) -> R
    where
        T: PartialEq + Debug,
    {
        match self.actual() {
            None => self
                .new_result()
                .add_fact("expected", "Some(_)")
                .add_fact("actual", "None")
                .do_fail(),
            Some(_) => self.new_result().do_ok(),
        }
    }

    #[track_caller]
    fn has_value<B>(&self, expected: B) -> R
    where
        B: Borrow<T>,
        T: PartialEq + Debug,
    {
        match self.actual() {
            Some(actual) if expected.borrow().eq(actual) => self.new_result().do_ok(),
            Some(actual) => self
                .new_result()
                .add_fact("expected", format!("Some({:?})", expected.borrow()))
                .add_fact("actual", format!("Some({:?})", actual))
                .do_fail(),
            None => self
                .new_result()
                .add_fact("expected", format!("Some({:?})", expected.borrow()))
                .add_fact("actual", "None")
                .do_fail(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;

    use super::*;

    #[test]
    fn is_none() {
        let none: Option<isize> = Option::None;
        let one: Option<isize> = Option::from(1);
        assert_that!(none).is_none();
        assert_that!(check_that!(one).is_none()).facts_are(vec![
            Fact::new("expected", "None"),
            Fact::new("actual", "Some(1)"),
        ]);
        assert_that!(check_that!(Option::Some("some")).is_none()).facts_are(vec![
            Fact::new("expected", "None"),
            Fact::new("actual", r#"Some("some")"#),
        ]);
    }

    #[test]
    fn is_some() {
        let none: Option<isize> = Option::None;
        let one: Option<isize> = Option::from(1);
        assert_that!(one).is_some();
        assert_that!(check_that!(none).is_some()).facts_are(vec![
            Fact::new("expected", "Some(_)"),
            Fact::new("actual", "None"),
        ]);
    }

    #[test]
    fn has_value() {
        let none: Option<isize> = Option::None;
        let one: Option<isize> = Option::from(1);
        assert_that!(one).has_value(1);
        assert_that!(Option::from("")).has_value("");

        assert_that!(check_that!(none).has_value(1)).facts_are(vec![
            Fact::new("expected", "Some(1)"),
            Fact::new("actual", "None"),
        ]);
        assert_that!(check_that!(one).has_value(2)).facts_are(vec![
            Fact::new("expected", "Some(2)"),
            Fact::new("actual", "Some(1)"),
        ]);
        assert_that!(check_that!(Option::from("1")).has_value("2")).facts_are(vec![
            Fact::new("expected", r#"Some("2")"#),
            Fact::new("actual", r#"Some("1")"#),
        ]);
    }
}
