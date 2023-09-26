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
use std::fmt::{Debug, Formatter};
use std::hash::Hash;

use crate::assertions::basic::EqualityAssertion;
use crate::assertions::iterator::{
    check_contains, check_does_not_contain, check_is_empty, check_is_not_empty,
};
use crate::base::{AssertionApi, AssertionResult, AssertionStrategy, FactStructure, Subject};
use crate::diff::map::{MapComparison, MapValueDiff};
use crate::Fact;

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

    /// Checks that the subject contains only entries from `expected`.
    fn contains_exactly<BM>(&self, expected: BM) -> R
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
                .add_formatted_fact(
                    "expected key to be mapped to value",
                    MapEntry::new(key.borrow(), value.borrow()),
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
                .add_formatted_fact(
                    "expected key to be mapped to value",
                    MapEntry::new(key.borrow(), value.borrow()),
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
                .add_formatted_fact(
                    "expected to not contain entry",
                    MapEntry::new(key.borrow(), value.borrow()),
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
        let expected_map = expected.borrow();
        let diff = MapComparison::from_hash_maps(self.actual(), expected_map);
        if diff.common.len() == expected_map.len() {
            return self.new_result().do_ok();
        }
        let (result, splitter) = feed_missing_entries_facts(
            "at least",
            self.new_result(),
            &diff,
            expected_map.len(),
            false,
        );
        feed_different_values_facts(result, &diff, splitter)
            .0
            .do_fail()
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
                result = result.add_simple_formatted_fact(MapEntry::new(key, value));
            }
            return result.do_fail();
        }
        return self.new_result().do_ok();
    }

    fn contains_exactly<BM>(&self, expected: BM) -> R
    where
        BM: Borrow<HashMap<K, V>>,
        K: Eq + Hash + Debug,
        V: Eq + Debug,
    {
        let expected_map = expected.borrow();
        let diff = MapComparison::from_hash_maps(self.actual(), expected_map);
        if diff.extra.is_empty() && diff.missing.is_empty() && diff.different_values.is_empty() {
            return self.new_result().do_ok();
        }
        let (result, splitter) = feed_missing_entries_facts(
            "exactly",
            self.new_result(),
            &diff,
            expected_map.len(),
            false,
        );
        let (result, splitter) = feed_extra_entries_facts(result, &diff, splitter);
        feed_different_values_facts(result, &diff, splitter)
            .0
            .do_fail()
    }

    fn key_set(&self) -> Subject<Keys<K, V>, (), R> {
        self.new_owned_subject(
            self.actual().keys(),
            Some(format!("{}.keys()", self.description_or_expr())),
            (),
        )
    }
}

fn pluralize<'a>(count: usize, single: &'a str, plural: &'a str) -> &'a str {
    if count == 1 {
        single
    } else {
        plural
    }
}

