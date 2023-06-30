// Copyright 2023 Google LLC
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
use crate::diff::iter::{SequenceComparison, SequenceOrderComparison};

/// Trait for iterator assertion.
///
/// # Example
/// ```
/// use assertor::*;
///
/// assert_that!(Vec::<usize>::new()).is_empty();
/// assert_that!(vec![1,2,3].iter()).contains(&2);
/// assert_that!(vec![1,2,3].iter()).contains_exactly(vec![3,2,1].iter());
/// assert_that!(vec![1,2,3].iter()).contains_exactly_in_order(vec![1,2,3].iter());
/// ```
/// ```should_panic
/// use assertor::*;
/// assert_that!(vec![1,2,3].iter()).contains(&4); // <- Panic here
/// // expected to contain  : 4
/// // but did not
/// // though it did contain: [1, 2, 3]
/// ```
/// ```should_panic
/// use assertor::*;
/// assert_that!(vec![1,2,3].iter()).contains_exactly_in_order(vec![3,2,1].iter());  // <- Panic here
/// // contents match, but order was wrong
/// // ---
/// // expected: [3, 2, 1]
/// // actual  : [1, 2, 3]
/// ```
pub trait IteratorAssertion<'a, S, T, R>
where
    AssertionResult: AssertionStrategy<R>,
{
    /// Checks that the subject iterator contains the element `expected`.
    ///
    /// # Example
    /// ```
    /// use assertor::*;
    /// assert_that!(vec![1, 2, 3].iter()).contains(&2);
    /// assert_that!("foobar".chars()).contains(&'a');
    /// ```
    /// ```should_panic
    /// use assertor::*;
    /// assert_that!("foobar".chars()).contains(&'z');
    /// // expected to contain  : 'z'
    /// // but did not
    /// // though it did contain: ['f', 'o', 'o', 'b', 'a', 'r']
    /// ```
    ///
    /// ## Related:
    /// - [`crate::StringAssertion::contains`]
    /// - [`crate::VecAssertion::contains`]
    fn contains<B>(&self, element: B) -> R
    where
        B: Borrow<T>,
        T: PartialEq + Debug;

    /// Checks that the subject iterator does not contains the element `expected`.
    ///
    /// # Example
    /// ```
    /// use assertor::*;
    /// assert_that!(vec![1, 2, 3].iter()).does_not_contain(&5);
    /// assert_that!("foobar".chars()).does_not_contain(&'x');
    /// ```
    /// ```should_panic
    /// use assertor::*;
    /// assert_that!("foobar".chars()).does_not_contain(&'a');
    /// // expected to not contain  : 'a'
    /// // but element was found
    /// // though it did contain: ['f', 'o', 'o', 'b', 'a', 'r']
    /// ```
    ///
    /// ## Related:
    /// - [`crate::StringAssertion::does_not_contain`]
    /// - [`crate::VecAssertion::does_not_contain`]
    fn does_not_contain<B>(&self, element: B) -> R
    where
        B: Borrow<T>,
        T: PartialEq + Debug;

    /// Checks that the subject exactly contains elements of `expected_iter`.
    ///
    /// This method doesn't take care of the order. Use
    /// [contains_exactly_in_order](`IteratorAssertion::contains_exactly_in_order`) to check
    /// elements are in the same order.
    ///
    /// # Example
    /// ```
    /// use assertor::*;
    /// assert_that!(vec![1, 2, 3].iter()).contains_exactly(vec![3, 2, 1].iter());
    /// assert_that!("foobarbaz".chars()).contains_exactly("bazbarfoo".chars());
    /// ```
    /// ```should_panic
    /// use assertor::*;
    /// assert_that!("foobarbaz".chars()).contains_exactly("bazbar".chars());
    /// // unexpected (3): ['f', 'o', 'o']
    /// //---
    /// // expected      : ['b', 'a', 'z', 'b', 'a', 'r']
    /// // actual        : ['f', 'o', 'o', 'b', 'a', 'r', 'b', 'a', 'z']
    /// ```
    fn contains_exactly<EI: Iterator<Item = T> + Clone>(self, expected_iter: EI) -> R
    where
        T: PartialEq + Debug;

    /// Checks that the subject exactly contains elements of `expected_iter` in the same order.
    ///
    /// # Example
    /// ```
    /// use assertor::*;
    /// assert_that!(vec![1, 2, 3].iter()).contains_exactly_in_order(vec![1, 2, 3].iter());
    /// assert_that!("foobarbaz".chars()).contains_exactly_in_order("foobarbaz".chars());
    /// ```
    /// ```should_panic
    /// use assertor::*;
    /// assert_that!("foobarbaz".chars()).contains_exactly_in_order("bazbar".chars());
    /// // unexpected (3): ['f', 'o', 'o']
    /// //---
    /// // expected      : ['b', 'a', 'z', 'b', 'a', 'r']
    /// // actual        : ['f', 'o', 'o', 'b', 'a', 'r', 'b', 'a', 'z']
    /// ```
    /// ```should_panic
    /// use assertor::*;
    /// assert_that!("foobarbaz".chars()).contains_exactly_in_order("bazbarfoo".chars());
    /// // contents match, but order was wrong
    /// // ---
    /// // expected: ['b', 'a', 'z', 'b', 'a', 'r', 'f', 'o', 'o']
    /// // actual  : ['f', 'o', 'o', 'b', 'a', 'r', 'b', 'a', 'z']
    /// ```
    fn contains_exactly_in_order<EI: Iterator<Item = T> + Clone>(self, expected_iter: EI) -> R
    where
        T: PartialEq + Debug;

    /// Checks that the subject contains at least all elements of `expected_iter`.
    ///
    /// This method doesn't take care of the order. Use
    /// [contains_all_of_in_order](`IteratorAssertion::contains_all_of_in_order`) to check
    /// elements are in the same order.
    ///
    /// # Example
    /// ```
    /// use assertor::*;
    /// assert_that!(vec![1, 2, 3].iter()).contains_all_of(vec![2, 3].iter());
    /// assert_that!("foobarbaz".chars()).contains_all_of("bazbar".chars());
    /// ```
    fn contains_all_of<EI: Iterator<Item = T> + Clone>(self, expected_iter: EI) -> R
    where
        T: PartialEq + Debug;

    /// Checks that the subject does not contains any elements of `elements`.
    ///
    /// # Example
    /// ```
    /// use assertor::*;
    /// assert_that!(vec![1, 2, 3].iter()).does_not_contain_any(vec![0, -5].iter());
    /// assert_that!("foobarbaz".chars()).does_not_contain_any("xyw".chars());
    /// ```
    /// ```should_panic
    /// use assertor::*;
    /// assert_that!("foobarbaz".chars()).does_not_contain_any("ab".chars());
    /// // unexpected (2): ['a', 'b']
    /// //---
    /// // expected to contain none of : ['a', 'b']
    /// // but was                     : ['f', 'o', 'o', 'b', 'a', 'r', 'b', 'a', 'z']
    /// ```
    fn does_not_contain_any<EI: Iterator<Item = T> + Clone>(&self, elements: EI) -> R
    where
        T: PartialEq + Debug;

    /// Checks that the subject contains at least all elements of `expected_iter` in the same order.
    ///
    /// # Example
    /// ```
    /// use assertor::*;
    /// assert_that!(vec![1, 2, 3].iter()).contains_all_of_in_order(vec![2, 3].iter());
    /// assert_that!("foobarbaz".chars()).contains_all_of_in_order("obarb".chars());
    /// ```
    fn contains_all_of_in_order<EI: Iterator<Item = T> + Clone>(self, expected_iter: EI) -> R
    where
        T: PartialEq + Debug;

    /// Checks that the subject is empty.
    ///
    /// # Example
    /// ```
    /// use assertor::*;
    /// assert_that!(Vec::<usize>::new()).is_empty();
    /// assert_that!("".chars()).is_empty();
    /// ```
    /// ```should_panic
    /// use assertor::*;
    /// assert_that!(vec![1]).is_empty();
    /// // expected to be empty
    /// // ---
    /// // actual: [1]
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
    /// assert_that!("abc".chars()).is_not_empty();
    /// ```
    /// ```should_panic
    /// use assertor::*;
    /// assert_that!(Vec::<usize>::new()).is_not_empty();
    /// // expected to be non-empty
    /// // ---
    /// // actual: []
    /// ```
    fn is_not_empty(&self) -> R
    where
        T: Debug;

    /// Checks that the subject has the given length.
    ///
    /// # Example
    /// ```
    /// use assertor::*;
    /// assert_that!(vec![1,2,3]).has_length(3);
    /// assert_that!("foobarbaz".chars()).has_length(9);
    /// ```
    /// ```should_panic
    /// use assertor::*;
    /// assert_that!(vec![1,2,3]).has_length(2);
    /// // value of: vec![1,2,3].size()
    /// // expected: 2
    /// // actual  : 3
    /// ```
    fn has_length(&self, length: usize) -> R
    where
        T: Debug;
}

