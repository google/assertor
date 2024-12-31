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
use std::fmt::{Debug, Formatter};
use std::hash::Hash;

use crate::assertions::basic::EqualityAssertion;
use crate::assertions::iterator::{
    check_contains, check_does_not_contain, check_is_empty, check_is_not_empty,
};
use crate::base::{AssertionApi, AssertionResult, AssertionStrategy, Subject};
use crate::diff::iter::SequenceOrderComparison;
use crate::diff::map::{MapComparison, MapLike, MapValueDiff, OrderedMapLike};

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
pub trait MapAssertion<'a, K: 'a + Eq, V, ML, R>
where
    AssertionResult: AssertionStrategy<R>,
    ML: MapLike<K, V>,
{
    /// Checks that the subject has the given length.
    #[track_caller]
    fn has_length(&self, length: usize) -> R;

    /// Checks that the subject is empty.
    #[track_caller]
    fn is_empty(&self) -> R
    where
        K: Debug;

    /// Checks that the subject is not empty.
    #[track_caller]
    fn is_not_empty(&self) -> R
    where
        K: Debug;

    /// Checks that the subject has the given `key`.
    #[track_caller]
    fn contains_key<BK>(&self, key: BK) -> R
    where
        BK: Borrow<K>,
        K: Eq + Hash + Debug;

    /// Checks that the subject does not have the given `key`.
    #[track_caller]
    fn does_not_contain_key<BK>(&self, key: BK) -> R
    where
        BK: Borrow<K>,
        K: Eq + Hash + Debug;

    /// Checks that the subject has entry with the given `key` and `value`.
    #[track_caller]
    fn contains_entry<BK, BV>(&self, key: BK, value: BV) -> R
    where
        BK: Borrow<K>,
        BV: Borrow<V>,
        K: Eq + Hash + Debug,
        V: Eq + Debug;

    /// Checks that the subject does not contain entry with the given `key` and `value`.
    #[track_caller]
    fn does_not_contain_entry<BK, BV>(&self, key: BK, value: BV) -> R
    where
        BK: Borrow<K>,
        BV: Borrow<V>,
        K: Eq + Hash + Debug,
        V: Eq + Debug;

    /// Checks that the subject contains all entries from `expected`.
    #[track_caller]
    fn contains_at_least<BM: 'a, OML: 'a>(&self, expected: BM) -> R
    where
        K: Eq + Hash + Debug,
        V: Eq + Debug,
        OML: MapLike<K, V> + 'a,
        BM: Borrow<OML> + 'a;

    /// Checks that the subject does not contain any entries from `expected`.
    #[track_caller]
    fn does_not_contain_any<BM: 'a, OML: 'a>(&self, expected: BM) -> R
    where
        K: Eq + Hash + Debug,
        V: Eq + Debug,
        OML: MapLike<K, V> + 'a,
        BM: Borrow<OML> + 'a;

    /// Checks that the subject contains only entries from `expected`.
    #[track_caller]
    fn contains_exactly<BM, OML>(&self, expected: BM) -> R
    where
        K: Eq + Hash + Debug,
        V: Eq + Debug,
        OML: MapLike<K, V> + 'a,
        BM: Borrow<OML> + 'a;

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
    /// ```
    fn key_set<'b>(&'b self) -> Subject<ML::It<'b>, (), R>
    where
        K: 'b;
}

/// Trait for ordered map assertion.
///
/// # Example
/// ```
/// use std::collections::BTreeMap;
/// use assertor::*;
///
/// let mut map = BTreeMap::new();
/// assert_that!(map).is_empty();
///
/// map.insert("one", 1);
/// map.insert("two", 2);
/// map.insert("three", 3);
///
/// assert_that!(map).has_length(3);
/// assert_that!(map).contains_key("one");
/// assert_that!(map).key_set().contains_exactly(vec!["three","two","one"].iter());
/// assert_that!(map).contains_all_of_in_order(BTreeMap::from([("one", 1), ("three", 3)]));
/// ```
pub trait OrderedMapAssertion<'a, K: 'a + Ord + Eq, V, ML, R>:
    MapAssertion<'a, K, V, ML, R>
where
    AssertionResult: AssertionStrategy<R>,
    ML: OrderedMapLike<K, V>,
{
    /// Checks that the subject exactly contains `expected` in the same order.
    #[track_caller]
    fn contains_exactly_in_order<BM, OML>(&self, expected: BM) -> R
    where
        K: Eq + Ord + Debug,
        V: Eq + Debug,
        OML: OrderedMapLike<K, V> + 'a,
        BM: Borrow<OML> + 'a;

    /// Checks that the subject contains at least all elements of `expected` in the same order.
    #[track_caller]
    fn contains_all_of_in_order<BM, OML>(&self, expected: BM) -> R
    where
        K: Eq + Ord + Debug,
        V: Eq + Debug,
        OML: OrderedMapLike<K, V> + 'a,
        BM: Borrow<OML> + 'a;
}