fn feed_different_values_facts<K: Eq + Hash + Debug, V: Eq + Debug>(
    mut result: AssertionResult,
    diff: &MapComparison<&K, &V>,
    splitter: bool,
) -> (AssertionResult, bool) {
    let has_diffs = !diff.different_values.is_empty();
    if has_diffs {
        if splitter {
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
        if !diff.different_values.is_empty() {
            result = result.add_complete_fact(create_map_value_diff_fact(&diff.different_values));
        }
    }
    (result, has_diffs)
}

fn feed_missing_entries_facts<K: Eq + Hash + Debug, V: Eq + Debug>(
    containment_spec: &str,
    mut result: AssertionResult,
    diff: &MapComparison<&K, &V>,
    expected_length: usize,
    splitter: bool,
) -> (AssertionResult, bool) {
    let has_diffs = !diff.missing.is_empty();
    if has_diffs {
        if splitter {
            result = result.add_splitter();
        }
        result = result
            .add_fact(
                format!(
                    "expected to contain {} {} provided {}",
                    containment_spec,
                    expected_length,
                    pluralize(expected_length, "entry", "entries")
                ),
                format!(
                    "but {} {} not found",
                    diff.missing.len(),
                    pluralize(diff.missing.len(), "entry", "entries")
                ),
            )
            .add_splitter();
        result = result.add_formatted_values_fact(
            format!(
                "{} not found",
                pluralize(diff.missing.len(), "entry was", "entries were")
            ),
            (&diff.missing)
                .into_iter()
                .map(|(k, v)| MapEntry::new(k, v))
                .collect(),
        );
    }
    (result, has_diffs)
}

fn feed_extra_entries_facts<K: Eq + Hash + Debug, V: Eq + Debug>(
    mut result: AssertionResult,
    diff: &MapComparison<&K, &V>,
    splitter: bool,
) -> (AssertionResult, bool) {
    let has_diffs = !diff.extra.is_empty();
    if has_diffs {
        if splitter {
            result = result.add_splitter();
        }
        result = result
            .add_fact(
                format!("expected to not contain additional entries"),
                format!(
                    "but {} additional {} found",
                    diff.extra.len(),
                    pluralize(diff.extra.len(), "entry was", "entries were")
                ),
            )
            .add_splitter();
        result = result.add_formatted_values_fact(
            format!(
                "unexpected {} found",
                pluralize(diff.extra.len(), "entry was", "entries were")
            ),
            (&diff.extra)
                .into_iter()
                .map(|(k, v)| MapEntry::new(k, v))
                .collect(),
        );
    }
    (result, has_diffs)
}

pub(crate) fn create_map_value_diff_fact<K: Eq + Hash + Debug, V: Eq + Debug>(
    value_diffs: &Vec<MapValueDiff<K, V>>,
) -> Fact {
    let mut key_diffs = vec![];
    for MapValueDiff {
        key,
        actual_value: left_value,
        expected_value: right_value,
    } in value_diffs
    {
        key_diffs.push(Box::new(FactStructure::KeyValue {
            key: format!("{:?}", key),
            value: Box::new(FactStructure::Nested {
                inner: vec![
                    (
                        "expected".to_string(),
                        Box::new(FactStructure::Value {
                            formatted_value: format!("{:?}", right_value),
                        }),
                    ),
                    (
                        "actual".to_string(),
                        Box::new(FactStructure::Value {
                            formatted_value: format!("{:?}", left_value),
                        }),
                    ),
                ],
            }),
        }));
    }
    Fact::Structural {
        inner: FactStructure::KeyValue {
            key: "keys were mapped to an unexpected values".to_string(),
            value: Box::new(FactStructure::List { values: key_diffs }),
        },
    }
}

struct MapEntry<'a, K: Debug, V: Debug> {
    key: &'a K,
    value: &'a V,
}

impl<'a, K: Debug, V: Debug> MapEntry<'a, K, V> {
    fn new(key: &'a K, value: &'a V) -> MapEntry<'a, K, V> {
        Self { key, value }
    }
}

