//! JavaScript-style comparison and truthiness for [`serde_json::Value`].
//!
//! This crate ports the coercion rules of JavaScript's `==`, `===`, `<`, `<=`,
//! `>`, `>=` and the language's truthiness rules onto `serde_json::Value` so
//! values built with [`serde_json::json!`] compare the way the equivalent JS
//! literals would.
//!
//! # Quick start
//!
//! ```
//! use js_like_eq::{JsCompare, JsonTruthy};
//! use serde_json::json;
//!
//! assert!(json!(1).js_eq(&json!("1")));
//! assert!(json!(null).js_eq(&json!(null)));
//! assert!(!json!(null).js_eq(&json!(0)));
//! assert!(json!([1]).js_eq(&json!(1)));
//! assert!(json!("b").js_gt(&json!("a")));
//! assert!(json!("hello").is_truthy());
//! ```
//!
//! For ergonomic operator-based comparison, wrap values in [`JsValue`]:
//!
//! ```
//! use js_like_eq::JsValue;
//! use serde_json::json;
//!
//! assert_eq!(JsValue::from(json!(1)), JsValue::from(json!("1")));
//! ```
//!
//! # Semantics
//!
//! `js_eq` mirrors `==`: numbers parse out of strings, single-element arrays
//! unwrap, empty arrays equal `false` and `""`, and arrays/objects never equal
//! anything structurally (matching JS reference semantics).
//!
//! `js_lt` / `js_le` mirror `<` / `<=`: `null` coerces to `0`, booleans coerce
//! to `0`/`1`, objects stringify to `"[object Object]"`, and arrays either
//! unwrap, become `false`, or join with commas depending on the partner type.
//!
//! `js_strict_eq` mirrors `===`: same type required, primitives compared by
//! value; arrays and objects never strictly equal each other since the JS rule
//! is reference identity and owned `Value`s have no identity to share.

use serde_json::{Value, json};

/// JavaScript-style comparison operators for [`serde_json::Value`].
///
/// See the [crate-level docs](crate) for the coercion rules. Default impls are
/// provided for `js_ne`, `js_strict_ne`, `js_gt`, and `js_ge` in terms of the
/// other methods.
pub trait JsCompare {
    /// JS `==` (loose equality with coercion).
    fn js_eq(&self, other: &Self) -> bool;

    /// JS `===` (strict equality: same type and value).
    fn js_strict_eq(&self, other: &Self) -> bool;

    /// JS `<`.
    fn js_lt(&self, other: &Self) -> bool;

    /// JS `<=`.
    fn js_le(&self, other: &Self) -> bool;

    /// JS `>`. Defaults to `other.js_lt(self)`.
    fn js_gt(&self, other: &Self) -> bool {
        other.js_lt(self)
    }

    /// JS `>=`. Defaults to `other.js_le(self)`.
    fn js_ge(&self, other: &Self) -> bool {
        other.js_le(self)
    }

    /// JS `!=`. Defaults to `!self.js_eq(other)`.
    fn js_ne(&self, other: &Self) -> bool {
        !self.js_eq(other)
    }

    /// JS `!==`. Defaults to `!self.js_strict_eq(other)`.
    fn js_strict_ne(&self, other: &Self) -> bool {
        !self.js_strict_eq(other)
    }
}

/// JavaScript-style truthiness for [`serde_json::Value`].
pub trait JsonTruthy {
    /// Returns `true` if the value would be considered truthy in JS.
    ///
    /// - `null` → `false`
    /// - `bool` → its own value
    /// - number → `true` if non-zero and non-NaN
    /// - string → `true` if non-empty
    /// - array / object → always `true` (even when empty), matching JS
    fn is_truthy(&self) -> bool;
}

impl JsonTruthy for Value {
    fn is_truthy(&self) -> bool {
        match self {
            Value::Null => false,
            Value::Bool(b) => *b,
            Value::Number(n) => n.as_f64().map(|f| f != 0.0 && !f.is_nan()).unwrap_or(false),
            Value::String(s) => !s.is_empty(),
            Value::Array(_) | Value::Object(_) => true,
        }
    }
}

