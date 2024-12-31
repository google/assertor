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

use num_traits::{Float, Zero};

use crate::base::{AssertionApi, AssertionResult, AssertionStrategy, Subject};

/// Trait for float assertion.
///
/// # Example
/// ```
/// use assertor::*;
/// assert_that!(0.1_f32).is_approx_equal_to(0.1);
/// assert_that!(0.1_f32)
///     .with_abs_tol(0.5)
///     .is_approx_equal_to(0.5);
/// assert_that!(0.1_f64)
///     .with_rel_tol(0.2)
///     .is_approx_equal_to(0.12); // 0.1 ± 0.12 * 0.2
/// ```
pub trait FloatAssertion<'a, S, R> {
    /// Set the relative tolerance.
    fn with_rel_tol(self, rel_tol: S) -> Subject<'a, S, FloatTolerance<S>, R>;
    /// Set the absolute tolerance.
    fn with_abs_tol(self, abs_tol: S) -> Subject<'a, S, FloatTolerance<S>, R>;

    /// Checks the subject is equal to `expected` with tolerance.
    ///
    /// The equality with tolerance is defined as following:
    /// ```math
    /// abs(actual - expected) <= (asb_tol + rel_tol * abs(expected))
    /// ```
    /// See also: [numpy.isclose](https://numpy.org/doc/stable/reference/generated/numpy.isclose.html)
    #[track_caller]
    fn is_approx_equal_to<B: Borrow<S>>(&self, expected: B) -> R
    where
        FloatTolerance<S>: Default;
}

pub struct FloatTolerance<S> {
    /// relative tolerance
    rel_tol: S,
    /// absolute tolerance
    abs_tol: S,
}

impl<S> FloatTolerance<S> {
    fn new(rel_tol: S, abs_tol: S) -> Self {
        FloatTolerance { rel_tol, abs_tol }
    }
    fn with_rel_tol(mut self, rel_tol: S) -> Self {
        self.rel_tol = rel_tol;
        self
    }
    fn with_abs_tol(mut self, abs_tol: S) -> Self {
        self.abs_tol = abs_tol;
        self
    }
}

impl<S: Zero> FloatTolerance<S> {
    fn zeros() -> Self {
        FloatTolerance::new(S::zero(), S::zero())
    }
}

impl Default for FloatTolerance<f32> {
    fn default() -> Self {
        // from numpy.isclose()
        FloatTolerance::new(1e-05, 1e-08)
    }
}

impl Default for FloatTolerance<f64> {
    fn default() -> Self {
        // from numpy.isclose()
        FloatTolerance::new(1e-05, 1e-08)
    }
}

impl<'a, S, R> FloatAssertion<'a, S, R> for Subject<'a, S, FloatTolerance<S>, R>
where
    S: Float + Debug,
    AssertionResult: AssertionStrategy<R>,
{
    fn with_rel_tol(mut self, rel_tol: S) -> Subject<'a, S, FloatTolerance<S>, R> {
        self.option_mut().rel_tol = rel_tol;
        self
    }

    fn with_abs_tol(mut self, abs_tol: S) -> Subject<'a, S, FloatTolerance<S>, R> {
        self.option_mut().abs_tol = abs_tol;
        self
    }

    fn is_approx_equal_to<B: Borrow<S>>(&self, expected: B) -> R {
        let diff = (*self.actual() - *expected.borrow()).abs();
        let tolerance: S = self.option().abs_tol + self.option().rel_tol * *expected.borrow();
        if diff < tolerance {
            self.new_result().do_ok()
        } else {
            self.new_result()
                .add_fact("expected", format!("{:?}", expected.borrow()))
                .add_fact("but was", format!("{:?}", self.actual()))
                .add_fact("outside tolerance", format!("{:?}", tolerance))
                .do_fail()
        }
    }
}

impl<'a, S, R: 'a> FloatAssertion<'a, S, R> for Subject<'a, S, (), R>
where
    S: Float + Debug,
    AssertionResult: AssertionStrategy<R>,
{
    fn with_rel_tol(self, rel_tol: S) -> Subject<'a, S, FloatTolerance<S>, R> {
        // XXX: consider to remove clone.
        self.new_owned_subject(
            *self.actual(),
            self.description().clone(),
            FloatTolerance::zeros().with_rel_tol(rel_tol),
        )
    }

    fn with_abs_tol(self, abs_tol: S) -> Subject<'a, S, FloatTolerance<S>, R> {
        // XXX: consider to remove clone.
        self.new_owned_subject(
            *self.actual(),
            self.description().clone(),
            FloatTolerance::zeros().with_abs_tol(abs_tol),
        )
    }

    fn is_approx_equal_to<B: Borrow<S>>(&self, expected: B) -> R
    where
        FloatTolerance<S>: Default,
    {
        self.new_subject(self.actual(), None, FloatTolerance::default())
            .is_approx_equal_to(expected)
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;

    use super::*;

    #[test]
    fn is_approx_equal_to() {
        assert_that!(0.1_f32).is_approx_equal_to(0.1);
        assert_that!(0.1_f32).is_approx_equal_to(0.1);
        assert_that!(0.1_f32)
            .with_abs_tol(0.5)
            .is_approx_equal_to(0.5);
        assert_that!(0.1_f32)
            .with_rel_tol(0.2)
            .is_approx_equal_to(0.12); // 0.1 ± 0.12 * 0.2

        assert_that!(0.1_f64).is_approx_equal_to(0.1);
        assert_that!(0.1_f64).is_approx_equal_to(0.100000001);
        assert_that!(0.1_f64)
            .with_abs_tol(0.5)
            .is_approx_equal_to(0.5);
        assert_that!(0.1_f64)
            .with_rel_tol(0.2)
            .is_approx_equal_to(0.12); // 0.1 ± 0.12 * 0.2

        // Failures
        assert_that!(check_that!(0.1).with_abs_tol(0.1).is_approx_equal_to(0.25)).facts_are(vec![
            Fact::new("expected", "0.25"),
            Fact::new("but was", "0.1"),
            Fact::new("outside tolerance", "0.1"),
        ]);
        assert_that!(check_that!(0.1).is_approx_equal_to(0.3)).facts_are(vec![
            Fact::new("expected", "0.3"),
            Fact::new("but was", "0.1"),
            Fact::new("outside tolerance", "3.01e-6"),
        ])
    }
}
