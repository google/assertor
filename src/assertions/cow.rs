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

use std::borrow::Cow;

use crate::base::{AssertionApi, AssertionResult, AssertionStrategy, Subject};

/// Trait for Cow assertion.
///
/// # Example
/// ```
/// use std::borrow::Cow;
/// use assertor::*;
///
/// assert_that!(Cow::Borrowed("foobar")).is_borrowed();
/// assert_that!(Cow::<str>::Owned("foobar".to_string())).is_owned();
/// assert_that!(Cow::<str>::Owned("foobar".to_string())).deref().is_same_string_to("foobar");
/// ```
pub trait CowAssertion<T: ?Sized, Y, R>
{
    /// Checks that the subject is [`Cow::Borrowed(_)`](`std::borrow::Cow::Borrowed`).
    fn is_borrowed(&self) -> R;

    /// Checks that the subject is [`Cow::Owned(_)`](`std::borrow::Cow::Owned`).
    fn is_owned(&self) -> R;

    /// Returns a new subject which is the dereferenced value of the subject.
    ///
    /// # Example
    /// ```
    /// use std::borrow::Cow;
    /// use assertor::*;
    ///
    /// let owned: Cow<str> = Cow::Owned("owned".to_string());
    /// let borrowed: Cow<str> = Cow::Borrowed("borrowed");
    /// assert_that!(owned).deref().is_same_string_to("owned");
    /// assert_that!(borrowed).deref().is_same_string_to("borrowed");
    ///
    /// let cow_float_value: Cow<f32> = Cow::Owned(1.23);
    /// assert_that!(cow_float_value).deref().is_approx_equal_to(1.23);
    fn deref(&self) -> Subject<Y, (), R>;
}

impl<'a, T: ?Sized, Y, R> CowAssertion<T, Y, R> for Subject<'a, Cow<'a, T>, (), R>
where
    T: ToOwned<Owned=Y>,
    AssertionResult: AssertionStrategy<R>,
{
    fn is_borrowed(&self) -> R {
        if matches!(self.actual(), Cow::Borrowed(_)) {
            self.new_result().do_ok()
        } else {
            self.new_result().add_simple_fact("expected borrowed, but actual was owned").do_fail()
        }
    }

    fn is_owned(&self) -> R {
        if matches!(self.actual(), Cow::Owned(_)) {
            self.new_result().do_ok()
        } else {
            self.new_result().add_simple_fact("expected owned, but actual was borrowed").do_fail()
        }
    }

    fn deref(&self) -> Subject<Y, (), R> {
        let value = self.actual().as_ref().to_owned();
        self.new_owned_subject(value, Some(format!("{}.deref()", self.description_or_expr())), ())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use crate::testing::CheckThatResultAssertion;

    use super::*;

    #[test]
    fn is_borrowed() {
        assert_that!(Cow::Borrowed("foobar")).is_borrowed();
        assert_that!(check_that!(Cow::<str>::Owned("foobar".to_string())).is_borrowed()).facts_are(
            vec![
                Fact::new_simple_fact("expected borrowed, but actual was owned")
            ]
        );
    }

    #[test]
    fn is_owned() {
        assert_that!(Cow::<str>::Owned("foobar".to_string())).is_owned();
        assert_that!(check_that!(Cow::Borrowed("foobar")).is_owned()).facts_are(
            vec![
                Fact::new_simple_fact("expected owned, but actual was borrowed")
            ]
        );
    }

    #[test]
    fn value() {
        assert_that!(Cow::<str>::Owned("foobar".to_string())).deref().is_same_string_to("foobar");
        assert_that!(Cow::Borrowed("foobar")).deref().is_same_string_to("foobar");

        let owned: Cow<Option<i32>> = Cow::Owned(Some(42));
        assert_that!(check_that!(owned).deref().is_none()).facts_are(vec![
            Fact::new("value of", "owned.deref()"),
            Fact::new("expected", "None"),
            Fact::new("actual", "Some(42)"),
        ]);
    }
}
