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
use std::collections::hash_map::Keys;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use crate::assertions::basic::EqualityAssertion;
use crate::assertions::iterator::check_is_empty;
use crate::base::{AssertionApi, AssertionResult, AssertionStrategy, Subject};

/// Trait for map assertion.
///
/// # Example
/// ```
/// use std::collections::HashMap;
/// use assertor::*;
///
/// let mut map = HashMap::new();
/// assert_that!(map).is_empty();
///
/// map.insert("one", 1);
/// map.insert("two", 2);
/// map.insert("three", 3);
///
/// assert_that!(map).has_length(3);
/// assert_that!(map).contains_key("one");
/// assert_that!(map).key_set().contains_exactly(vec!["three","two","one"].iter());
/// ```
pub trait MapAssertion<'a, K, V, R>
where
    AssertionResult: AssertionStrategy<R>,
{
    /// Checks that the subject has the given length.
    fn has_length(&self, length: usize) -> R;

    /// Checks that the subject is empty.
    fn is_empty(&self) -> R
    where
        K: Debug;

    /// Checks that the subject has the given `key`.
    fn contains_key<BK>(&self, key: BK) -> R
    where
        BK: Borrow<K>,
        K: Eq + Hash + Debug;

    /// Checks that the subject has entry with the given `key` and `value`.
    fn contains_entry<BK, BV>(&self, key: BK, value: BV) -> R
    where
        BK: Borrow<K>,
        BV: Borrow<V>,
        K: Eq + Hash + Debug,
        V: Eq + Debug;

    /// Returns a new subject which is an key set of the subject and which implements
    /// [`crate::IteratorAssertion`].
    ///
    /// # Example
    /// ```
    /// use std::collections::HashMap;
    /// use assertor::*;
    /// use assertor::IteratorAssertion;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("one", 1);
    /// map.insert("two", 2);
    /// map.insert("three", 3);
    ///
    /// assert_that!(map).key_set().contains(&"one");
    /// assert_that!(map).key_set().contains_exactly(vec!["three","two","one"].iter());
    /// assert_that!(map).key_set().contains_all_of(vec!["one", "two"].iter());
    fn key_set(&self) -> Subject<Keys<K, V>, (), R>;
}

