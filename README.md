# js_like_eq

JavaScript-style loose equality (`==`), strict equality (`===`), relational
comparison (`<`, `<=`, `>`, `>=`), and truthiness for `serde_json::Value`.

If you've ever had a JSON-driven rule engine and wanted `[1] == 1` and
`null == null` and `"5" < 10` to behave the way V8 does — without shipping a JS
engine — this crate is that.

## Usage

```toml
[dependencies]
js_like_eq = "0.1"
serde_json = "1"
```

```rust
use js_like_eq::{JsCompare, JsValue, JsonTruthy};
use serde_json::json;

assert!(json!(1).js_eq(&json!("1")));
assert!(json!([1]).js_eq(&json!(1)));
assert!(json!(null).js_eq(&json!(null)));
assert!(!json!(null).js_eq(&json!(0)));

assert!(json!("b").js_gt(&json!("a")));
assert!(json!(null).js_lt(&json!(1)));

assert!(json!("hello").is_truthy());
assert!(!json!("").is_truthy());

// Ergonomic `==` via the newtype wrapper.
assert_eq!(JsValue::from(json!([1])), JsValue::from(json!(1)));
```

## Semantics

- **`js_eq`** mirrors `==`: strings parse into numbers, single-element arrays
  unwrap, empty arrays equal `false` and `""`, arrays/objects never compare
  equal structurally (matching JS reference semantics).
- **`js_strict_eq`** mirrors `===`: same type required; primitives compared by
  value; arrays/objects always return `false` (owned `Value`s have no shared
  identity, so the closest mirror of JS's reference rule is "two literals are
  not `===`"). Use `serde_json::Value`'s own `PartialEq` if you want structural
  equality of composites.
- **`js_lt` / `js_le`** mirror `<` / `<=`: `null` coerces to `0`, booleans to
  `0`/`1`, objects stringify to `"[object Object]"`, and arrays either unwrap,
  become `false`, or join with commas depending on the partner type.
- **`is_truthy`** matches JS exactly: `null`, `false`, `0`, `NaN`, and `""` are
  falsy; everything else (including `[]` and `{}`) is truthy.

`PartialOrd` is intentionally **not** implemented on `JsValue` because JS
relational operators do not form a consistent partial order (e.g. two empty
objects are `<=` each other in both directions yet `==` is false).

## Testing

```bash
cargo test --lib         # ~190 unit tests covering the coercion table
cargo test -- --ignored  # Node oracle cross-checks (requires `node` in PATH)
cargo clippy
```

The integration test in `tests/node_cross_test.rs` spawns Node and `eval()`s
the equivalent JS expression to confirm Rust's answer matches V8.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
