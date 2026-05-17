use js_like_eq::{JavaScriptEquality, JsonTruthy};
use serde_json::json;

fn main() {
    // Loose equality (JS ==)
    println!("=== Loose Equality ===");
    println!("1 == '1'    : {}", json!(1).equals(&json!("1")));
    println!("null == null: {}", json!(null).equals(&json!(null)));
    println!("null == 0   : {}", json!(null).equals(&json!(0)));
    println!("[] == false : {}", json!([]).equals(&json!(false)));
    println!("[] == ''    : {}", json!([]).equals(&json!("")));
    println!("[1] == 1    : {}", json!([1]).equals(&json!(1)));
    println!("true == 1   : {}", json!(true).equals(&json!(1)));
    println!("false == 0  : {}", json!(false).equals(&json!(0)));

    // Relational comparisons
    println!("\n=== Relational ===");
    println!("2 > 1       : {}", json!(2).greater(&json!(1)));
    println!("'b' > 'a'   : {}", json!("b").greater(&json!("a")));
    println!("null < 1    : {}", json!(null).less_than(&json!(1)));
    println!("[] < 1      : {}", json!([]).less_than(&json!(1)));

    // Truthiness
    println!("\n=== Truthiness ===");
    println!("'hello'     : {}", json!("hello").is_truthy(false));
    println!("''          : {}", json!("").is_truthy(false));
    println!("null        : {}", json!(null).is_truthy(false));
    println!("[]          : {}", json!([]).is_truthy(false));
    println!("[1,2]       : {}", json!([1, 2]).is_truthy(false));
}
