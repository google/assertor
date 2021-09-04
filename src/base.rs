use std::fmt;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Deref;

/// An assertion macro that panics when the assertion fails.
#[macro_export]
macro_rules! assert_that {
    ($actual:expr) => {
        $crate::Subject::new(
            &$actual,
            stringify!($actual)
                .to_string()
                .replace(" ", "")
                .replace("\n", ""),
            /* description= */ None,
            /* option= */ (),
            Some($crate::Location::new(
                file!().to_string(),
                line!(),
                column!(),
            )),
            std::marker::PhantomData::<()>,
        )
    };
}

pub struct Subject<'a, Sub, Opt, Ret> {
    actual: ActualValue<'a, Sub>,

    /// Stringified expression of actual value.
    /// Ex. `assert_that!(vec![1,2,3]).has_length(10)` -> `vec![1,2,3]`
    expr: String,

    /// Description for actual value. Will be used with "value of" fact message.
    /// Ex. assert_that!(actual_vec).has_length(10) -> "value of: actual_vec.len()"
    description: Option<String>,

    /// Options that changes assertion behavior.
    /// Ex. tolerance for float almost equality assertion.
    ///
    /// For subjects having no option, unit `()` should be specified as option type `Opt`.
    ///
    /// Design Note: this option should be in generics to achieve changing available methods
    /// depending on the option type. Ex. when float tolerance is specified, not related methods
    /// should be unavailable.
    option: Opt,

    location: Option<Location>,
    return_type: PhantomData<Ret>,
}

impl<'a, Sub, Opt, Ret> Subject<'a, Sub, Opt, Ret> {
    #[allow(dead_code)] // Used by macros.
    pub fn new(
        actual: &'a Sub,
        expr: String,
        description: Option<String>,
        option: Opt,
        location: Option<Location>,
        return_type: PhantomData<Ret>,
    ) -> Self {
        Subject {
            actual: ActualValue::Borrowed(actual),
            expr,
            description,
            option,
            location,
            return_type,
        }
    }

    pub(super) fn new_from_owned_actual(
        actual: Sub,
        expr: String,
        description: Option<String>,
        option: Opt,
        location: Option<Location>,
        return_type: PhantomData<Ret>,
    ) -> Self {
        Subject {
            actual: ActualValue::Owned(actual),
            expr,
            description,
            option,
            location,
            return_type,
        }
    }
}

pub enum ActualValue<'a, S> {
    Owned(S),
    Borrowed(&'a S),
}

impl<'a, S> Deref for ActualValue<'a, S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        match &self {
            ActualValue::Owned(value) => value,
            ActualValue::Borrowed(value) => value,
        }
    }
}

/// API for assertion library developer.
///
/// Note: This trait hides methods for library developer from library users.
/// TODO(koku): think better name...
pub trait AssertionApi<'a, Sub, Opt, Ret> {
    /// Builds a new instance of [AssertionResult].
    fn new_result(&self) -> AssertionResult;

    /// Actual value.
    fn actual(&self) -> &Sub;

    /// Returns [stringified](https://doc.rust-lang.org/std/macro.stringify.html) expression of
    /// applied actual value.
    /// Ex. For `assert_that!(vec![1,2,3])`, `expr` will be `"vec![1,2,3]"`.   
    fn expr(&self) -> &String;

    /// Returns description for actual value. For derived subjects (see [AssertionApi.new_subject]),
    /// `description` describes how derived from the original subject. For non derived subjects,
    /// `None` is returned instead.
    fn description(&self) -> &Option<String>;

    /// Returns description for actual value. For derived subjects (see [AssertionApi.new_subject]),
    /// `description` describes how derived from the original subject. For non derived subjects,
    /// `expr` is returned instead.
    fn description_or_expr(&self) -> &String;

    fn option(&self) -> &Opt;
    fn option_mut(&mut self) -> &mut Opt;

    /// Code location.
    fn location(&self) -> &Option<Location>;

    /// Creates a new derived subject.
    ///
    /// `new_description` should describe how it derives from the previous subject in
    /// code-like style. For example, in case of asserting the length of a vector `vec![1,2,3]`, a
    /// derived subject for the vector length can be created by this method. The new_actual will be
    /// `vec![1,2,3].len()` and `new_description` can be `vec![1,2,3].len()` or
    /// `vec![1,2,3].size()`. You may need `format!()` and `AssertionApi::description_or_expr()` to
    /// generate `new_description`.
    fn new_subject<NewSub, NewOpt>(
        &self,
        new_actual: &'a NewSub,
        new_description: Option<String>,
        new_opt: NewOpt,
    ) -> Subject<NewSub, NewOpt, Ret>;

    /// Creates a new derived subject.
    ///
    /// `new_description` should describe how it derives from the previous subject in
    /// code-like style. For example, in case of asserting the length of a vector `vec![1,2,3]`, a
    /// derived subject for the vector length can be created by this method. The new_actual will be
    /// `vec![1,2,3].len()` and `new_description` can be `vec![1,2,3].len()` or
    /// `vec![1,2,3].size()`. You may need `format!()` and `AssertionApi::description_or_expr()` to
    /// generate `new_description`.
    ///
    /// Differently from `new_subject`, this method takes owned actual value instead reference.
    fn new_owned_subject<'b, NewSub, NewOpt>(
        &self,
        new_actual: NewSub,
        new_description: Option<String>,
        new_option: NewOpt,
    ) -> Subject<'b, NewSub, NewOpt, Ret>;
}

