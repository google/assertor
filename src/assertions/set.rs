use std::borrow::Borrow;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

use crate::base::{AssertionApi, AssertionResult, ReturnStrategy, Subject};

pub trait SetAssertion<'a, S, T, R> {
    fn contains<B: Borrow<T>>(&self, expected: B) -> R
    where
        T: PartialEq + Eq + Debug + Hash;
}

impl<'a, T, R> SetAssertion<'a, HashSet<T>, T, R> for Subject<'a, HashSet<T>, (), R>
where
    AssertionResult: ReturnStrategy<R>,
{
    fn contains<B: Borrow<T>>(&self, expected: B) -> R
    where
        T: PartialEq + Eq + Debug + Hash,
    {
        if self.actual().contains(expected.borrow()) {
            self.new_result().do_ok()
        } else {
            self.new_result()
                .add_fact("expected to contain", format!("{:?}", expected.borrow()))
                .add_simple_fact("but did not")
                .add_fact(
                    "though it did contain",
                    // TODO: better error message
                    format!("{:?}", self.actual().iter().collect::<Vec<_>>()),
                )
                .do_fail()
        }
    }
}
#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

    use crate::*;

    use super::*;

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
