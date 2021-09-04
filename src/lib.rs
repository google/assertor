extern crate num;

#[allow(unused_imports)]
use assertions::basic::{ComparableAssertion, EqualityAssertion};
#[allow(unused_imports)]
use assertions::float::FloatAssertion;
#[allow(unused_imports)]
use assertions::iterator::IteratorAssertion;
#[allow(unused_imports)]
use assertions::map::MapAssertion;
#[allow(unused_imports)]
use assertions::result::ResultAssertion;
#[allow(unused_imports)]
use assertions::testing::AssertionResultAssertion;
#[allow(unused_imports)]
use base::{AssertionResult, CheckThatResult, Fact, Location, ReturnStrategy, Subject};

mod assertions;
mod base;
