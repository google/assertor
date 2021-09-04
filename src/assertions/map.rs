use std::borrow::Borrow;
use std::collections::hash_map::Keys;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use crate::assertions::basic::EqualityAssertion;
use crate::assertions::iterator::check_is_empty;
use crate::base::{AssertionApi, AssertionResult, ReturnStrategy, Subject};

pub trait MapAssertion<'a, K, V, R>
where
    AssertionResult: ReturnStrategy<R>,
{
    fn has_length(&self, length: usize) -> R;
    fn is_empty(&self) -> R;
    fn contains_key<BK>(&self, key: BK) -> R
    where
        BK: Borrow<K>,
        K: Eq + Hash + Debug;
    fn key_set(&self) -> Subject<Keys<K, V>, (), R>;
}

impl<'a, K, V, R> MapAssertion<'a, K, V, R> for Subject<'a, HashMap<K, V>, (), R>
where
    AssertionResult: ReturnStrategy<R>,
{
    fn has_length(&self, length: usize) -> R {
        self.new_subject(
            &self.actual().keys().len(),
            Some(format!("{}.len()", self.description_or_expr())),
            (),
        )
        .is_equal_to(length)
    }

    fn is_empty(&self) -> R {
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

    fn key_set(&self) -> Subject<'a, Keys<K, V>, (), R> {
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
            // TODO: fix unstable value order.
            // Fact::new("though it did contain", r#"["c", "a", "b"]"#),
        ]);
        assert_that!(result)
            .fact_keys()
            .contains(&"though it did contain".to_string());
        // Skip test for value because key order is not stable.
    }
}
