How to extend
=============

## 1. Write your own Assertion trait.
- S: Subject type (e.g. HashSet<T>)

```rust
trait SetAssertion<S, T, R> {
    fn contains<B: Borrow<T>>(&self, expected: B) -> R;
    fn is_equal_to<B: Borrow<S>>(&self, expected: S) -> R;
}
```

## 2. Write implementation for `Subject<S, (), R>`.

```rust
impl<T, R> SetAssertion<HashSet<T>, T, R> for Subject<HashSet<T>, (), R> {
    fn contains<B: Borrow<T>>(&self, expected: B) -> R {
        if (self.actual().contains(expected)) {
            self.new_result().do_ok()
        } else {
            self.new_result()
                .add_fact("expected to contain", "XXX")
                .add_fact(...)
                .do_fail()
        }
    }
}
```

## 3. Write tests.

```rust
#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

    use crate::*;

    use super::*;

    #[test]
    fn contains() {
        assert_that!(HashSet::from_iter(vec![1, 2, 3].iter())).contains(&3);

        // Failures
        let result = check_that!(HashSet::from_iter(vec![1, 2, 3].iter())).contains(&10);
        assert_that!(result).facts_are_at_least(vec![
            Fact::new("expected to contain", "10"),
            Fact::new_simple_fact("but did not"),
        ]);
        assert_that!(result)
            .fact_keys()
            .contains(&"though it did contain".to_string());
        // Skip test for value because key order is not stable.
    }
}
```

Do and Dont
===========

DONT: Method chain of multiple independent assertions
-----------------------------------------------------

```rust
assert_that!(3).is_less_than(10).is_larger_than(1);
assert_that!(vec![1,2,3]).contains_exactly(3, 2, 1).in_order(); // Java Truth like 
```

### DO:

##### Recommended: Independent assertions.

```rust
assert_that!(3).is_less_than(10);
assert_that!(3).is_larger_than(1);
```

##### Ok: Define one assertion method to check both.

```rust
assert_that!(3).is_in(1, 10);
assert_that!(vec![1,2,3]).contains_exactly_in_order(3, 2, 1);
```

### Why?

- Readability
    - One `assert_that!()` should check one assertion for readability.
- Design complication
    - Return type of `assert_that!(3).is_less_than(10)` will be complicated like `Result<Subject<..>, R>`.

DO: Derived subjects
--------------------

```rust
assert_that!(hash_map).key_set().contains( & "foo");
assert_that!(hash_map).key_set().contains_exactly( & "foo", & "bar");
```

It is one option not to use derived subject when the assertion method is expected to be used frequently and when the api
of derived subject is hard to use (`&` in argument in previous example).

##### Ok

```rust
assert_that!(hash_map).contains_key("foo");
assert_that!(hash_map).contains_key("bar");
```

### Why?

- API simplicity
    - To avoid having many method having `key_set` prefix (
      ex. `assert_that!(..).key_set_contains(), assert_that!(..).key_set_contains_exactly()`)
- Code duplication
    - Derived subject enables to reuse the existing implementation easily.

### What is `derived subject`?

The feature to create a new subject whose type is different from original subject type.

- `Subject.new_subject(..)`
- `Subject.new_owned_subject(..)`

```rust

impl<'a, K, V, R> MapAssertion<'a, K, V, R> for Subject<'a, HashMap<K, V>, (), R>
    where
        AssertionResult: ReturnStrategy<R>,
{
    fn key_set(&self) -> Subject<'a, Keys<K, V>, (), R> {
        self.new_owned_subject(
            self.actual().keys(),
            Some(format!("{}.keys()", self.description_or_expr())),
            (),
        )
    }
}
```