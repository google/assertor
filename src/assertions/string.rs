use crate::assertions::basic::EqualityAssertion;
use crate::base::{AssertionApi, AssertionResult, ReturnStrategy, Subject};

pub trait StringAssertion<R> {
    fn is_same_string_to<E: Into<String>>(&self, expected: E) -> R;
    fn contains<E: Into<String>>(&self, expected: E) -> R;
}

impl<'s, R> StringAssertion<R> for Subject<'_, String, (), R>
where
    AssertionResult: ReturnStrategy<R>,
{
    fn is_same_string_to<E: Into<String>>(&self, expected: E) -> R {
        let subject: Subject<String, (), R> = self.new_subject(self.actual(), None, ());
        EqualityAssertion::is_equal_to(&subject, expected.into())
    }

    fn contains<E: Into<String>>(&self, expected: E) -> R {
        let expected_str = expected.into();
        if self.actual().contains(&expected_str) {
            self.new_result().do_ok()
        } else {
            self.new_result()
                .add_fact("expected a string that contains", expected_str)
                .add_fact("but was", self.actual())
                .do_fail()
        }
    }
}

impl<'s, R> StringAssertion<R> for Subject<'_, &str, (), R>
where
    AssertionResult: ReturnStrategy<R>,
{
    fn is_same_string_to<E: Into<String>>(&self, expected: E) -> R {
        self.new_owned_subject(self.actual().to_string(), None, ())
            .is_same_string_to(expected)
    }

    fn contains<E: Into<String>>(&self, expected: E) -> R {
        self.new_owned_subject(self.actual().to_string(), None, ())
            .contains(expected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn is_same_string_to() {
        assert_that!("foo").is_same_string_to("foo");
        assert_that!("").is_same_string_to("");
        assert_that!("ninja".to_string()).is_same_string_to("ninja");
        assert_that!("ninja".to_string()).is_same_string_to("ninja".to_string());
        assert_that!(check_that!("ninja").is_same_string_to("bar")).facts_are(vec![
            Fact::new("expected", "\"bar\""),
            Fact::new("actual", "\"ninja\""),
        ]);
    }

    #[test]
    fn contains() {
        assert_that!("foobarbaz").contains("foo");
        assert_that!("foobarbaz").contains("bar");
        assert_that!("foobarbaz").contains("baz");
        assert_that!("foobarbaz").contains("b");

        assert_that!(check_that!("foo").contains("baz")).facts_are(vec![
            Fact::new("expected a string that contains", "baz"),
            Fact::new("but was", "foo"),
        ])
    }
}
