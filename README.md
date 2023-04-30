Assertor
========
Assertor makes test assertions and failure messages more human-readable.

[![crates.io](https://img.shields.io/crates/v/assertor.svg)](https://crates.io/crates/assertor)
[![license](https://img.shields.io/github/license/google/assertor.svg)](https://github.com/google/assertor/LICENSE)
[![docs.rs](https://docs.rs/assertor/badge.svg)](https://docs.rs/crate/assertor/)
[![OpenSSF
Scorecard](https://api.securityscorecards.dev/projects/github.com/google/assertor/badge)](https://api.securityscorecards.dev/projects/github.com/google/assertor)

Assertor is heavily affected by [Java Truth](https://github.com/google/truth) in terms of API design and error messages,
but this is a totally different project from Java Truth.

### Disclaimer

This is not an official Google product, it is just code that happens to be owned by Google.

⚠ The API is not fully stable and may be changed until version 1.0.

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
- [ ] Better diff: HashMap


