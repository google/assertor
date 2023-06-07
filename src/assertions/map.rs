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
use crate::assertions::iterator::{
    check_contains, check_does_not_contain, check_is_empty, check_is_not_empty,
};
use crate::base::{AssertionApi, AssertionResult, AssertionStrategy, Subject};
use crate::diff::map::{MapComparison, MapValueDiff};

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

    /// Checks that the subject is not empty.
    fn is_not_empty(&self) -> R
    where
        K: Debug;

    /// Checks that the subject has the given `key`.
    fn contains_key<BK>(&self, key: BK) -> R
    where
        BK: Borrow<K>,
        K: Eq + Hash + Debug;

    /// Checks that the subject does not have the given `key`.
    fn does_not_contain_key<BK>(&self, key: BK) -> R
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

    /// Checks that the subject does not contain entry with the given `key` and `value`.
    fn does_not_contain_entry<BK, BV>(&self, key: BK, value: BV) -> R
    where
        BK: Borrow<K>,
        BV: Borrow<V>,
        K: Eq + Hash + Debug,
        V: Eq + Debug;

    /// Checks that the subject contains all entries from `expected`.
    fn contains_at_least<BM>(&self, expected: BM) -> R
    where
        BM: Borrow<HashMap<K, V>>,
        K: Eq + Hash + Debug,
        V: Eq + Debug;

    /// Checks that the subject does not contain any entries from `expected`.
    fn does_not_contain_any<BM>(&self, expected: BM) -> R
    where
        BM: Borrow<HashMap<K, V>>,
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

    fn is_not_empty(&self) -> R
    where
        K: Debug,
    {
        check_is_not_empty(self.new_result(), self.actual().keys())
    }

    fn contains_key<BK>(&self, key: BK) -> R
    where
        BK: Borrow<K>,
        K: Eq + Hash + Debug,
    {
        check_contains(self.new_result(), self.actual().keys(), &key.borrow())
    }

    fn does_not_contain_key<BK>(&self, key: BK) -> R
    where
        BK: Borrow<K>,
        K: Eq + Hash + Debug,
    {
        check_does_not_contain(self.new_result(), self.actual().keys(), &key.borrow())
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

    fn does_not_contain_entry<BK, BV>(&self, key: BK, value: BV) -> R
    where
        BK: Borrow<K>,
        BV: Borrow<V>,
        K: Eq + Hash + Debug,
        V: Eq + Debug,
    {
        let actual_value = self.actual().get(key.borrow());
        if Some(value.borrow()) == actual_value {
            self.new_result()
                .add_fact(
                    "expected to not contain entry",
                    format!("{:?} -> {:?}", key.borrow(), value.borrow()),
                )
                .add_simple_fact("but entry was found")
                .add_splitter()
                // TODO: add better representation of the map
                .add_fact(
                    "though it did contain",
                    format!("{:?}", self.actual().keys().collect::<Vec<_>>()),
                )
                .do_fail()
        } else {
            self.new_result().do_ok()
        }
    }

    fn contains_at_least<BM>(&self, expected: BM) -> R
    where
        BM: Borrow<HashMap<K, V>>,
        K: Eq + Hash + Debug,
        V: Eq + Debug,
    {
        fn pluralize<'a>(count: usize, single: &'a str, plural: &'a str) -> &'a str {
            if count == 1 {
                single
            } else {
                plural
            }
        }
        let expected_map = expected.borrow();
        let diff = MapComparison::from_hash_maps(self.actual(), expected_map);
        if diff.common.len() == expected_map.len() {
            return self.new_result().do_ok();
        }
        let mut result = self.new_result();
        if !diff.exclusive_right.is_empty() {
            result = result
                .add_fact(
                    format!(
                        "expected to contain at least {} provided {}",
                        expected_map.len(),
                        pluralize(expected_map.len(), "entry", "entries")
                    ),
                    format!(
                        "but {} {} not found",
                        diff.exclusive_right.len(),
                        pluralize(diff.exclusive_right.len(), "entry", "entries")
                    ),
                )
                .add_splitter();
            for (key, value) in diff.exclusive_right {
                result =
                    result.add_fact("entry was not found", format!("{:?} -> {:?}", key, value));
            }
        }
        if !diff.different_values.is_empty() {
            if !result.facts().is_empty() {
                result = result.add_splitter();
            }
            result = result
                .add_fact(
                    "expected to contain the same entries",
                    format!(
                        "but found {} {} different",
                        diff.different_values.len(),
                        pluralize(
                            diff.different_values.len(),
                            "entry that is",
                            "entries that are"
                        )
                    ),
                )
                .add_splitter();
            for MapValueDiff {
                key,
                left_value,
                right_value,
            } in diff.different_values
            {
                result = result.add_fact(
                    format!("key {:?} was mapped to an unexpected value", key),
                    format!(
                        "expected value {:?}, but found {:?}",
                        right_value, left_value
                    ),
                );
            }
        }
        return result.do_fail();
    }

    fn does_not_contain_any<BM>(&self, expected: BM) -> R
    where
        BM: Borrow<HashMap<K, V>>,
        K: Eq + Hash + Debug,
        V: Eq + Debug,
    {
        let expected_map = expected.borrow();
        let diff = MapComparison::from_hash_maps(self.actual(), expected_map);
        if !diff.common.is_empty() {
            let mut result = self
                .new_result()
                .add_simple_fact(format!("found {} unexpected entries", diff.common.len()))
                .add_splitter();
            for (key, value) in diff.common {
                result = result.add_simple_fact(format!("{:?} -> {:?}", key, value));
            }
            return result.do_fail();
        }
        return self.new_result().do_ok();
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

        // failures
        assert_that!(check_that!(HashMap::from([("a", "b")])).is_empty()).facts_are(vec![
            Fact::new_simple_fact("expected to be empty"),
            Fact::new_splitter(),
            Fact::new("actual", "[\"a\"]"),
        ])
    }

    #[test]
    fn is_not_empty() {
        let non_empty: HashMap<&str, &str> = HashMap::from([("a", "b")]);
        assert_that!(non_empty).is_not_empty();

        // failures
        let empty_map: HashMap<&str, &str> = HashMap::new();
        assert_that!(check_that!(empty_map).is_not_empty()).facts_are(vec![
            Fact::new_simple_fact("expected to be non-empty"),
            Fact::new_splitter(),
            Fact::new("actual", "[]"),
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
            Fact::new("expected to contain", r#""not exist""#),
            Fact::new_simple_fact("but did not"),
        ]);
        assert_that!(result)
            .fact_keys()
            .contains(&"though it did contain".to_string());
        // Skip test for value because key order is not stable.
    }

    #[test]
    fn does_not_contain_key() {
        let mut map_abc: HashMap<&str, &str> = HashMap::new();
        map_abc.insert("a", "1");
        map_abc.insert("b", "2");
        map_abc.insert("c", "3");
        assert_that!(map_abc).does_not_contain_key("x");
        assert_that!(map_abc).does_not_contain_key("y");

        // failures
        let result = check_that!(map_abc).does_not_contain_key("a");
        assert_that!(result).facts_are_at_least(vec![
            Fact::new("expected to not contain", r#""a""#),
            Fact::new_simple_fact("but element was found"),
        ]);
        assert_that!(result)
            .fact_keys()
            .contains(&"though it did contain".to_string());
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

    #[test]
    fn does_not_contain_entry() {
        let mut map_abc: HashMap<&str, &str> = HashMap::new();
        map_abc.insert("a", "1");
        map_abc.insert("b", "2");
        map_abc.insert("c", "3");

        // different key
        assert_that!(map_abc).does_not_contain_entry("x", "1");
        // different value
        assert_that!(map_abc).does_not_contain_entry("a", "2");
        assert_that!(map_abc).does_not_contain_entry("b", "3");
        assert_that!(map_abc).does_not_contain_entry("c", "4");

        // failure
        let result = check_that!(map_abc).does_not_contain_entry("a", "1");
        assert_that!(result).facts_are_at_least(vec![
            Fact::new("expected to not contain entry", "\"a\" -> \"1\""),
            Fact::new_simple_fact("but entry was found"),
            Fact::new_splitter(),
        ]);
        assert_that!(result)
            .fact_keys()
            .contains(&"though it did contain".to_string());
    }

    #[test]
    fn contains_at_least() {
        let mut map_abc: HashMap<&str, &str> = HashMap::new();
        map_abc.insert("a", "1");
        map_abc.insert("b", "2");
        map_abc.insert("c", "3");
        assert_that!(map_abc).contains_at_least(HashMap::from([("a", "1")]));
        assert_that!(map_abc).contains_at_least(HashMap::from([("a", "1"), ("b", "2")]));

        // case 1: missing key
        let result = check_that!(map_abc).contains_at_least(HashMap::from([("not exist", "1")]));
        assert_that!(result).facts_are_at_least(vec![
            Fact::new(
                "expected to contain at least 1 provided entry",
                "but 1 entry not found",
            ),
            Fact::new_splitter(),
            Fact::new("entry was not found", r#""not exist" -> "1""#),
        ]);

        // case 2: mismatched entries
        let result = check_that!(map_abc).contains_at_least(HashMap::from([("c", "5")]));
        assert_that!(result).facts_are_at_least(vec![
            Fact::new(
                "expected to contain the same entries",
                "but found 1 entry that is different",
            ),
            Fact::new_splitter(),
            Fact::new(
                r#"key "c" was mapped to an unexpected value"#,
                r#"expected value "5", but found "3""#,
            ),
        ]);

        // case 3: both mismatched and absent key
        let result =
            check_that!(map_abc).contains_at_least(HashMap::from([("not exist", "1"), ("c", "5")]));
        assert_that!(result).facts_are_at_least(vec![
            Fact::new(
                "expected to contain at least 2 provided entries",
                "but 1 entry not found",
            ),
            Fact::new_splitter(),
            Fact::new("entry was not found", r#""not exist" -> "1""#),
            Fact::new_splitter(),
            Fact::new(
                "expected to contain the same entries",
                "but found 1 entry that is different",
            ),
            Fact::new_splitter(),
            Fact::new(
                r#"key "c" was mapped to an unexpected value"#,
                r#"expected value "5", but found "3""#,
            ),
        ]);
    }

    #[test]
    fn does_not_contain_any() {
        let mut map_abc: HashMap<&str, &str> = HashMap::new();
        map_abc.insert("a", "1");
        map_abc.insert("b", "2");
        map_abc.insert("c", "3");

        assert_that!(map_abc).does_not_contain_any(HashMap::from([
            ("a", "2"),
            ("b", "3"),
            ("x", "1"),
        ]));

        let result = check_that!(map_abc).does_not_contain_any(HashMap::from([
            ("a", "1"),
            ("c", "3"),
            ("x", "g"),
        ]));
        assert_that!(result).facts_are_at_least(vec![
            Fact::new_simple_fact("found 2 unexpected entries"),
            Fact::new_splitter(),
        ]);
        assert_that!(result).facts_are_at_least(vec![Fact::new_simple_fact(r#""c" -> "3""#)]);
        assert_that!(result).facts_are_at_least(vec![Fact::new_simple_fact(r#""a" -> "1""#)]);
    }
}
