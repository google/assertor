Assertor
========
Assertor makes test assertions and failure messages more human-readable.

Assertor is heavyly affected by [Java Truth](https://github.com/google/truth) in terms of API design and error messages. Assertor is a totally different project from Java Truth.

### Disclaimer

This is not an official Google product, it is just code that happens to be owned by Google.

Example
-------

```rust
use assertor::*;

#[test]
fn test_it() {
    assert_that!("foobarbaz").contains("bar");
    assert_that!("foobarbaz").ends_with("baz");

    assert_that!(0.5).with_abs_tol(0.2).is_approx_equal_to(0.6);

    assert_that!(vec!["a", "b"]).contains("a");
    assert_that!(vec!["a", "b"]).has_length(2);
    assert_that!(vec!["a", "b"]).contains_exactly(vec!["a", "b"]);

    assert_that!(Option::Some("Foo")).has_value("Foo");
}
```

### Failure cases

```rust
use assertor::*;

fn test_it() {
    assert_that!(vec!["a", "b", "c"]).contains_exactly(vec!["b", "c", "d"]);
    // missing (1)   : ["d"]
    // unexpected (1): ["a"]
    // ---
    // expected      : ["b", "c", "d"]
    // actual        : ["a", "b", "c"]
}
```

## Feature ideas

- [ ] Color / Bold
- [ ] Better diff: vec
- [ ] Better diff: set

## Non goals

#### Structural diff similarity to [ProtoTruth](https://truth.dev/protobufs)

**Preferred:** Define custom `Assertion` or assert multiple.

```rust
struct Foo {
    name: String,
    ids: Vec<usize>,
}

fn test_it() {
    let act = Foo { name: "abc", ids: vec![1, 2, 3] };
    let exp = Foo { name: "xyz", ids: vec![3, 2, 1] };
    assert_that!(act).ignoring_field_order("ids").is_equal_to(exp);

    // Preferred
    assert_that!(act.name).is_same_string_to(exp.name);
    assert_that!(act.ids).contains_exactly(exp.ids);
}
```
