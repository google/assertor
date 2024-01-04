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
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

use crate::assertions::iterator::{check_is_empty, IteratorAssertion};
use crate::base::{AssertionApi, AssertionResult, AssertionStrategy, Subject};
use crate::EqualityAssertion;

/// Trait for set assertion.
///
/// # Example
/// ```
/// use assertor::*;
/// use std::collections::HashSet;
///
/// let mut set = HashSet::new();
/// assert_that!(set).is_empty();
///
/// set.insert("a");
/// set.insert("b");
/// set.insert("c");
///
/// assert_that!(set).contains("a");
/// assert_that!(set).has_length(3);
/// ```
/// ```should_panic
/// use assertor::*;
/// use std::collections::HashSet;
///
/// let mut set = HashSet::new();
/// set.insert("a");
/// assert_that!(set).contains("z");
/// // expected to contain  : "z"
/// // but did not
/// // though it did contain: ["a"]
/// ```
pub trait SetAssertion<'a, S, T, R> {
    /// Checks that the subject has the given length.
    fn has_length(&self, length: usize) -> R;

    /// Checks that the subject is empty.
    fn is_empty(&self) -> R
    where
        T: Debug;

    /// Checks that the subject has `expected`.
    fn contains<B: Borrow<T>>(&self, expected: B) -> R
    where
        T: PartialEq + Eq + Debug + Hash;

    /// Checks that the subject does not contain `element`.
    fn does_not_contain<B>(&self, element: B) -> R
    where
        B: Borrow<T>,
        T: PartialEq + Debug;

    /// Checks that the subject does not contain any element of `elements`.
    fn does_not_contain_any<B: Borrow<Vec<T>>>(&self, elements: B) -> R
    where
        T: PartialEq + Debug;
}

impl<'a, T, R> SetAssertion<'a, HashSet<T>, T, R> for Subject<'a, HashSet<T>, (), R>
where
    AssertionResult: AssertionStrategy<R>,
{
    #[track_caller]
    fn has_length(&self, length: usize) -> R {
        self.new_subject(
            &self.actual().len(),
            Some(format!("{}.len()", self.description_or_expr())),
            (),
        )
        .is_equal_to(length)
    }

    #[track_caller]
    fn is_empty(&self) -> R
    where
        T: Debug,
    {
        check_is_empty(self.new_result(), self.actual().iter())
    }

    #[track_caller]
    fn contains<B: Borrow<T>>(&self, expected: B) -> R
    where
        T: PartialEq + Eq + Debug + Hash,
    {
        self.new_owned_subject(self.actual().iter(), None, ())
            .contains(expected.borrow())
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
    fn does_not_contain_any<B: Borrow<Vec<T>>>(&self, elements: B) -> R
    where
        T: PartialEq + Debug,
    {
        self.new_owned_subject(self.actual().iter(), None, ())
            .does_not_contain_any(elements.borrow().iter())
    }
}

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

    use crate::testing::*;

    use super::*;

    #[test]
    fn has_length() {
        assert_that!(HashSet::from_iter(vec![1].iter())).has_length(1);
        assert_that!(HashSet::from_iter(vec![1, 2, 3].iter())).has_length(3);
        assert_that!(check_that!(HashSet::from_iter(vec![1].iter())).has_length(3)).facts_are(
            vec![
                Fact::new("value of", "HashSet::from_iter(vec![1].iter()).len()"),
                Fact::new("expected", "3"),
                Fact::new("actual", "1"),
            ],
        );
    }

    #[test]
    fn is_empty() {
        assert_that!(HashSet::<&usize>::from_iter(vec![].iter())).is_empty();
        assert_that!(check_that!(HashSet::from_iter(vec![1].iter())).is_empty()).facts_are(vec![
            Fact::new_simple_fact("expected to be empty"),
            Fact::new_splitter(),
            Fact::new_multi_value_fact("actual", vec!["1"]),
        ]);
    }

    #[test]
    fn contains() {
        assert_that!(HashSet::from_iter(vec![1, 2, 3].iter())).contains(&3);

        // Failures
        let result = check_that!(HashSet::from_iter(vec![1, 2, 3].iter())).contains(&10);
        assert_that!(result).facts_are_at_least(vec![
            Fact::new("expected to contain", "10"),
            Fact::new_simple_fact("but did not"),
        ]);
        assert_that!(result)
            .fact_keys()
            .contains(&"though it did contain".to_string());
        // Skip test for value because key order is not stable.
    }
}
