pub use crate::assertions::testing::CheckThatResultAssertion;
pub use crate::{assert_that, check_that, Fact};
use crate::{AssertionResult, AssertionStrategy};

/// An assertion macro that returns [`CheckThatResult`] as assertion result. Expected to be used
/// with [``]
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
    fn do_fail(&self) -> CheckThatResult {
        // XXX: Maybe removable clone. Think better way.
        CheckThatResult(Err(self.clone()))
    }

    fn do_ok(&self) -> CheckThatResult {
        // XXX: Unnecessary AssertionResult instantiation for ok cases.
        CheckThatResult(Ok(()))
    }
}

impl AsRef<Result<(), AssertionResult>> for CheckThatResult {
    fn as_ref(&self) -> &Result<(), AssertionResult> {
        &self.0
    }
}
