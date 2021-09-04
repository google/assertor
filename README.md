Assertor
========
Assertor makes test assertions and failure messages more human-readable.

Assertor is heavy affected by [Java Truth](https://github.com/google/truth) in terms of API design and error messages.

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
 