impl<'a, Sub, Opt, Ret> AssertionApi<'a, Sub, Opt, Ret> for Subject<'a, Sub, Opt, Ret> {
    fn new_result(&self) -> AssertionResult {
        let mut result = AssertionResult::new(self.location());
        match &self.description {
            None => {}
            Some(description) => {
                result = result.add_fact("value of", description);
            }
        };
        result
    }

    fn actual(&self) -> &Sub {
        &self.actual
    }

    fn expr(&self) -> &String {
        &self.expr
    }

    fn description(&self) -> &Option<String> {
        &self.description
    }

    fn description_or_expr(&self) -> &String {
        match &self.description {
            None => self.expr(),
            Some(value) => value,
        }
    }

    fn option(&self) -> &Opt {
        &self.option
    }
    fn option_mut(&mut self) -> &mut Opt {
        &mut self.option
    }

    fn location(&self) -> &Option<Location> {
        &self.location
    }

    fn new_subject<NewSub, NewOpt>(
        &self,
        new_actual: &'a NewSub,
        new_description: Option<String>,
        new_option: NewOpt,
    ) -> Subject<NewSub, NewOpt, Ret> {
        Subject::new(
            new_actual,
            self.expr.clone(),
            new_description,
            new_option,
            self.location.clone(),
            self.return_type,
        )
    }
    fn new_owned_subject<'b, NewSub, NewOpt>(
        &self,
        new_actual: NewSub,
        new_description: Option<String>,
        new_option: NewOpt,
    ) -> Subject<'b, NewSub, NewOpt, Ret> {
        Subject::new_from_owned_actual(
            new_actual,
            self.expr.clone(),
            new_description,
            new_option,
            self.location.clone(),
            self.return_type,
        )
    }
}

/// A behavior for assertion pass and failure. [`AssertionResult`] implements this traits.  
///
/// Behavior for assertion pass and failure is different between [`assert_that`] and [`check_that`].
/// [`assert_that`] panics when assertion fails, but [`check_that`] results a struct in both cases.
/// Those assertion behavior is switched by [`Subject.return_type`] and [`AssertionStrategy`].
pub trait AssertionStrategy<R> {
    /// Behavior when assertion fails.
    fn do_fail(&self) -> R;

    /// Behavior when assertion passes.
    fn do_ok(&self) -> R;
}

impl AssertionStrategy<()> for AssertionResult {
    fn do_fail(&self) {
        std::panic::panic_any(self.generate_message());
    }

    fn do_ok(&self) {}
}

#[derive(Clone)]
pub struct AssertionResult {
    location: Option<String>,
    facts: Vec<Fact>,
}

impl AssertionResult {
    pub(self) fn new(location: &Option<Location>) -> Self {
        AssertionResult {
            location: location.as_ref().map(|loc| format!("{}", loc)),
            facts: vec![],
        }
    }

    #[inline]
    pub fn add_fact<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.facts.push(Fact::new(key, value));
        self
    }

    #[inline]
    pub fn add_simple_fact<V: Into<String>>(mut self, value: V) -> Self {
        self.facts.push(Fact::new_simple_fact(value));
        self
    }

    #[inline]
    pub fn add_splitter(mut self) -> Self {
        self.facts.push(Fact::new_splitter());
        self
    }

    pub fn generate_message(&self) -> String {
        let mut messages = vec![];

        messages.push(format!(
            "assertion failed{maybe_loc}",
            maybe_loc = match &self.location {
                None => format!(""),
                Some(loc) => format!(": {}", loc),
            }
        ));

        let longest_key_length = self
            .facts
            .iter()
            .flat_map(|fact| match fact {
                Fact::KeyValue { key, .. } => Some(key),
                Fact::Value { .. } => None,
                Fact::Splitter => None,
            })
            .map(|key| key.len())
            .max()
            .unwrap_or(0);

        for x in self.facts.iter() {
            match x {
                Fact::KeyValue { key, value } => messages.push(format!(
                    "{key:width$}: {value}",
                    key = key,
                    value = value,
                    width = longest_key_length
                )),
                Fact::Value { value } => messages.push(value.to_string()),
                Fact::Splitter => messages.push(String::from("---")),
            }
        }
        messages.join("\n")
    }

    pub fn facts(&self) -> &Vec<Fact> {
        &self.facts
    }
}

