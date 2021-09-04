Assertor
========
Assertor makes test assertions and failure messages more human-friendly.

Assertor is nearly Rust port of [Java Truth](https://github.com/google/truth), but is actually not related to Guava team
who maintains Java Truth library.

### Disclaimer

This is not an official Google product, it is just code that happens to be owned by Google.

Basic example
-------------

TODO(koku): write examples with expected messages.

```rust
#[test]
fn test_example() {
    let actual_text = "FooBar";
    assert_that!(actual_text).contains("Baz");
    //

    let mut actual_hashmap = HashMap::new();
    actual_hashmap.put();
}
```
 
