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

use crate::base::{AssertionApi, AssertionResult, AssertionStrategy, Subject};

/// Trait for boolean assertion.
///
/// # Example
/// ```
/// use assertor::*;
///
/// assert_that!(true).is_true();
/// assert_that!(false).is_false();
/// ```
pub trait BooleanAssertion<R> {
    /// Checks that the subject is equal to `true`.
    fn is_true(&self) -> R;

    /// Checks that the subject is equal to `false`.
    fn is_false(&self) -> R;
}

impl<R> BooleanAssertion<R> for Subject<'_, bool, (), R>
where
    AssertionResult: AssertionStrategy<R>,
{
    fn is_true(&self) -> R {
        if *self.actual() {
            self.new_result().do_ok()
        } else {
            self.new_result()
                .add_simple_fact("expected true")
                .add_simple_fact("but actual was false")
                .do_fail()
        }
    }

    fn is_false(&self) -> R {
        if !self.actual() {
            self.new_result().do_ok()
        } else {
            self.new_result()
                .add_simple_fact("expected false")
                .add_simple_fact("but actual was true")
                .do_fail()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;

    use super::*;

    #[test]
    fn is_true() {
        assert_that!(true).is_true();

        assert_that!(check_that!(false).is_true()).facts_are(vec![
            Fact::new_simple_fact("expected true"),
            Fact::new_simple_fact("but actual was false"),
        ])
    }

    #[test]
    fn is_false() {
        assert_that!(false).is_false();

        assert_that!(check_that!(true).is_false()).facts_are(vec![
            Fact::new_simple_fact("expected false"),
            Fact::new_simple_fact("but actual was true"),
        ])
    }
}