impl std::fmt::Debug for AssertionResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*self.generate_message())
    }
}

#[derive(Debug, Clone)]
pub struct Location {
    file: String,
    line: u32,
    column: u32,
}

impl Location {
    #[allow(dead_code)] // Used by macros.
    pub fn new<I: Into<String>>(file: I, line: u32, column: u32) -> Self {
        Location {
            file: file.into(),
            line,
            column,
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}:{}:{}", self.file, self.line, self.column))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Fact {
    KeyValue { key: String, value: String },
    Value { value: String },
    Splitter,
}

impl Fact {
    pub fn new<K: Into<String>, V: Into<String>>(key: K, value: V) -> Fact {
        Fact::KeyValue {
            key: key.into(),
            value: value.into(),
        }
    }
    pub fn new_simple_fact<V: Into<String>>(value: V) -> Fact {
        Fact::Value {
            value: value.into(),
        }
    }
    pub fn new_splitter() -> Fact {
        Fact::Splitter
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::CheckThatResult;
    use crate::*;

    use super::*;

    #[test]
    fn assert_that() {
        // macro doesn't fail
        assert_that!(1);
        assert_that!(vec![""]);
    }

    #[test]
    fn assert_that_unit_return_type() {
        assert_eq!(assert_that!(1).return_type, PhantomData::<()>::default());
        assert_eq!(
            assert_that!(vec![""]).return_type,
            PhantomData::<()>::default()
        );
    }

    #[test]
    fn check_that() {
        // macro doesn't fail
        check_that!(1);
        check_that!(vec![""]);
    }

    #[test]
    fn check_that_result_return_type() {
        assert_eq!(
            check_that!(1).return_type,
            PhantomData::<CheckThatResult>::default()
        );
        assert_eq!(
            check_that!(vec![""]).return_type,
            PhantomData::<CheckThatResult>::default()
        );
    }

    #[test]
    fn assert_result_message_generation() {
        assert_eq!(
            AssertionResult::new(&None).generate_message(),
            "assertion failed"
        );
        assert_eq!(
            AssertionResult::new(&Some(Location::new("foo.rs", 123, 456))).generate_message(),
            "assertion failed: foo.rs:123:456"
        );
        assert_eq!(
            AssertionResult::new(&Some(Location::new("foo.rs", 123, 456)))
                .add_fact("foo", "bar")
                .generate_message(),
            r#"assertion failed: foo.rs:123:456
foo: bar"#
        );
        assert_eq!(
            AssertionResult::new(&Some(Location::new("foo.rs", 123, 456)))
                .add_fact("foo", "bar")
                .add_fact("looooong key", "align indent")
                .generate_message(),
            r#"assertion failed: foo.rs:123:456
foo         : bar
looooong key: align indent"#
        );
        assert_eq!(
            AssertionResult::new(&Some(Location::new("foo.rs", 123, 456)))
                .add_fact("foo", "bar")
                .add_fact("looooong key", "align indent")
                .add_fact("s", "hort")
                .generate_message(),
            r#"assertion failed: foo.rs:123:456
foo         : bar
looooong key: align indent
s           : hort"#
        );
        assert_eq!(
            AssertionResult::new(&Some(Location::new("foo.rs", 123, 456)))
                .add_fact("foo", "bar")
                .add_splitter()
                .add_fact("s", "hort")
                .generate_message(),
            r#"assertion failed: foo.rs:123:456
foo: bar
---
s  : hort"#
        );
        assert_eq!(
            AssertionResult::new(&Some(Location::new("foo.rs", 123, 456)))
                .add_fact("foo", "bar")
                .add_simple_fact("I am ninja")
                .add_fact("s", "hort")
                .generate_message(),
            r#"assertion failed: foo.rs:123:456
foo: bar
I am ninja
s  : hort"#
        );
    }
}
