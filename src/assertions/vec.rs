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

use crate::assertions::iterator::{
    check_has_length, check_is_empty, check_is_not_empty, IteratorAssertion,
};
use crate::base::{AssertionApi, AssertionResult, AssertionStrategy, Subject};

/// Trait for vector assertion.
///
/// Compared to [`crate::IteratorAssertion`], [`VecAssertion`] simplifies code because it is not
/// needed to take reference of the expected value and to call `vec.iter()` in the actual value.
///
/// ```
/// use assertor::*;
/// assert_that!(vec![1,2,3].iter()).contains(&2);  // IteratorAssertion
/// assert_that!(vec![1,2,3]).contains(2); // VecAssertion
/// ```
///
/// # Example
/// ```
/// use assertor::*;
/// use assertor::VecAssertion;
///
/// assert_that!(Vec::<usize>::new()).is_empty();
/// assert_that!(vec![1,2,3]).has_length(3);
/// assert_that!(vec![1,2,3]).contains(2);
/// assert_that!(vec![1,2,3]).contains_exactly(vec![3,2,1]);
/// assert_that!(vec![1,2,3]).contains_exactly_in_order(vec![1,2,3]);
/// ```
pub trait VecAssertion<'a, S, T, R>
where
    AssertionResult: AssertionStrategy<R>,
    Self: Sized,
{
    /// Checks that the subject contains the element `expected`.
    ///
    /// # Example
    /// ```
    /// use assertor::*;
    /// assert_that!(vec![1, 2, 3]).contains(2);
    /// ```
    fn contains<B>(&self, element: B) -> R
    where
        B: Borrow<T>,
        T: PartialEq + Debug;

    /// Checks that the subject does not contains the `element`.
    ///
    /// # Example
    /// ```
    /// use assertor::*;
    /// assert_that!(vec![1, 2, 3]).does_not_contain(5);
    /// ```
    fn does_not_contain<B>(&self, element: B) -> R
    where
        B: Borrow<T>,
        T: PartialEq + Debug;

    /// Checks that the subject exactly contains elements of `expected_vec`.
    ///
    /// This method doesn't take care of the order. Use
    /// [contains_exactly_in_order](`VecAssertion::contains_exactly_in_order`) to check
    /// elements are in the same order.
    ///
    /// # Example
    /// ```
    /// use assertor::*;
    /// assert_that!(vec![1, 2, 3]).contains_exactly(vec![3, 2, 1]);
    /// ```
    /// ```should_panic
    /// use assertor::*;
    /// assert_that!(vec![1]).contains_exactly(vec![1,2]);
    /// assert_that!(vec![1,2]).contains_exactly(vec![1]);
    /// ```
    fn contains_exactly<B: Borrow<Vec<T>>>(self, expected_vec: B) -> R
    where
        T: PartialEq + Debug;

    /// Checks that the subject exactly contains `expected_vec` in the same order.
    ///
    /// # Example
    /// ```
    /// use assertor::*;
    /// assert_that!(vec![1, 2, 3]).contains_exactly_in_order(vec![1, 2, 3]);
    /// ```
    /// ```should_panic
    /// use assertor::*;
    /// assert_that!(vec![1]).contains_exactly_in_order(vec![1,2]);
    /// assert_that!(vec![1,2]).contains_exactly_in_order(vec![1]);
    /// ```
    fn contains_exactly_in_order<B: Borrow<Vec<T>>>(self, expected_vec: B) -> R
    where
        T: PartialEq + Debug;

    /// Checks that the subject does not contain any element of `elements`.
    ///
    /// # Example
    /// ```
    /// use assertor::*;
    /// assert_that!(vec![1, 2, 3]).does_not_contain_any(vec![0, -5]);
    /// ```
    /// ```should_panic
    /// use assertor::*;
    /// assert_that!(vec![1,2]).does_not_contain_any(vec![1]);
    /// ```
    fn does_not_contain_any<B: Borrow<Vec<T>>>(&self, elements: B) -> R
    where
        T: PartialEq + Debug;

    /// Checks that the subject is empty.
    ///
    /// # Example
    /// ```
    /// use assertor::*;
    /// assert_that!(Vec::<usize>::new()).is_empty();
    /// ```
    fn is_empty(&self) -> R
    where
        T: Debug;

    /// Checks that the subject is not empty.
    ///
    /// # Example
    /// ```
    /// use assertor::*;
    /// assert_that!(vec![1]).is_not_empty();
    /// ```
    fn is_not_empty(&self) -> R
    where
        T: Debug;

    /// Checks that the subject is the given length.
    ///
    /// # Example
    /// ```
    /// use assertor::*;
    /// assert_that!(vec![1, 2, 3]).has_length(3);
    /// ```
    fn has_length(&self, length: usize) -> R;
}