impl<'a, K, V, ML, R> MapAssertion<'a, K, V, ML, R> for Subject<'a, ML, (), R>
where
    AssertionResult: AssertionStrategy<R>,
    K: 'a + Eq,
    ML: MapLike<K, V>,
{
    fn has_length(&self, length: usize) -> R {
        self.new_subject(
            &self.actual().len(),
            Some(format!("{}.len()", self.description_or_expr())),
            (),
        )
        .is_equal_to(length)
    }

    fn is_empty(&self) -> R
    where
        K: Debug,
    {
        check_is_empty(self.new_result(), self.actual().keys().into_iter())
    }

    fn is_not_empty(&self) -> R
    where
        K: Debug,
    {
        check_is_not_empty(self.new_result(), self.actual().keys().into_iter())
    }

    fn contains_key<BK>(&self, key: BK) -> R
    where
        BK: Borrow<K>,
        K: Eq + Hash + Debug,
    {
        check_contains(
            self.new_result(),
            self.actual().keys().into_iter(),
            &key.borrow(),
        )
    }

    fn does_not_contain_key<BK>(&self, key: BK) -> R
    where
        BK: Borrow<K>,
        K: Eq + Hash + Debug,
    {
        check_does_not_contain(
            self.new_result(),
            self.actual().keys().into_iter(),
            &key.borrow(),
        )
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
                    format!("{:?}", self.actual().keys()),
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
                    format!("{:?}", self.actual().keys()),
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
                    format!("{:?}", self.actual().keys()),
                )
                .do_fail()
        } else {
            self.new_result().do_ok()
        }
    }

    fn contains_at_least<BM, OML>(&self, expected: BM) -> R
    where
        K: Eq + Hash + Debug,
        V: Eq + Debug,
        OML: MapLike<K, V> + 'a,
        BM: Borrow<OML> + 'a,
    {
        let expected_map = expected.borrow();
        let diff = MapComparison::from_map_like(self.actual(), expected_map, None);
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

    fn does_not_contain_any<BM: 'a, OML: 'a>(&self, expected: BM) -> R
    where
        K: Eq + Hash + Debug,
        V: Eq + Debug,
        OML: MapLike<K, V> + 'a,
        BM: Borrow<OML> + 'a,
    {
        let expected_map = expected.borrow();
        let diff = MapComparison::from_map_like(self.actual(), expected_map, None);
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

    fn contains_exactly<BM, OML>(&self, expected: BM) -> R
    where
        K: Eq + Hash + Debug,
        V: Eq + Debug,
        OML: MapLike<K, V> + 'a,
        BM: Borrow<OML> + 'a,
    {
        let expected_map = expected.borrow();
        let diff = MapComparison::from_map_like(self.actual(), expected_map, None);
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

    fn key_set<'b>(&'b self) -> Subject<ML::It<'b>, (), R>
    where
        K: 'b,
    {
        self.new_owned_subject(
            self.actual().keys_iter(),
            Some(format!("{}.keys()", self.description_or_expr())),
            (),
        )
    }
}

impl<'a, K, V, ML, R> OrderedMapAssertion<'a, K, V, ML, R> for Subject<'a, ML, (), R>
where
    AssertionResult: AssertionStrategy<R>,
    K: 'a + Eq + Ord,
    ML: OrderedMapLike<K, V>,
{
    fn contains_exactly_in_order<BM, OML>(&self, expected: BM) -> R
    where
        K: Eq + Ord + Debug,
        V: Eq + Debug,
        OML: OrderedMapLike<K, V> + 'a,
        BM: Borrow<OML> + 'a,
    {
        let map_diff = MapComparison::from_map_like(
            self.actual(),
            expected.borrow(),
            Some(SequenceOrderComparison::Strict),
        );
        let (values_assertion_result, values_different) =
            feed_different_values_facts(self.new_result(), &map_diff, false);
        let key_order_comparison = map_diff.key_order_comparison.unwrap();
        let (order_assertion_result, order_ok) = super::iterator::check_contains_exactly_in_order(
            key_order_comparison,
            self.actual().keys().into_iter(),
            expected.borrow().keys().into_iter(),
            values_assertion_result,
        );

        if order_ok && !values_different {
            order_assertion_result.do_ok()
        } else {
            order_assertion_result.do_fail()
        }
    }

    fn contains_all_of_in_order<BM, OML>(&self, expected: BM) -> R
    where
        K: Eq + Ord + Debug,
        V: Eq + Debug,
        OML: OrderedMapLike<K, V> + 'a,
        BM: Borrow<OML> + 'a,
    {
        let map_diff = MapComparison::from_map_like(
            self.actual(),
            expected.borrow(),
            Some(SequenceOrderComparison::Relative),
        );
        let (values_assertion_result, values_different) =
            feed_different_values_facts(self.new_result(), &map_diff, false);
        let key_order_comparison = map_diff.key_order_comparison.unwrap();
        let (order_assertion_result, order_ok) = super::iterator::check_contains_all_of_in_order(
            key_order_comparison,
            self.actual().keys().into_iter(),
            expected.borrow().keys().into_iter(),
            values_assertion_result,
        );

        if order_ok && !values_different {
            order_assertion_result.do_ok()
        } else {
            order_assertion_result.do_fail()
        }
    }
}

fn pluralize<'a>(count: usize, single: &'a str, plural: &'a str) -> &'a str {
    if count == 1 {
        single
    } else {
        plural
    }
}

fn feed_different_values_facts<K: Eq + Debug, V: Eq + Debug>(
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
                        "entries that are",
                    )
                ),
            )
            .add_splitter();
        let mut ordered_diffs: Vec<_> = diff.different_values.iter().collect();
        ordered_diffs.sort_by(|d1, d2| format!("{:?}", d1.key).cmp(&format!("{:?}", d2.key)));
        result = result.add_formatted_values_fact(
            format!(
                "{} mapped to unexpected {}",
                pluralize(diff.different_values.len(), "key was", "keys were"),
                pluralize(diff.different_values.len(), "value", "values")
            ),
            ordered_diffs,
        );
    }
    (result, has_diffs)
}

