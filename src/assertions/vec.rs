use crate::assertions::iterator::{check_has_length, check_is_empty, IteratorAssertion};
use crate::base::{AssertionApi, AssertionResult, ReturnStrategy, Subject};
use std::borrow::Borrow;
use std::fmt::Debug;

pub trait VecAssertion<'a, S, T, R>
where
    AssertionResult: ReturnStrategy<R>,
    Self: Sized,
{
    fn contains<B>(&self, element: B) -> R
    where
        B: Borrow<T>,
        T: PartialEq + Debug;

    fn contains_exactly<B: Borrow<Vec<T>>>(self, expected_iter: B) -> R
    where
        T: PartialEq + Debug;

    fn contains_exactly_in_order<B: Borrow<Vec<T>>>(self, expected_iter: B) -> R
    where
        T: PartialEq + Debug;

    fn is_empty(&self) -> R;

    fn has_length(&self, length: usize) -> R;
}

impl<'a, T, R> VecAssertion<'a, Vec<T>, T, R> for Subject<'a, Vec<T>, (), R>
where
    AssertionResult: ReturnStrategy<R>,
{
    fn contains<B>(&self, element: B) -> R
    where
        B: Borrow<T>,
        T: PartialEq + Debug,
    {
        self.new_subject(&self.actual().iter(), None, ())
            .contains(element.borrow())
    }

    fn contains_exactly<B: Borrow<Vec<T>>>(self, expected_iter: B) -> R
    where
        T: PartialEq + Debug,
    {
        self.new_owned_subject(self.actual().iter(), None, ())
            .contains_exactly(expected_iter.borrow().iter())
    }

    fn contains_exactly_in_order<B: Borrow<Vec<T>>>(self, expected_iter: B) -> R
    where
        T: PartialEq + Debug,
    {
        self.new_owned_subject(self.actual().iter(), None, ())
            .contains_exactly_in_order(expected_iter.borrow().iter())
    }

    fn is_empty(&self) -> R {
        check_is_empty(self.new_result(), self.actual().iter())
    }

    fn has_length(&self, length: usize) -> R {
        check_has_length(self.new_result(), self.actual().iter(), self.expr(), length)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assertions::result::ResultAssertion;
    use crate::assertions::string::StringAssertion;
    use crate::assertions::testing::AssertionResultAssertion;
    use crate::base::Fact;
    use crate::{assert_that, check_that};

    #[test]
    fn contains() {
        assert_that!(vec![1, 2, 3]).contains(&3);

        // Failures
        assert_that!(check_that!(vec![1, 2, 3]).contains(&10))
            .fact_value_for_key("expected to contain")
            .is_same_string_to("10");
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
                Fact::new("actual", "[2, 1, 3]"),
            ],
        )
    }
    #[test]
    fn is_empty() {
        assert_that!(Vec::<usize>::new()).is_empty();

        // Failures
        assert_that!(check_that!(vec![1]).is_empty())
            .facts_are(vec![Fact::new_simple_fact("expected to be empty")])
    }
    #[test]
    fn has_size() {
        assert_that!(vec![1, 2, 3]).has_length(3);
        assert_that!(Vec::<usize>::new()).has_length(0);

        // Failures
        assert_that!(check_that!(Vec::<usize>::new()).has_length(3)).is_err();
    }
}