impl JsCompare for Value {
    fn js_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Null, Value::Null) => true,
            // In JS, comparing null to anything other than null/undefined is false.
            // Arrays and objects never structurally equal each other (reference semantics).
            (Value::Null, _) | (_, Value::Null) => false,
            (Value::Array(_), Value::Array(_)) => false,
            (_, Value::Object(_)) | (Value::Object(_), _) => false,
            (Value::Bool(_), Value::Bool(_)) | (Value::String(_), Value::String(_)) => {
                self == other
            }
            (Value::Number(a), Value::Number(b)) => match (a.as_f64(), b.as_f64()) {
                (Some(a), Some(b)) => approx_eq(a, b),
                _ => false,
            },
            (Value::Number(n), Value::String(s)) | (Value::String(s), Value::Number(n)) => {
                if let (Ok(parsed), Some(num)) = (s.parse::<f64>(), n.as_f64()) {
                    approx_eq(parsed, num)
                } else {
                    false
                }
            }
            (Value::Bool(b), Value::Number(n)) | (Value::Number(n), Value::Bool(b)) => {
                match (n.as_f64(), b) {
                    (Some(num), true) => num == 1.0,
                    (Some(num), false) => num == 0.0,
                    _ => false,
                }
            }
            (Value::Bool(b), Value::String(s)) | (Value::String(s), Value::Bool(b)) => {
                if s.is_empty() {
                    return !b;
                }
                match (s.parse::<f64>(), b) {
                    (Ok(parsed), true) => parsed == 1.0,
                    (Ok(parsed), false) => parsed == 0.0,
                    _ => false,
                }
            }
            // In JS, comma-separated strings compare elementwise against arrays.
            (Value::String(s), Value::Array(values)) | (Value::Array(values), Value::String(s)) => {
                if s.is_empty() && values.is_empty() {
                    return true;
                }
                let mut s_elements = s.split(',');
                let mut array_elements = values.iter();
                loop {
                    match (array_elements.next(), s_elements.next()) {
                        (Some(array_value), Some(string_element)) => {
                            if !array_value.js_eq(&Value::String(string_element.to_string())) {
                                return false;
                            }
                        }
                        (None, None) => return true,
                        _ => return false,
                    }
                }
            }
            // Single-element arrays unwrap; empty arrays equal `false`.
            (other, Value::Array(values)) | (Value::Array(values), other) => {
                if values.len() == 1 {
                    return other.js_eq(&values[0]);
                }
                if let Value::Bool(b) = other {
                    if values.is_empty() {
                        return !b;
                    }
                }
                false
            }
        }
    }

    fn js_strict_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Null, Value::Null) => true,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => match (a.as_f64(), b.as_f64()) {
                (Some(x), Some(y)) => approx_eq(x, y),
                _ => a == b,
            },
            (Value::String(a), Value::String(b)) => a == b,
            // JS `===` for composites is reference identity; owned `Value`s have
            // no shared identity, so we mirror "two literals are not `===`".
            (Value::Array(_), Value::Array(_)) | (Value::Object(_), Value::Object(_)) => false,
            _ => false,
        }
    }

    fn js_lt(&self, rhs: &Self) -> bool {
        coerce_and_compare(self, rhs, |a, b| a < b, |a, b| a < b, |a, b| a < b)
    }

    fn js_le(&self, rhs: &Self) -> bool {
        coerce_and_compare(self, rhs, |a, b| a <= b, |a, b| a <= b, |a, b| a <= b)
    }
}

/// An owned wrapper around [`serde_json::Value`] that compares with JS semantics.
///
/// Implements [`PartialEq`] via [`JsCompare::js_eq`] so values can be compared
/// with `==` and `!=` directly. Ordering is intentionally **not** implemented
/// via [`PartialOrd`] because JS `<`/`<=`/`==` do not form a consistent
/// `PartialOrd` (e.g. two empty objects are `<=` each other in both directions
/// yet `==` is false). Use [`JsCompare::js_lt`] etc. for ordering.
///
/// # Example
///
/// ```
/// use js_like_eq::JsValue;
/// use serde_json::json;
///
/// let a = JsValue::from(json!([1]));
/// let b = JsValue::from(json!(1));
/// assert_eq!(a, b);
/// ```
#[derive(Debug, Clone)]
pub struct JsValue(pub Value);

impl JsValue {
    /// Returns a reference to the inner [`Value`].
    pub fn inner(&self) -> &Value {
        &self.0
    }

    /// Consumes the wrapper and returns the inner [`Value`].
    pub fn into_inner(self) -> Value {
        self.0
    }
}

impl From<Value> for JsValue {
    fn from(v: Value) -> Self {
        JsValue(v)
    }
}

impl From<JsValue> for Value {
    fn from(v: JsValue) -> Self {
        v.0
    }
}

impl AsRef<Value> for JsValue {
    fn as_ref(&self) -> &Value {
        &self.0
    }
}

impl std::ops::Deref for JsValue {
    type Target = Value;

    fn deref(&self) -> &Value {
        &self.0
    }
}

impl PartialEq for JsValue {
    fn eq(&self, other: &Self) -> bool {
        self.0.js_eq(&other.0)
    }
}

