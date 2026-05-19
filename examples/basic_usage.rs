use js_like_eq::{JsCompare, JsValue, JsonTruthy};
use serde_json::json;

fn main() {
    // Loose equality (JS ==)
    println!("=== Loose equality ===");
    println!("1 == '1'     : {}", json!(1).js_eq(&json!("1")));
    println!("null == null : {}", json!(null).js_eq(&json!(null)));
    println!("null == 0    : {}", json!(null).js_eq(&json!(0)));
    println!("[] == false  : {}", json!([]).js_eq(&json!(false)));
    println!("[] == ''     : {}", json!([]).js_eq(&json!("")));
    println!("[1] == 1     : {}", json!([1]).js_eq(&json!(1)));
    println!("true == 1    : {}", json!(true).js_eq(&json!(1)));
    println!("false == 0   : {}", json!(false).js_eq(&json!(0)));

    // Strict equality (JS ===)
    println!("\n=== Strict equality ===");
    println!("1 === '1'    : {}", json!(1).js_strict_eq(&json!("1")));
    println!("1 === 1      : {}", json!(1).js_strict_eq(&json!(1)));
    println!("null === null: {}", json!(null).js_strict_eq(&json!(null)));

    // Relational
    println!("\n=== Relational ===");
    println!("2 > 1        : {}", json!(2).js_gt(&json!(1)));
    println!("'b' > 'a'    : {}", json!("b").js_gt(&json!("a")));
    println!("null < 1     : {}", json!(null).js_lt(&json!(1)));
    println!("[] < 1       : {}", json!([]).js_lt(&json!(1)));

    // Truthiness
    println!("\n=== Truthiness ===");
    println!("'hello'      : {}", json!("hello").is_truthy());
    println!("''           : {}", json!("").is_truthy());
    println!("null         : {}", json!(null).is_truthy());
    println!("0            : {}", json!(0).is_truthy());
    println!("[]           : {}", json!([]).is_truthy()); // true in JS
    println!("[1,2]        : {}", json!([1, 2]).is_truthy());

    // JsValue wrapper for `==` ergonomics
    println!("\n=== JsValue wrapper ===");
    let a = JsValue::from(json!([1]));
    let b = JsValue::from(json!(1));
    println!("JsValue([1]) == JsValue(1): {}", a == b);
}
