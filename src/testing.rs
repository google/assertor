pub use crate::assertions::testing::AssertionResultAssertion;
pub use crate::{assert_that, check_that, Fact};
use crate::{AssertionResult, ReturnStrategy};

/// An assertion macro that returns [`Result<(), AssertionResult>`](`CheckThatResult`) as an
/// assertion result.
#[macro_export]
macro_rules! check_that {
    ($actual:expr) => {
        $crate::Subject::new(
            &$actual,
            stringify!($actual).to_string(),
            /*description=*/ None,
            /*option=*/ (),
            Some($crate::Location::new(
                file!().to_string(),
                line!(),
                column!(),
            )),
            std::marker::PhantomData::<$crate::testing::CheckThatResult>,
        )
    };
}

pub type CheckThatResult = Result<(), AssertionResult>;

impl ReturnStrategy<CheckThatResult> for AssertionResult {
    fn do_fail(&self) -> CheckThatResult {
        // XXX: Maybe removable clone. Think better way.
        Err(self.clone())
    }

    fn do_ok(&self) -> CheckThatResult {
        // XXX: Unnecessary AssertionResult instantiation for ok cases.
        Ok(())
    }
}
