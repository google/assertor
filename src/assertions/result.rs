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
use std::fmt::Debug;

use crate::base::{AssertionApi, AssertionResult, AssertionStrategy, Subject};

/// Trait for result assertion.
///
/// # Example
/// ```
/// use assertor::*;
///
/// let ok : Result<usize, usize>= Ok(0);
/// let err : Result<usize, usize>= Err(1);
///
/// assert_that!(ok).is_ok();
/// assert_that!(err).is_err();
/// assert_that!(ok).has_ok(0);
/// assert_that!(err).has_err(1);
/// ```
pub trait ResultAssertion<R, OK, ERR> {
    /// Checks that the subject is [`Result::Ok(_)`](`std::result::Result::Ok`).
    fn is_ok(&self) -> R;

    /// Checks that the subject is [`Result::Err(_)`](`std::result::Result::Err`).
    fn is_err(&self) -> R;

    /// Checks that the subject is [`Result::Ok(expected)`](`std::result::Result::Err`).
    fn has_ok<B: Borrow<OK>>(&self, expected: B) -> R
    where
        OK: PartialEq;

    /// Checks that the subject is [`Result::Err(expected)`](`std::result::Result::Err`).
    fn has_err<B: Borrow<ERR>>(&self, expected: B) -> R
    where
        ERR: PartialEq;
}

impl<R, OK: Debug, ERR: Debug> ResultAssertion<R, OK, ERR> for Subject<'_, Result<OK, ERR>, (), R>
where
    AssertionResult: AssertionStrategy<R>,
{
    fn is_ok(&self) -> R {
        if self.actual().is_ok() {
            self.new_result().do_ok()
        } else {
            self.new_result()
                .add_fact("expected", "Result::Err")
                .add_fact("actual", "Result::Ok")
                .add_splitter()
                .add_fact("actual", format!("{:?}", self.actual()))
                .do_fail()
        }
    }

    fn is_err(&self) -> R {
        if self.actual().is_err() {
            self.new_result().do_ok()
        } else {
            self.new_result()
                .add_fact("expected", "Result::Err")
                .add_fact("actual", "Result::Ok")
                .add_splitter()
                .add_fact("actual", format!("{:?}", self.actual()))
                .do_fail()
        }
    }

    fn has_ok<B: Borrow<OK>>(&self, expected: B) -> R
    where
        OK: PartialEq,
    {
        match self.actual() {
            Ok(actual) if actual.eq(expected.borrow()) => self.new_result().do_ok(),
            Ok(actual) => self
                .new_result()
                .add_fact("expected", format!("Ok({:?})", expected.borrow()))
                .add_fact("actual", format!("Ok({:?})", actual))
                .do_fail(),
            Err(actual) => self
                .new_result()
                .add_fact("expected", format!("Ok({:?})", expected.borrow()))
                .add_fact("actual", format!("Err({:?})", actual))
                .do_fail(),
        }
    }

    fn has_err<B: Borrow<ERR>>(&self, expected: B) -> R
    where
        ERR: PartialEq,
    {
        match self.actual() {
            Err(actual) if actual.eq(expected.borrow()) => self.new_result().do_ok(),
            Err(actual) => self
                .new_result()
                .add_fact("expected", format!("Err({:?})", expected.borrow()))
                .add_fact("actual", format!("Err({:?})", actual))
                .do_fail(),
            Ok(actual) => self
                .new_result()
                .add_fact("expected", format!("Err({:?})", expected.borrow()))
                .add_fact("actual", format!("Ok({:?})", actual))
                .do_fail(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;

    use super::*;

    #[test]
    fn is_ok() {
        assert_that!(Result::<_, ()>::Ok(0)).is_ok();
        assert_that!(Result::<_, ()>::Ok("23")).is_ok();
        assert_that!(Result::<_, ()>::Ok(())).is_ok();
    }

    #[test]
    fn is_err() {
        assert_that!(Result::<(), _>::Err(0)).is_err();
        assert_that!(Result::<(), _>::Err("23")).is_err();
        assert_that!(Result::<(), _>::Err(())).is_err();
    }

    #[test]
    fn has_ok() {
        assert_that!(Result::<_, ()>::Ok(0)).has_ok(0);
        assert_that!(Result::<_, ()>::Ok("23")).has_ok("23");
        assert_that!(Result::<_, ()>::Ok(())).has_ok(());

        // Failures
        assert_that!(check_that!(Result::<_, ()>::Ok(0)).has_ok(1)).facts_are(vec![
            Fact::new("expected", "Ok(1)"),
            Fact::new("actual", "Ok(0)"),
        ]);
        assert_that!(check_that!(Result::<(), ()>::Err(())).has_ok(())).facts_are(vec![
            Fact::new("expected", "Ok(())"),
            Fact::new("actual", "Err(())"),
        ]);
        assert_that!(check_that!(Result::<&str, &str>::Err("")).has_ok("")).facts_are(vec![
            Fact::new("expected", r#"Ok("")"#),
            Fact::new("actual", r#"Err("")"#),
        ]);
        assert_that!(check_that!(Result::<&str, &str>::Ok("")).has_ok("expected")).facts_are(vec![
            Fact::new("expected", r#"Ok("expected")"#),
            Fact::new("actual", r#"Ok("")"#),
        ]);
    }

    #[test]
    fn has_err() {
        assert_that!(Result::<(), _>::Err(0)).has_err(0);
        assert_that!(Result::<(), _>::Err("23")).has_err("23");
        assert_that!(Result::<(), _>::Err(())).has_err(());

        // Failures
        assert_that!(check_that!(Result::<(), _>::Err(0)).has_err(1)).facts_are(vec![
            Fact::new("expected", "Err(1)"),
            Fact::new("actual", "Err(0)"),
        ]);
        assert_that!(check_that!(Result::<(), ()>::Ok(())).has_err(())).facts_are(vec![
            Fact::new("expected", "Err(())"),
            Fact::new("actual", "Ok(())"),
        ]);
        assert_that!(check_that!(Result::<&str, &str>::Ok("")).has_err("")).facts_are(vec![
            Fact::new("expected", r#"Err("")"#),
            Fact::new("actual", r#"Ok("")"#),
        ]);
        assert_that!(check_that!(Result::<&str, &str>::Err("")).has_err("expected")).facts_are(
            vec![
                Fact::new("expected", r#"Err("expected")"#),
                Fact::new("actual", r#"Err("")"#),
            ],
        );
    }
}