impl<'a, S, T, R> IteratorAssertion<'a, S, T, R> for Subject<'a, S, (), R>
where
    S: Iterator<Item = T> + Clone,
    AssertionResult: AssertionStrategy<R>,
{
    fn contains<B>(&self, element: B) -> R
    where
        B: Borrow<T>,
        T: PartialEq + Debug,
    {
        check_contains(self.new_result(), self.actual().clone(), element.borrow())
    }

    fn does_not_contain<B>(&self, element: B) -> R
    where
        B: Borrow<T>,
        T: PartialEq + Debug,
    {
        check_does_not_contain(self.new_result(), self.actual().clone(), element.borrow())
    }

    fn contains_exactly<EI: Iterator<Item = T> + Clone>(self, expected_iter: EI) -> R
    where
        T: PartialEq + Debug,
    {
        let comparison = SequenceComparison::from_iter(
            self.actual().clone(),
            expected_iter.clone(),
            SequenceOrderComparison::Strict,
        );
        if comparison.contains_exactly() {
            self.new_result().do_ok()
        } else {
            feed_facts_about_item_diff(
                self.new_result(),
                comparison.missing,
                comparison.extra,
                self.actual().clone(),
                expected_iter,
            )
            .do_fail()
        }
    }

    fn contains_exactly_in_order<EI: Iterator<Item = T> + Clone>(self, expected_iter: EI) -> R
    where
        T: PartialEq + Debug,
    {
        let comparison = SequenceComparison::from_iter(
            self.actual().clone(),
            expected_iter.clone(),
            SequenceOrderComparison::Strict,
        );
        if comparison.contains_exactly() && comparison.order_preserved {
            self.new_result().do_ok()
        } else if comparison.contains_exactly() && !comparison.order_preserved {
            self.new_result()
                .add_simple_fact("contents match, but order was wrong")
                .add_splitter()
                .add_fact(
                    "expected",
                    format!("{:?}", expected_iter.collect::<Vec<_>>()),
                )
                .add_fact(
                    "actual",
                    format!("{:?}", self.actual().clone().collect::<Vec<_>>()),
                )
                .do_fail()
        } else {
            feed_facts_about_item_diff(
                self.new_result(),
                comparison.missing,
                comparison.extra,
                self.actual().clone(),
                expected_iter,
            )
            .do_fail()
        }
    }

    fn contains_all_of<EI: Iterator<Item = T> + Clone>(self, expected_iter: EI) -> R
    where
        T: PartialEq + Debug,
    {
        let comparison = SequenceComparison::from_iter(
            self.actual().clone(),
            expected_iter.clone(),
            SequenceOrderComparison::Relative,
        );
        if comparison.contains_all() {
            self.new_result().do_ok()
        } else {
            let missing = comparison.missing;
            self.new_result()
                .add_fact(
                    format!("missing ({})", missing.len()),
                    format!("{:?}", missing),
                )
                // Idea: implement near_miss_obj
                // .add_fact("tough it did contain", format!("{:?}", near_miss_obj))
                .add_splitter()
                .add_fact(
                    "expected to contain at least",
                    format!("{:?}", expected_iter.collect::<Vec<_>>()),
                )
                .add_fact(
                    "but was",
                    format!("{:?}", self.actual().clone().collect::<Vec<_>>()),
                )
                .do_fail()
        }
    }

    fn does_not_contain_any<EI: Iterator<Item = T> + Clone>(&self, elements: EI) -> R
    where
        T: PartialEq + Debug,
    {
        let els = elements.clone().collect::<Vec<T>>();
        // set-like intersection satisfies containment requirement for this case
        // TODO: move to sequence comparison API instead of in-place computation
        let intersection: Vec<T> = self
            .actual()
            .clone()
            .filter(|el| els.contains(el))
            .collect();
        // handle empty iterables
        if intersection.is_empty()
            || self.actual().clone().next().is_none()
            || elements.clone().next().is_none()
        {
            self.new_result().do_ok()
        } else {
            self.new_result()
                .add_fact(
                    format!("found ({})", intersection.len()),
                    format!("{:?}", intersection),
                )
                .add_splitter()
                .add_fact(
                    "expected to contain none of",
                    format!("{:?}", elements.collect::<Vec<_>>()),
                )
                .add_fact(
                    "but was",
                    format!("{:?}", self.actual().clone().collect::<Vec<_>>()),
                )
                .do_fail()
        }
    }

    fn contains_all_of_in_order<EI: Iterator<Item = T> + Clone>(self, expected_iter: EI) -> R
    where
        T: PartialEq + Debug,
    {
        let comparison = SequenceComparison::from_iter(
            self.actual().clone(),
            expected_iter.clone(),
            SequenceOrderComparison::Relative,
        );
        if comparison.contains_all() && comparison.order_preserved {
            self.new_result().do_ok()
        } else if comparison.contains_all() {
            self.new_result()
                .add_simple_fact("required elements were all found, but order was wrong")
                .add_fact(
                    "expected order for required elements",
                    format!("{:?}", expected_iter.collect::<Vec<_>>()),
                )
                .add_fact(
                    "but was",
                    format!("{:?}", self.actual().clone().collect::<Vec<_>>()),
                )
                .do_fail()
        } else {
            let missing = comparison.missing;
            self.new_result()
                .add_fact(
                    format!("missing ({})", missing.len()),
                    format!("{:?}", missing),
                )
                // Idea: implement near_miss_obj
                // .add_fact("tough it did contain", format!("{:?}", near_miss_obj))
                .add_splitter()
                .add_fact(
                    "expected to contain at least",
                    format!("{:?}", expected_iter.collect::<Vec<_>>()),
                )
                .add_fact(
                    "but was",
                    format!("{:?}", self.actual().clone().collect::<Vec<_>>()),
                )
                .do_fail()
        }
    }

    fn is_empty(&self) -> R
    where
        T: Debug,
    {
        check_is_empty(self.new_result(), self.actual().clone())
    }

    fn is_not_empty(&self) -> R
    where
        T: Debug,
    {
        check_is_not_empty(self.new_result(), self.actual().clone())
    }

    fn has_length(&self, length: usize) -> R
    where
        T: Debug,
    {
        check_has_length(
            self.new_result(),
            self.actual().clone(),
            self.expr(),
            length,
        )
    }
}