fn feed_missing_entries_facts<K: Eq + Debug, V: Eq + Debug>(
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

fn feed_extra_entries_facts<K: Eq + Debug, V: Eq + Debug>(
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
                "expected to not contain additional entries".to_string(),
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

impl<K: Debug, V: PartialEq + Debug> Debug for MapValueDiff<&K, &V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                r#"{{ key: {:?}, expected: {:?}, actual: {:?} }}"#,
                self.key, self.actual_value, self.expected_value
            )
            .as_str(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;
    use crate::{assert_that, check_that, Fact, IteratorAssertion, SetAssertion};
    use std::collections::{BTreeMap, HashMap};

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
            Fact::new_multi_value_fact(
                r#"key was mapped to unexpected value"#,
                vec![r#"{ key: "c", expected: "3", actual: "5" }"#],
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
            Fact::new_multi_value_fact("entry was not found", vec![r#""not exist" ⟶ "1""#]),
            Fact::new_splitter(),
            Fact::new(
                "expected to contain the same entries",
                "but found 1 entry that is different",
            ),
            Fact::new_splitter(),
            Fact::new_multi_value_fact(
                r#"key was mapped to unexpected value"#,
                vec![r#"{ key: "c", expected: "3", actual: "5" }"#],
            ),
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
        let result = check_that!(HashMap::from([("a", "1"), ("b", "f")]))
            .contains_at_least(HashMap::from([("a", "2"), ("b", "g")]));
        assert_that!(result).facts_are_at_least(vec![
            Fact::new(
                "expected to contain the same entries",
                "but found 2 entries that are different",
            ),
            Fact::new_splitter(),
            Fact::new_multi_value_fact(
                r#"keys were mapped to unexpected values"#,
                vec![
                    r#"{ key: "a", expected: "1", actual: "2" }"#,
                    r#"{ key: "b", expected: "f", actual: "g" }"#,
                ],
            ),
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
            Fact::new_multi_value_fact(
                r#"key was mapped to unexpected value"#,
                vec![r#"{ key: "a", expected: "1", actual: "2" }"#],
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
        assert_that!(result).facts_are_at_least(vec![Fact::new_simple_fact(r#""c" ⟶ "3""#)]);
        assert_that!(result).facts_are_at_least(vec![Fact::new_simple_fact(r#""a" ⟶ "1""#)]);
    }

    #[test]
    fn supports_any_map() {
        let empty: BTreeMap<String, String> = BTreeMap::new();
        let tree_map = BTreeMap::from([("hello", "sorted_map"), ("world", "in")]);
        assert_that!(tree_map).has_length(2);
        assert_that!(empty).is_empty();
        assert_that!(tree_map).is_not_empty();
        assert_that!(tree_map).contains_key("hello");
        assert_that!(tree_map).does_not_contain_key(&"key");
        assert_that!(tree_map).key_set().contains(&"hello");
        assert_that!(tree_map).contains_entry("hello", "sorted_map");
        assert_that!(tree_map).does_not_contain_entry("hello", "other");
        assert_that!(tree_map).contains_at_least(BTreeMap::from([("world", "in")]));
        assert_that!(tree_map).contains_at_least(HashMap::from([("world", "in")]));
        assert_that!(tree_map)
            .contains_exactly(BTreeMap::from([("hello", "sorted_map"), ("world", "in")]));
        assert_that!(tree_map)
            .contains_exactly(HashMap::from([("hello", "sorted_map"), ("world", "in")]));
        assert_that!(tree_map).does_not_contain_any(BTreeMap::from([("world", "nope")]));
        assert_that!(tree_map).does_not_contain_any(HashMap::from([("world", "nope")]));
    }

    #[test]
    fn contains_exactly_in_order() {
        let tree_map = BTreeMap::from([("hello", "sorted_map"), ("world", "in")]);
        assert_that!(tree_map)
            .contains_exactly_in_order(BTreeMap::from([("hello", "sorted_map"), ("world", "in")]));

        // Wrong value
        let result = check_that!(tree_map)
            .contains_exactly_in_order(BTreeMap::from([("hello", "wrong"), ("world", "in")]));
        assert_that!(result).facts_are_at_least(vec![
            Fact::new(
                "expected to contain the same entries",
                "but found 1 entry that is different",
            ),
            Fact::new_splitter(),
            Fact::new_multi_value_fact(
                r#"key was mapped to unexpected value"#,
                vec![r#"{ key: "hello", expected: "sorted_map", actual: "wrong" }"#],
            ),
        ]);

        // Extra key
        let result = check_that!(tree_map)
            .contains_exactly_in_order(BTreeMap::from([("hello", "sorted_map"), ("was", "at")]));
        assert_that!(result).facts_are(vec![
            Fact::new("missing (1)", r#"["was"]"#),
            Fact::new("unexpected (1)", r#"["world"]"#),
            Fact::new_splitter(),
            Fact::new_multi_value_fact("expected", vec![r#""hello""#, r#""was""#]),
            Fact::new_multi_value_fact("actual", vec![r#""hello""#, r#""world""#]),
        ]);

        // Extra key and wrong value
        let result = check_that!(tree_map)
            .contains_exactly_in_order(BTreeMap::from([("hello", "wrong"), ("was", "at")]));
        assert_that!(result).facts_are(vec![
            Fact::new(
                "expected to contain the same entries",
                "but found 1 entry that is different",
            ),
            Fact::new_splitter(),
            Fact::new_multi_value_fact(
                r#"key was mapped to unexpected value"#,
                vec![r#"{ key: "hello", expected: "sorted_map", actual: "wrong" }"#],
            ),
            Fact::new("missing (1)", r#"["was"]"#),
            Fact::new("unexpected (1)", r#"["world"]"#),
            Fact::new_splitter(),
            Fact::new_multi_value_fact("expected", vec![r#""hello""#, r#""was""#]),
            Fact::new_multi_value_fact("actual", vec![r#""hello""#, r#""world""#]),
        ]);
    }

    #[test]
    fn contains_all_of_in_order() {
        let tree_map = BTreeMap::from([("hello", "sorted_map"), ("lang", "en"), ("world", "in")]);
        assert_that!(tree_map)
            .contains_all_of_in_order(BTreeMap::from([("hello", "sorted_map"), ("world", "in")]));

        // Extra key and wrong value
        let result = check_that!(tree_map)
            .contains_exactly_in_order(BTreeMap::from([("hello", "wrong"), ("ww", "w")]));
        assert_that!(result).facts_are(vec![
            Fact::new(
                "expected to contain the same entries",
                "but found 1 entry that is different",
            ),
            Fact::new_splitter(),
            Fact::new_multi_value_fact(
                r#"key was mapped to unexpected value"#,
                vec![r#"{ key: "hello", expected: "sorted_map", actual: "wrong" }"#],
            ),
            Fact::new("missing (1)", r#"["ww"]"#),
            Fact::new("unexpected (2)", r#"["lang", "world"]"#),
            Fact::new_splitter(),
            Fact::new_multi_value_fact("expected", vec![r#""hello""#, r#""ww""#]),
            Fact::new_multi_value_fact("actual", vec![r#""hello""#, r#""lang""#, r#""world""#]),
        ]);
    }
}