impl<'a, T, R> VecAssertion<'a, Vec<T>, T, R> for Subject<'a, Vec<T>, (), R>
where
    AssertionResult: AssertionStrategy<R>,
{
    #[track_caller]
    fn contains<B>(&self, element: B) -> R
    where
        B: Borrow<T>,
        T: PartialEq + Debug,
    {
        self.new_subject(&self.actual().iter(), None, ())
            .contains(element.borrow())
    }

    #[track_caller]
    fn does_not_contain<B>(&self, element: B) -> R
    where
        B: Borrow<T>,
        T: PartialEq + Debug,
    {
        self.new_owned_subject(self.actual().iter(), None, ())
            .does_not_contain(element.borrow())
    }

    #[track_caller]
    fn contains_exactly<B: Borrow<Vec<T>>>(self, expected_iter: B) -> R
    where
        T: PartialEq + Debug,
    {
        self.new_owned_subject(self.actual().iter(), None, ())
            .contains_exactly(expected_iter.borrow().iter())
    }

    #[track_caller]
    fn contains_exactly_in_order<B: Borrow<Vec<T>>>(self, expected_iter: B) -> R
    where
        T: PartialEq + Debug,
    {
        self.new_owned_subject(self.actual().iter(), None, ())
            .contains_exactly_in_order(expected_iter.borrow().iter())
    }

    #[track_caller]
    fn does_not_contain_any<B: Borrow<Vec<T>>>(&self, elements: B) -> R
    where
        T: PartialEq + Debug,
    {
        self.new_owned_subject(self.actual().iter(), None, ())
            .does_not_contain_any(elements.borrow().iter())
    }

    #[track_caller]
    fn is_empty(&self) -> R
    where
        T: Debug,
    {
        check_is_empty(self.new_result(), self.actual().iter())
    }

    #[track_caller]
    fn is_not_empty(&self) -> R
    where
        T: Debug,
    {
        check_is_not_empty(self.new_result(), self.actual().iter())
    }

    #[track_caller]
    fn has_length(&self, length: usize) -> R {
        check_has_length(self.new_result(), self.actual().iter(), self.expr(), length)
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;

    use super::*;

    #[test]
    fn contains() {
        assert_that!(vec![1, 2, 3]).contains(&3);

        // Failures
        assert_that!(check_that!(vec![1, 2, 3]).contains(&10)).facts_are(vec![
            Fact::new("expected to contain", "10"),
            Fact::new_simple_fact("but did not"),
            Fact::new_multi_value_fact("though it did contain", vec!["1", "2", "3"]),
        ]);
    }

    #[test]
    fn contains_exactly() {
        assert_that!(vec![1, 2, 3]).contains_exactly(vec![1, 2, 3]);
        assert_that!(vec![2, 1, 3]).contains_exactly(vec![1, 2, 3]);
    }

    #[test]
    fn contains_exactly_in_order() {
        assert_that!(vec![1, 2, 3]).contains_exactly_in_order(vec![1, 2, 3]);
        assert_that!(check_that!(vec![2, 1, 3]).contains_exactly_in_order(vec![1, 2, 3])).facts_are(
            vec![
                Fact::new_simple_fact("contents match, but order was wrong"),
                Fact::new_splitter(),
                Fact::new_multi_value_fact("expected", vec!["1", "2", "3"]),
                Fact::new_multi_value_fact("actual", vec!["2", "1", "3"]),
            ],
        )
    }

    #[test]
    fn is_empty() {
        assert_that!(Vec::<usize>::new()).is_empty();

        // Failures
        assert_that!(check_that!(vec![1]).is_empty()).facts_are(vec![
            Fact::new_simple_fact("expected to be empty"),
            Fact::new_splitter(),
            Fact::new_multi_value_fact("actual", vec!["1"]),
        ])
    }

    #[test]
    fn is_not_empty() {
        assert_that!(Vec::<usize>::from([1])).is_not_empty();

        // Failures
        assert_that!(check_that!(Vec::<usize>::new()).is_not_empty()).facts_are(vec![
            Fact::new_simple_fact("expected to be non-empty"),
            Fact::new_splitter(),
            Fact::new("actual", "[]"),
        ])
    }

    #[test]
    fn has_size() {
        assert_that!(vec![1, 2, 3]).has_length(3);
        assert_that!(Vec::<usize>::new()).has_length(0);

        // Failures
        assert_that!(check_that!(Vec::<usize>::new()).has_length(3)).facts_are(vec![
            Fact::new("value of", "Vec::<usize>::new().size()"),
            Fact::new("expected", "3"),
            Fact::new("actual", "0"),
        ]);
    }
}