pub(crate) fn check_is_empty<I, T, R>(assertion_result: AssertionResult, actual_iter: I) -> R
where
    AssertionResult: AssertionStrategy<R>,
    I: Iterator<Item = T> + Clone,
    T: Debug,
{
    if actual_iter.clone().next().is_none() {
        assertion_result.do_ok()
    } else {
        assertion_result
            .add_simple_fact("expected to be empty")
            .add_splitter()
            .add_fact("actual", format!("{:?}", actual_iter.collect::<Vec<_>>()))
            .do_fail()
    }
}

pub(crate) fn check_is_not_empty<I, T, R>(assertion_result: AssertionResult, actual_iter: I) -> R
where
    AssertionResult: AssertionStrategy<R>,
    I: Iterator<Item = T> + Clone,
    T: Debug,
{
    if actual_iter.clone().next().is_none() {
        assertion_result
            .add_simple_fact("expected to be non-empty")
            .add_splitter()
            .add_fact("actual", format!("{:?}", actual_iter.collect::<Vec<_>>()))
            .do_fail()
    } else {
        assertion_result.do_ok()
    }
}

pub(crate) fn check_contains<I, T, R>(
    assertion_result: AssertionResult,
    actual_iter: I,
    element: &T,
) -> R
where
    AssertionResult: AssertionStrategy<R>,
    I: Iterator<Item = T> + Clone,
    T: PartialEq + Debug,
{
    if actual_iter.clone().any(|x| x.eq(element.borrow())) {
        assertion_result.do_ok()
    } else {
        assertion_result
            .add_fact("expected to contain", format!("{:?}", element))
            .add_simple_fact("but did not")
            .add_fact(
                "though it did contain",
                // TODO: better error message
                format!("{:?}", actual_iter.collect::<Vec<_>>()),
            )
            .do_fail()
    }
}