impl<'a, K, V, R> MapAssertion<'a, K, V, R> for Subject<'a, HashMap<K, V>, (), R>
where
    AssertionResult: AssertionStrategy<R>,
{
    fn has_length(&self, length: usize) -> R {
        self.new_subject(
            &self.actual().keys().len(),
            Some(format!("{}.len()", self.description_or_expr())),
            (),
        )
        .is_equal_to(length)
    }

    fn is_empty(&self) -> R
    where
        K: Debug,
    {
        check_is_empty(self.new_result(), self.actual().keys())
    }

    fn contains_key<BK>(&self, key: BK) -> R
    where
        BK: Borrow<K>,
        K: Eq + Hash + Debug,
    {
        if self.actual().contains_key(key.borrow()) {
            self.new_result().do_ok()
        } else {
            self.new_result()
                .add_fact("expected to contain key", format!("{:?}", key.borrow()))
                .add_simple_fact("but did not")
                .add_splitter()
                .add_fact(
                    "though it did contain keys",
                    format!("{:?}", self.actual().keys().collect::<Vec<_>>()),
                )
                .do_fail()
        }
    }

    fn contains_entry<BK, BV>(&self, key: BK, value: BV) -> R
    where
        BK: Borrow<K>,
        BV: Borrow<V>,
        K: Eq + Hash + Debug,
        V: Eq + Debug,
    {
        let actual_value = self.actual().get(key.borrow());
        if Some(value.borrow()) == actual_value {
            self.new_result().do_ok()
        } else if actual_value.is_none() {
            self.new_result()
                .add_fact(
                    "expected key to be mapped to value",
                    format!("{:?} -> {:?}", key.borrow(), value.borrow()),
                )
                .add_fact("but key was not found", format!("{:?}", key.borrow()))
                .add_splitter()
                .add_fact(
                    "though it did contain keys",
                    format!("{:?}", self.actual().keys().collect::<Vec<_>>()),
                )
                .do_fail()
        } else {
            self.new_result()
                .add_fact(
                    "expected key to be mapped to value",
                    format!("{:?} -> {:?}", key.borrow(), value.borrow()),
                )
                .add_fact(
                    "but key was mapped to a different value",
                    format!("{:?}", actual_value.unwrap().borrow()),
                )
                .add_splitter()
                .add_fact(
                    "though it did contain keys",
                    format!("{:?}", self.actual().keys().collect::<Vec<_>>()),
                )
                .do_fail()
        }
    }

    fn key_set(&self) -> Subject<Keys<K, V>, (), R> {
        self.new_owned_subject(
            self.actual().keys(),
            Some(format!("{}.keys()", self.description_or_expr())),
            (),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;
    use crate::{assert_that, check_that, Fact, IteratorAssertion, SetAssertion};

    use super::*;

    #[test]
    fn has_length() {
        let map_empty: HashMap<&str, &str> = HashMap::new();
        assert_that!(map_empty).has_length(0);

        let mut map_with_two_entry = HashMap::new();
        map_with_two_entry.insert("1", "y");
        map_with_two_entry.insert("2", "z");
        assert_that!(map_with_two_entry).has_length(2);

        // failures
        assert_that!(check_that!(map_empty).has_length(1)).facts_are(vec![
            Fact::new("value of", "map_empty.len()"),
            Fact::new("expected", "1"),
            Fact::new("actual", "0"),
        ])
    }

    #[test]
    fn is_empty() {
        let map_empty: HashMap<&str, &str> = HashMap::new();
        assert_that!(map_empty).is_empty();

        let mut map_with_two_entry = HashMap::new();
        map_with_two_entry.insert("1", "y");
        map_with_two_entry.insert("2", "z");
        assert_that!(map_with_two_entry).has_length(2);

        // failures
        assert_that!(check_that!(map_empty).has_length(1)).facts_are(vec![
            Fact::new("value of", "map_empty.len()"),
            Fact::new("expected", "1"),
            Fact::new("actual", "0"),
        ])
    }

    #[test]
    fn contains_key() {
        let mut map_abc: HashMap<&str, &str> = HashMap::new();
        map_abc.insert("a", "1");
        map_abc.insert("b", "2");
        map_abc.insert("c", "3");
        assert_that!(map_abc).contains_key("a");
        assert_that!(map_abc).contains_key("b");
        assert_that!(map_abc).contains_key("c");

        // failures
        let result = check_that!(map_abc).contains_key("not exist");
        assert_that!(result).facts_are_at_least(vec![
            Fact::new("expected to contain key", r#""not exist""#),
            Fact::new_simple_fact("but did not"),
            Fact::new_splitter(),
        ]);
        assert_that!(result)
            .fact_keys()
            .contains(&"though it did contain keys".to_string());
        // Skip test for value because key order is not stable.
    }

    #[test]
    fn key_set() {
        let mut map_abc: HashMap<&str, &str> = HashMap::new();
        map_abc.insert("a", "1");
        map_abc.insert("b", "2");
        map_abc.insert("c", "3");
        assert_that!(map_abc).key_set().contains(&"a");
        assert_that!(map_abc).key_set().contains(&"b");
        assert_that!(map_abc).key_set().contains(&"c");

        // failures
        let result = check_that!(map_abc).key_set().contains(&"not exist");
        assert_that!(result).facts_are_at_least(vec![
            Fact::new("value of", "map_abc.keys()"),
            Fact::new("expected to contain", "\"not exist\""),
            Fact::new_simple_fact("but did not"),
            /* TODO: fix unstable value order.
             * Fact::new("though it did contain", r#"["c", "a", "b"]"#), */
        ]);
        assert_that!(result)
            .fact_keys()
            .contains(&"though it did contain".to_string());
        // Skip test for value because key order is not stable.
    }

    #[test]
    fn contains_entry() {
        let mut map_abc: HashMap<&str, &str> = HashMap::new();
        map_abc.insert("a", "1");
        map_abc.insert("b", "2");
        map_abc.insert("c", "3");
        assert_that!(map_abc).contains_entry("a", "1");
        assert_that!(map_abc).contains_entry("b", "2");
        assert_that!(map_abc).contains_entry("c", "3");

        // failures: missing key
        let result = check_that!(map_abc).contains_entry("not exist", "1");
        assert_that!(result).facts_are_at_least(vec![
            Fact::new(
                "expected key to be mapped to value",
                r#""not exist" -> "1""#,
            ),
            Fact::new("but key was not found", r#""not exist""#),
            Fact::new_splitter(),
        ]);
        assert_that!(result)
            .fact_keys()
            .contains(&"though it did contain keys".to_string());
        // Skip test for value because key order is not stable.

        // failures: not equal value
        let result = check_that!(map_abc).contains_entry("a", "2");
        assert_that!(result).facts_are_at_least(vec![
            Fact::new("expected key to be mapped to value", r#""a" -> "2""#),
            Fact::new("but key was mapped to a different value", r#""1""#),
            Fact::new_splitter(),
        ]);
        assert_that!(result)
            .fact_keys()
            .contains(&"though it did contain keys".to_string());
        // Skip test for value because key order is not stable.
    }
}
