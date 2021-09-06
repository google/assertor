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

pub use crate::assertions::testing::CheckThatResultAssertion;
pub use crate::{assert_that, check_that, Fact};
use crate::{AssertionResult, AssertionStrategy};

/// *Only for library developers.* An assertion macro for asserting assertion result.
///
/// # Example
/// ```compile_fail // because of referring private module.
/// use assertor::*;
/// use assertor::testing::*;
///
/// assert_that!(check_that!("actual_string").is_same_string_to("expected_string")).facts_are(vec![
///     Fact::new("expected", "expected_string"),
///     Fact::new("actual", "actual_string"),
/// ]);
/// ```
#[macro_export]
macro_rules! check_that {
    ($actual:expr) => {
        $crate::Subject::new(
            &$actual,
            stringify!($actual).to_string(),
            /* description= */ None,
            /* option= */ (),
            Some($crate::Location::new(
                file!().to_string(),
                line!(),
                column!(),
            )),
            std::marker::PhantomData::<$crate::testing::CheckThatResult>,
        )
    };
}

pub struct CheckThatResult(Result<(), AssertionResult>);

impl AssertionStrategy<CheckThatResult> for AssertionResult {
    fn do_fail(self) -> CheckThatResult {
        CheckThatResult(Err(self))
    }

    fn do_ok(self) -> CheckThatResult {
        // XXX: Unnecessary AssertionResult instantiation for ok cases.
        CheckThatResult(Ok(()))
    }
}

impl AsRef<Result<(), AssertionResult>> for CheckThatResult {
    fn as_ref(&self) -> &Result<(), AssertionResult> {
        &self.0
    }
}