impl<'a, K: Debug, V: Debug> Debug for MapEntry<'a, K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?} ⟶ {:?}", self.key, self.value).as_str())
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
            Fact::new_multi_value_fact("actual", vec![r#""a""#]),
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
            Fact::new("expected to contain", r#""not exist""#),
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
            Fact::new("expected key to be mapped to value", r#""not exist" ⟶ "1""#),
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
            Fact::new("expected key to be mapped to value", r#""a" ⟶ "2""#),
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
            Fact::new("expected to not contain entry", r#""a" ⟶ "1""#),
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
            Fact::new_multi_value_fact("entry was not found", vec![r#""not exist" ⟶ "1""#]),
        ]);

        // case 2: mismatched entries
        let result = check_that!(map_abc).contains_at_least(HashMap::from([("c", "5")]));
        assert_that!(result).facts_are_at_least(vec![
            Fact::new(
                "expected to contain the same entries",
                "but found 1 entry that is different",
            ),
            Fact::new_splitter(),
            create_map_value_diff_fact(&vec![MapValueDiff {
                key: "c",
                actual_value: "3",
                expected_value: "5",
            }]),
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
            Fact::new_multi_value_fact("entry was not found", vec![r#""not exist" ⟶ "1""#]),
            Fact::new_splitter(),
            Fact::new(
                "expected to contain the same entries",
                "but found 1 entry that is different",
            ),
            Fact::new_splitter(),
            create_map_value_diff_fact(&vec![MapValueDiff {
                key: "c",
                actual_value: "3",
                expected_value: "5",
            }]),
        ]);
    }

    #[test]
    fn contains_exactly() {
        let mut map_abc: HashMap<&str, &str> = HashMap::new();
        map_abc.insert("a", "1");
        map_abc.insert("b", "2");
        map_abc.insert("c", "3");
        assert_that!(map_abc).contains_exactly(HashMap::from([("a", "1"), ("c", "3"), ("b", "2")]));

        // case 1: missing key
        let result = check_that!(map_abc).contains_exactly(HashMap::from([("not exist", "1")]));
        assert_that!(result).facts_are_at_least(vec![
            Fact::new(
                "expected to contain exactly 1 provided entry",
                "but 1 entry not found",
            ),
            Fact::new_splitter(),
            Fact::new_multi_value_fact("entry was not found", vec![r#""not exist" ⟶ "1""#]),
        ]);

        // case 2: extra key
        let result = check_that!(HashMap::from([
            ("a", "1"),
            ("c", "3"),
            ("b", "2"),
            ("ex", "1")
        ]))
        .contains_exactly(map_abc);
        assert_that!(result).facts_are_at_least(vec![
            Fact::new(
                "expected to not contain additional entries",
                "but 1 additional entry was found",
            ),
            Fact::new_splitter(),
            Fact::new_multi_value_fact("unexpected entry was found", vec![r#""ex" ⟶ "1""#]),
        ]);

        // case 3: mismatched entries
        let result =
            check_that!(HashMap::from([("a", "1")])).contains_at_least(HashMap::from([("a", "2")]));
        assert_that!(result).facts_are_at_least(vec![
            Fact::new(
                "expected to contain the same entries",
                "but found 1 entry that is different",
            ),
            Fact::new_splitter(),
            create_map_value_diff_fact(&vec![MapValueDiff {
                key: "a",
                actual_value: "1",
                expected_value: "2",
            }]),
        ]);

        // case 4: all mismatches
        let result = check_that!(HashMap::from([("a", "1"), ("b", "2")]))
            .contains_exactly(HashMap::from([("a", "2"), ("c", "2")]));
        assert_that!(result).facts_are_at_least(vec![
            Fact::new(
                "expected to contain exactly 2 provided entries",
                "but 1 entry not found",
            ),
            Fact::new_splitter(),
            Fact::new_multi_value_fact("entry was not found", vec![r#""c" ⟶ "2""#]),
            Fact::new_splitter(),
            Fact::new(
                "expected to not contain additional entries",
                "but 1 additional entry was found",
            ),
            Fact::new_splitter(),
            Fact::new_multi_value_fact("unexpected entry was found", vec![r#""b" ⟶ "2""#]),
            Fact::new_splitter(),
            Fact::new(
                "expected to contain the same entries",
                "but found 1 entry that is different",
            ),
            Fact::new_splitter(),
            create_map_value_diff_fact(&vec![MapValueDiff {
                key: "a",
                actual_value: "1",
                expected_value: "2",
            }]),
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
        assert_that!(result).facts_are_at_least(vec![Fact::new_simple_fact(r#""c" ⟶ "3""#)]);
        assert_that!(result).facts_are_at_least(vec![Fact::new_simple_fact(r#""a" ⟶ "1""#)]);
    }
}