pub(crate) fn check_does_not_contain<I, T, R>(
    assertion_result: AssertionResult,
    actual_iter: I,
    element: &T,
) -> R
where
    AssertionResult: AssertionStrategy<R>,
    I: Iterator<Item = T> + Clone,
    T: PartialEq + Debug,
{
    if actual_iter.clone().any(|x| x.eq(element.borrow())) {
        assertion_result
            .add_fact("expected to not contain", format!("{:?}", element))
            .add_simple_fact("but element was found")
            .add_fact(
                "though it did contain",
                // TODO: better error message
                format!("{:?}", actual_iter.collect::<Vec<_>>()),
            )
            .do_fail()
    } else {
        assertion_result.do_ok()
    }
}

pub(crate) fn feed_facts_about_item_diff<
    T: Debug,
    A: Debug,
    E: Debug,
    IA: Iterator<Item = A>,
    IE: Iterator<Item = E>,
>(
    mut result: AssertionResult,
    missing: Vec<T>,
    extra: Vec<T>,
    actual_iter: IA,
    expected_iter: IE,
) -> AssertionResult {
    let mut splitter = false;
    if !missing.is_empty() {
        result = result.add_fact(
            format!("missing ({})", missing.len()),
            format!("{:?}", missing),
        );
        splitter = true;
    }
    if !extra.is_empty() {
        result = result.add_fact(
            format!("unexpected ({})", extra.len()),
            format!("{:?}", extra),
        );
        splitter = true;
    }
    if splitter {
        result = result.add_splitter();
    }
    result
        .add_fact(
            "expected",
            format!("{:?}", expected_iter.collect::<Vec<_>>()),
        )
        .add_fact("actual", format!("{:?}", actual_iter.collect::<Vec<_>>()))
}

