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

use crate::assertions::iterator::IteratorAssertion;
use crate::base::{AssertionApi, AssertionResult, AssertionStrategy, Fact, FactStructure, Subject};
use crate::testing::CheckThatResult;

/// Trait for assertions for assertion messages.
///
/// # Example
///
/// ```ignore
/// use assertor::*;
/// use assertor::testing::*;
///
/// assert_that!(check_that!("actual_string").is_same_string_to("expected_string")).facts_are(vec![
///     Fact::new("expected", "expected_string"),
///     Fact::new("actual", "actual_string"),
/// ]);
/// ```
pub trait CheckThatResultAssertion<'a, R> {
    /// Checks that the assertion result contains elements of `facts` in order.
    fn facts_are<B: Borrow<Vec<Fact>>>(&self, facts: B) -> R;

    /// Checks that the assertion result contains elements of `facts` in order.
    fn facts_are_at_least<B: Borrow<Vec<Fact>>>(&self, facts: B) -> R;

    /// Returns the first fact value whose key is equal to `key`.
    fn fact_value_for_key<I: Into<String>>(&self, key: I) -> Subject<String, (), R>;

    /// Returns keys of the assertion messages.
    fn fact_keys(&self) -> Subject<'a, HashSet<&String>, (), R>;
}

fn get_assertion_result<'a, 'o, R>(
    subject: &'o Subject<'a, CheckThatResult, (), R>,
) -> &'o AssertionResult {
    subject
        .actual()
        .as_ref()
        .as_ref()
        // TODO: Improve error message; should have line-no.
        .expect_err("Expected Err but got Ok because this is assertion for error message.")
}

impl<'a, R> CheckThatResultAssertion<'a, R> for Subject<'a, CheckThatResult, (), R>
where
    AssertionResult: AssertionStrategy<R>,
{
    fn facts_are<B: Borrow<Vec<Fact>>>(&self, expected: B) -> R {
        self.new_owned_subject(
            get_assertion_result(self).facts().iter(),
            Some(format!("{}.facts()", self.description_or_expr())),
            (),
        )
        .contains_exactly_in_order(expected.borrow().iter())
    }

    fn facts_are_at_least<B: Borrow<Vec<Fact>>>(&self, facts: B) -> R {
        self.new_owned_subject(
            get_assertion_result(self).facts().iter(),
            Some(format!("{}.facts()", self.description_or_expr())),
            (),
        )
        .contains_all_of_in_order(facts.borrow().iter())
    }

    fn fact_value_for_key<I: Into<String>>(&self, key: I) -> Subject<String, (), R> {
        let key_str = key.into();
        let assertion_result = get_assertion_result(self);
        let value = assertion_result
            .facts()
            .iter()
            .flat_map(|fact| match fact {
                Fact::Structural {
                    inner: FactStructure::KeyValue { key, value },
                } if key.eq(&key_str) => match *(value.clone()) {
                    FactStructure::Value { formatted_value } => Some(formatted_value.to_string()),
                    _ => None,
                },
                _ => None,
            })
            .next()
            .unwrap_or_else(|| {
                panic!(
                    "key `{}` not found in assertion result.\n{:?}",
                    key_str,
                    assertion_result.generate_message()
                )
            })
            .clone();
        self.new_owned_subject(
            value,
            Some(format!("{}.[key={}]", self.description_or_expr(), key_str)),
            (),
        )
    }

    fn fact_keys(&self) -> Subject<HashSet<&String>, (), R> {
        let assertion_result = get_assertion_result(self);
        let keys: HashSet<&String> = assertion_result
            .facts()
            .iter()
            .flat_map(|fact| match fact {
                Fact::Structural {
                    inner: FactStructure::KeyValue { key, .. },
                } => Some(key),
                _ => None,
            })
            .collect();
        self.new_owned_subject(
            keys,
            Some(format!("{}.keys()", self.description_or_expr())),
            (),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use crate::testing::*;

    use super::*;

    trait TestAssertion<'a, S, R> {
        fn is_same_to<B>(&self, expected: B) -> R
        where
            B: Borrow<S>,
            S: PartialEq + Debug;
    }

    impl<'a, S, R> TestAssertion<'a, S, R> for Subject<'a, S, (), R>
    where
        AssertionResult: AssertionStrategy<R>,
    {
        fn is_same_to<B>(&self, expected: B) -> R
        where
            B: Borrow<S>,
            S: PartialEq + Debug,
        {
            match expected.borrow().eq(self.actual().borrow()) {
                true => self.new_result().add_simple_fact("same").do_ok(),
                false => self.new_result().add_simple_fact("not same").do_fail(),
            }
        }
    }

    #[test]
    fn test_assertion() {
        assert_that!("same").is_same_to("same");
        assert_that!(check_that!("actual").is_same_to("expected"))
            .facts_are(vec![Fact::new_simple_fact("not same")]);
    }

    #[test]
    fn facts_are() {
        let failed: CheckThatResult = check_that!("actual").is_same_to("expected");
        let rs = check_that!(failed).facts_are(vec![]);
        println!("{}", format!("{:?}", rs.0.clone().err().unwrap().facts()));
        assert_that!(rs).facts_are(vec![
            Fact::new("value of", r#"failed.facts()"#),
            Fact::new(
                "unexpected (1)",
                r#"[Structural { inner: Value { formatted_value: "not same" } }]"#,
            ),
            Fact::new_splitter(),
            Fact::new_multi_value_fact::<&str, &str>("expected", vec![]),
            Fact::new_multi_value_fact(
                "actual",
                vec![r#"Structural { inner: Value { formatted_value: "not same" } }"#],
            ),
        ]);
    }
}
