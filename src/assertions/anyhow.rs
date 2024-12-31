// Copyright 2024 Google LLC
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


use crate::base::{AssertionApi, AssertionResult, AssertionStrategy, Subject};
use crate::StringAssertion;

/// Trait for anyhow error assertion.
///
/// # Example
///
/// ```rust
/// use assertor::*;
/// use anyhow::anyhow;
///
/// fn anyhow_func() -> anyhow::Result<()> {
///     Err(anyhow!("failed to parse something in foobar"))
/// }
///
/// fn test_it() {
///     assert_that!(anyhow_func()).err().has_message("failed to parse something in foobar");
///     assert_that!(anyhow_func()).err().as_string().contains("parse something");
///     assert_that!(anyhow_func()).err().as_string().starts_with("failed");
///     assert_that!(anyhow_func()).err().as_string().ends_with("foobar");
/// }
/// ```
pub trait AnyhowErrorAssertion<R> {
    /// Returns a new `String` subject which is the message of the error.
    ///
    /// Related: [`StringAssertion`](crate::StringAssertion)
    ///
    /// ```
    /// use assertor::*;
    /// use anyhow::anyhow;
    ///
    /// assert_that!(anyhow!("error message")).as_string().is_same_string_to("error message");
    /// assert_that!(anyhow!("error message")).as_string().contains("error");
    ///
    ///
    /// fn some_func() -> anyhow::Result<()> {
    ///    Err(anyhow!("error message"))
    /// }
    /// assert_that!(some_func()).err().as_string().starts_with("error");
    /// assert_that!(some_func()).err().as_string().ends_with("message");
    /// ```
    fn as_string(&self) -> Subject<String, (), R>;

    /// Checks that the error message contains `expected`.
    /// ```
    /// use assertor::*;
    /// use anyhow::anyhow;
    ///
    /// assert_that!(anyhow!("error message")).has_message("error message");
    ///
    /// fn some_func() -> anyhow::Result<()> {
    ///    Err(anyhow!("error message"))
    /// }
    /// assert_that!(some_func()).err().has_message("error message")
    /// ```
    #[track_caller]
    fn has_message<E: Into<String>>(&self, expected: E) -> R;
}

impl<R> AnyhowErrorAssertion<R> for Subject<'_, anyhow::Error, (), R>
    where
        AssertionResult: AssertionStrategy<R>,
{
    fn as_string(&self) -> Subject<String, (), R> {
        let message = self.actual().to_string();
        self.new_owned_subject(message, Some(format!("{}.to_string()", self.description_or_expr())), ())
    }

    fn has_message<E: Into<String>>(&self, expected: E) -> R {
        self.as_string().is_same_string_to(expected)
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;

    use super::*;

    #[test]
    fn as_string() {
        assert_that!(anyhow::Error::msg("error message")).as_string().is_same_string_to("error message");
        assert_that!(anyhow::Error::msg("error message")).as_string().starts_with("error");
        assert_that!(anyhow::Error::msg("error message")).as_string().contains("or");

        assert_that!(check_that!(anyhow::Error::msg("error message")).as_string().is_same_string_to("wrong")).facts_are(
            vec![
                Fact::new("expected", "\"wrong\""),
                Fact::new("actual", "\"error message\""),
            ]
        );
    }

    #[test]
    fn has_message() {
        assert_that!(anyhow::Error::msg("error message")).has_message("error message");
        assert_that!(check_that!(anyhow::Error::msg("error message")).has_message("wrong")).facts_are(
            vec![
                Fact::new("expected", "\"wrong\""),
                Fact::new("actual", "\"error message\""),
            ]
        );
    }
}