pub(crate) fn check_has_length<I, T, R>(
    assertion_result: AssertionResult,
    actual_iter: I,
    actual_expr: &str,
    length: usize,
) -> R
where
    AssertionResult: AssertionStrategy<R>,
    I: Iterator<Item = T> + Clone,
{
    let actual = actual_iter.count();
    if actual.eq(&length) {
        assertion_result.do_ok()
    } else {
        assertion_result
            .add_fact("value of", format!("{}.size()", actual_expr))
            .add_fact("expected", format!("{}", length))
            .add_fact("actual", format!("{}", actual))
            .do_fail()
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;

    use super::*;

    #[test]
    fn contains() {
        assert_that!(vec![1, 2, 3].iter()).contains(&3);

        // Failures
        assert_that!(check_that!(vec![1, 2, 3].iter()).contains(&10)).facts_are(vec![
            Fact::new("expected to contain", "10"),
            Fact::new_simple_fact("but did not"),
            Fact::new("though it did contain", "[1, 2, 3]"),
        ]);
    }

    #[test]
    fn contains_exactly() {
        assert_that!(vec![1, 2, 3].iter()).contains_exactly(vec![1, 2, 3].iter());
        assert_that!(vec![2, 1, 3].iter()).contains_exactly(vec![1, 2, 3].iter());

        assert_that!(check_that!("foobarbaz".chars()).contains_exactly("bazbar".chars()))
            .facts_are(vec![
                Fact::new("unexpected (3)", r#"['f', 'o', 'o']"#),
                Fact::Splitter,
                Fact::new("expected", r#"['b', 'a', 'z', 'b', 'a', 'r']"#),
                Fact::new("actual", r#"['f', 'o', 'o', 'b', 'a', 'r', 'b', 'a', 'z']"#),
            ]);
    }

    #[test]
    fn contains_exactly_in_order() {
        assert_that!(vec![1, 2, 3].iter()).contains_exactly_in_order(vec![1, 2, 3].iter());
        // failures
        assert_that!(check_that!(vec![1, 2].iter()).contains_exactly_in_order(vec![1, 2, 3].iter()))
            .facts_are(vec![
                Fact::new("missing (1)", "[3]"),
                Fact::new_splitter(),
                Fact::new("expected", "[1, 2, 3]"),
                Fact::new("actual", "[1, 2]"),
            ]);
        assert_that!(check_that!(vec![1, 2, 3].iter()).contains_exactly_in_order(vec![1, 2].iter()))
            .facts_are(vec![
                Fact::new("unexpected (1)", "[3]"),
                Fact::new_splitter(),
                Fact::new("expected", "[1, 2]"),
                Fact::new("actual", "[1, 2, 3]"),
            ]);
        assert_that!(check_that!(vec![1, 2].iter()).contains_exactly_in_order(vec![2, 3].iter()))
            .facts_are(vec![
                Fact::new("missing (1)", "[3]"),
                Fact::new("unexpected (1)", "[1]"),
                Fact::new_splitter(),
                Fact::new("expected", "[2, 3]"),
                Fact::new("actual", "[1, 2]"),
            ]);
        assert_that!(
            check_that!(vec![2, 1, 3].iter()).contains_exactly_in_order(vec![1, 2, 3].iter())
        )
        .facts_are(vec![
            Fact::new_simple_fact("contents match, but order was wrong"),
            Fact::new_splitter(),
            Fact::new("expected", "[1, 2, 3]"),
            Fact::new("actual", "[2, 1, 3]"),
        ])
    }

    #[test]
    fn contains_at_least() {
        assert_that!(vec![1, 2, 3].iter()).contains_all_of(vec![].iter());
        assert_that!(vec![1, 2, 3].iter()).contains_all_of(vec![1, 2].iter());
        assert_that!(vec![1, 2, 3].iter()).contains_all_of(vec![2, 3].iter());
        assert_that!(vec![1, 2, 3].iter()).contains_all_of(vec![1, 2, 3].iter());

        // Failures
        assert_that!(check_that!(vec![1, 2, 3].iter()).contains_all_of(vec![3, 4].iter()))
            .facts_are(vec![
                Fact::new("missing (1)", "[4]"),
                Fact::new_splitter(),
                Fact::new("expected to contain at least", "[3, 4]"),
                Fact::new("but was", "[1, 2, 3]"),
            ])
    }

    #[test]
    fn contains_at_least_in_order() {
        assert_that!(vec![1, 2, 3].iter()).contains_all_of_in_order(vec![].iter());
        assert_that!(vec![1, 2, 3].iter()).contains_all_of_in_order(vec![1, 2].iter());
        assert_that!(vec![1, 2, 3].iter()).contains_all_of_in_order(vec![2, 3].iter());
        assert_that!(vec![1, 2, 3].iter()).contains_all_of_in_order(vec![1, 3].iter());
        assert_that!(vec![1, 2, 3].iter()).contains_all_of_in_order(vec![1, 2, 3].iter());

        // Failures
        assert_that!(check_that!(vec![1, 2, 3].iter()).contains_all_of_in_order(vec![3, 4].iter()))
            .facts_are(vec![
                Fact::new("missing (1)", "[4]"),
                Fact::new_splitter(),
                Fact::new("expected to contain at least", "[3, 4]"),
                Fact::new("but was", "[1, 2, 3]"),
            ]);
        assert_that!(
            check_that!(vec![1, 2, 3].iter()).contains_all_of_in_order(vec![3, 2, 1].iter())
        )
        .facts_are(vec![
            Fact::new_simple_fact("required elements were all found, but order was wrong"),
            Fact::new("expected order for required elements", "[3, 2, 1]"),
            Fact::new("but was", "[1, 2, 3]"),
        ]);
        assert_that!(check_that!(vec![1, 2, 3].iter()).contains_all_of_in_order(vec![2, 1].iter()))
            .facts_are(vec![
                Fact::new_simple_fact("required elements were all found, but order was wrong"),
                Fact::new("expected order for required elements", "[2, 1]"),
                Fact::new("but was", "[1, 2, 3]"),
            ]);
    }

    #[test]
    fn is_empty() {
        assert_that!(Vec::<usize>::new().iter()).is_empty();

        // Failures
        assert_that!(check_that!(vec![1].iter()).is_empty()).facts_are(vec![
            Fact::new_simple_fact("expected to be empty"),
            Fact::new_splitter(),
            Fact::new("actual", "[1]"),
        ]);
    }

    #[test]
    fn has_size() {
        assert_that!(vec![1, 2, 3].iter()).has_length(3);
        assert_that!(Vec::<usize>::new().iter()).has_length(0);

        // Failures
        assert_that!(check_that!(Vec::<usize>::new().iter()).has_length(3)).facts_are(vec![
            Fact::new("value of", "Vec::<usize>::new().iter().size()"),
            Fact::new("expected", "3"),
            Fact::new("actual", "0"),
        ]);
    }

    #[test]
    fn does_not_contain() {
        assert_that!(vec![1, 2, 3].iter()).does_not_contain(&5);
        assert_that!(Vec::<usize>::new().iter()).does_not_contain(&0);

        // Failures
        assert_that!(check_that!(vec![1].iter()).does_not_contain(&1)).facts_are(vec![
            Fact::new("expected to not contain", "1"),
            Fact::new_simple_fact("but element was found"),
            Fact::new("though it did contain", "[1]"),
        ]);
    }

    #[test]
    fn does_not_contain_any() {
        assert_that!(vec![1, 2, 3].iter()).does_not_contain_any(vec![4, 5].iter());
        assert_that!(vec![1, 2, 3].iter()).does_not_contain_any(vec![].iter());
        assert_that!(Vec::<usize>::new().iter()).does_not_contain_any(vec![1, 2, 3].iter());
        assert_that!(Vec::<usize>::new().iter()).does_not_contain_any(Vec::<usize>::new().iter());

        // Failures
        assert_that!(check_that!(vec![1, 2, 3].iter()).does_not_contain_any(vec![2, 3].iter()))
            .facts_are(vec![
                Fact::new("found (2)", "[2, 3]"),
                Fact::new_splitter(),
                Fact::new("expected to contain none of", "[2, 3]"),
                Fact::new("but was", "[1, 2, 3]"),
            ]);
    }
}
