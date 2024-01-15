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

use crate::assertions::basic::EqualityAssertion;
use crate::base::{AssertionApi, AssertionResult, AssertionStrategy, Subject};

/// Trait for string assertion.
///
/// # Example
/// ```
/// use assertor::*;
///
/// assert_that!("foobarbaz").is_same_string_to("foobarbaz");
/// assert_that!("foobarbaz").contains("bar");
/// assert_that!("foobarbaz").starts_with("foo");
/// assert_that!("foobarbaz").ends_with("baz");
/// ```
pub trait StringAssertion<R> {
    /// Checks that the subject is same string to `expected`.
    fn is_same_string_to<E: Into<String>>(&self, expected: E) -> R;

    /// Checks that the subject contains `expected`.
    fn contains<E: Into<String>>(&self, expected: E) -> R;

    /// Checks that the subject does not contains `value`.
    fn does_not_contain<E: Into<String>>(&self, value: E) -> R;

    /// Checks that the subject starts with `expected`.
    fn starts_with<E: Into<String>>(&self, expected: E) -> R;

    /// Checks that the subject ends with `expected`.
    fn ends_with<E: Into<String>>(&self, expected: E) -> R;
}

impl<R> StringAssertion<R> for Subject<'_, String, (), R>
where
    AssertionResult: AssertionStrategy<R>,
{
    #[track_caller]
    fn is_same_string_to<E: Into<String>>(&self, expected: E) -> R {
        let subject: Subject<String, (), R> = self.new_subject(self.actual(), None, ());
        EqualityAssertion::is_equal_to(&subject, expected.into())
    }

    #[track_caller]
    fn contains<E: Into<String>>(&self, expected: E) -> R {
        let expected_str = expected.into();
        if self.actual().contains(&expected_str) {
            self.new_result().do_ok()
        } else {
            self.new_result()
                .add_fact("expected a string that contains", expected_str)
                .add_fact("but was", self.actual())
                .do_fail()
        }
    }

    #[track_caller]
    fn does_not_contain<E: Into<String>>(&self, value: E) -> R {
        let expected_str = value.into();
        if self.actual().contains(&expected_str) {
            self.new_result()
                .add_fact("expected a string to not contain", expected_str)
                .add_fact("but was", self.actual())
                .do_fail()
        } else {
            self.new_result().do_ok()
        }
    }

    #[track_caller]
    fn starts_with<E: Into<String>>(&self, expected: E) -> R {
        let expected_str = expected.into();
        if self.actual().starts_with(&expected_str) {
            self.new_result().do_ok()
        } else {
            self.new_result()
                .add_fact("expected a string that starts with", expected_str)
                .add_fact("but was", self.actual())
                .do_fail()
        }
    }

    #[track_caller]
    fn ends_with<E: Into<String>>(&self, expected: E) -> R {
        let expected_str = expected.into();
        if self.actual().ends_with(&expected_str) {
            self.new_result().do_ok()
        } else {
            self.new_result()
                .add_fact("expected a string that ends with", expected_str)
                .add_fact("but was", self.actual())
                .do_fail()
        }
    }
}

impl<R> StringAssertion<R> for Subject<'_, &str, (), R>
where
    AssertionResult: AssertionStrategy<R>,
{
    fn is_same_string_to<E: Into<String>>(&self, expected: E) -> R {
        self.new_owned_subject(self.actual().to_string(), None, ())
            .is_same_string_to(expected)
    }

    fn contains<E: Into<String>>(&self, expected: E) -> R {
        self.new_owned_subject(self.actual().to_string(), None, ())
            .contains(expected)
    }

    fn does_not_contain<E: Into<String>>(&self, value: E) -> R {
        self.new_owned_subject(self.actual().to_string(), None, ())
            .does_not_contain(value)
    }

    fn starts_with<E: Into<String>>(&self, expected: E) -> R {
        self.new_owned_subject(self.actual().to_string(), None, ())
            .starts_with(expected)
    }

    fn ends_with<E: Into<String>>(&self, expected: E) -> R {
        self.new_owned_subject(self.actual().to_string(), None, ())
            .ends_with(expected)
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;

    use super::*;

    #[test]
    fn is_same_string_to() {
        assert_that!("foo").is_same_string_to("foo");
        assert_that!("").is_same_string_to("");
        assert_that!("ninja".to_string()).is_same_string_to("ninja");
        assert_that!("ninja".to_string()).is_same_string_to("ninja".to_string());
        assert_that!(check_that!("ninja").is_same_string_to("bar")).facts_are(vec![
            Fact::new("expected", r#""bar""#),
            Fact::new("actual", r#""ninja""#),
        ]);
    }

    #[test]
    fn starts_with() {
        assert_that!("foobarbaz").starts_with("foo");
        assert_that!(check_that!("foobarbaz").starts_with("baz")).facts_are(vec![
            Fact::new("expected a string that starts with", "baz"),
            Fact::new("but was", "foobarbaz"),
        ])
    }

    #[test]
    fn ends_with() {
        assert_that!("foobarbaz").ends_with("baz");
        assert_that!(check_that!("foobarbaz").ends_with("foo")).facts_are(vec![
            Fact::new("expected a string that ends with", "foo"),
            Fact::new("but was", "foobarbaz"),
        ])
    }

    #[test]
    fn contains() {
        assert_that!("foobarbaz").contains("foo");
        assert_that!("foobarbaz").contains("bar");
        assert_that!("foobarbaz").contains("baz");
        assert_that!("foobarbaz").contains("b");

        assert_that!(check_that!("foo").contains("baz")).facts_are(vec![
            Fact::new("expected a string that contains", "baz"),
            Fact::new("but was", "foo"),
        ])
    }

    #[test]
    fn does_not_contain() {
        assert_that!("foobarbaz").does_not_contain("was");
        assert_that!("foobarbaz").does_not_contain("bla");
        assert_that!("foobarbaz").does_not_contain("up");
        assert_that!("foobarbaz").does_not_contain("x");

        assert_that!(check_that!("foo").does_not_contain("fo")).facts_are(vec![
            Fact::new("expected a string to not contain", "fo"),
            Fact::new("but was", "foo"),
        ])
    }
}