/// Approximate equality for `f64`, used to absorb tiny rounding error.
///
/// Uses [`f64::EPSILON`] as an absolute tolerance, which is correct only for
/// numbers near `1.0`. Callers comparing values of very different magnitude
/// should use [`PartialEq`] directly.
pub(crate) fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < f64::EPSILON
}

/// JS `Array.prototype.toString`: comma-join, flattening nested arrays and
/// rendering objects as `"[object Object]"`.
pub(crate) fn array_to_string(vec: &[Value]) -> String {
    vec.iter()
        .map(|item| match item {
            Value::String(s) => s.clone(),
            Value::Array(values) => array_to_string(values),
            Value::Object(_) => "[object Object]".to_string(),
            _ => item.to_string(),
        })
        .collect::<Vec<String>>()
        .join(",")
}

/// Shared engine for `<`, `<=`, `>`, `>=`.
///
/// Recursively coerces operands toward a common primitive type, then calls the
/// appropriate comparator closure. The recursion terminates when both sides
/// are the same primitive (`bool`/`bool`, `string`/`string`, `number`/`number`).
fn coerce_and_compare<B, S, N>(
    lhs: &Value,
    rhs: &Value,
    bool_compare: B,
    string_compare: S,
    num_compare: N,
) -> bool
where
    B: Fn(&bool, &bool) -> bool,
    S: Fn(&str, &str) -> bool,
    N: Fn(&f64, &f64) -> bool,
{
    match (lhs, rhs) {
        // `null` coerces to `0` under relational operators.
        (Value::Null, _) => {
            coerce_and_compare(&json!(0), rhs, bool_compare, string_compare, num_compare)
        }
        (_, Value::Null) => {
            coerce_and_compare(lhs, &json!(0), bool_compare, string_compare, num_compare)
        }
        (Value::Bool(l), Value::Bool(r)) => bool_compare(l, r),
        (Value::String(l), Value::String(r)) => string_compare(l, r),
        (Value::Number(l), Value::Number(r)) => match (l.as_f64(), r.as_f64()) {
            (Some(l), Some(r)) => num_compare(&l, &r),
            _ => false,
        },
        (Value::Number(_), Value::String(s)) | (Value::String(s), Value::Number(_)) => {
            let s_parsed: Value = if s.is_empty() {
                json!(0)
            } else {
                match s.parse::<f64>() {
                    Ok(parsed) => json!(parsed),
                    // NaN compared with anything is false in JS.
                    _ => return false,
                }
            };
            if matches!((lhs, rhs), (Value::Number(_), Value::String(_))) {
                coerce_and_compare(lhs, &s_parsed, bool_compare, string_compare, num_compare)
            } else {
                coerce_and_compare(&s_parsed, rhs, bool_compare, string_compare, num_compare)
            }
        }
        (_, Value::Bool(b)) => {
            let coerced = if *b { json!(1) } else { json!(0) };
            coerce_and_compare(lhs, &coerced, bool_compare, string_compare, num_compare)
        }
        (Value::Bool(b), _) => {
            let coerced = if *b { json!(1) } else { json!(0) };
            coerce_and_compare(&coerced, rhs, bool_compare, string_compare, num_compare)
        }
        (Value::Object(_), _) | (_, Value::Object(_)) => {
            let placeholder = json!("[object Object]");
            if matches!(lhs, Value::Object(_)) {
                coerce_and_compare(
                    &placeholder,
                    rhs,
                    bool_compare,
                    string_compare,
                    num_compare,
                )
            } else {
                coerce_and_compare(
                    lhs,
                    &placeholder,
                    bool_compare,
                    string_compare,
                    num_compare,
                )
            }
        }
        (Value::Array(values), other) | (other, Value::Array(values)) => {
            let array_coerced = if values.len() == 1
                && !matches!(other, Value::String(_) | Value::Array(_))
            {
                values[0].clone()
            } else if values.is_empty() && !matches!(other, Value::String(_) | Value::Array(_)) {
                json!(false)
            } else {
                match other {
                    Value::Bool(_) | Value::Number(_) | Value::String(_) | Value::Array(_) => {
                        json!(array_to_string(values))
                    }
                    Value::Object(_) | Value::Null => unreachable!(),
                }
            };
            if matches!(lhs, Value::Array(_)) {
                coerce_and_compare(
                    &array_coerced,
                    other,
                    bool_compare,
                    string_compare,
                    num_compare,
                )
            } else {
                coerce_and_compare(
                    other,
                    &array_coerced,
                    bool_compare,
                    string_compare,
                    num_compare,
                )
            }
        }
    }
}

#[cfg(test)]
mod tests;
