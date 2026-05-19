use super::*;
#[test]
fn test_is_equal() {
    let a = json!(1);
    let b = json!(1);
    assert!(a.js_eq(&b));
}
#[test]
fn test_is_equal_num_num() {
    let a = json!(256);
    let b = json!(256);
    assert!(a.js_eq(&b));
}

#[test]
fn test_is_equal_num_float() {
    let a = json!(256);
    let b = json!(256.0);
    assert!(a.js_eq(&b));
}
#[test]
fn test_is_equal_num_float_2() {
    let a = json!(1.00000000000000000000000000000000000001);
    let b = json!(1);
    assert!(a.js_eq(&b));
}
#[test]
fn test_is_equal_float_float() {
    let a = json!(0.125);
    let b = json!(0.125);
    assert!(a.js_eq(&b));
}
#[test]
fn test_is_equal_string_num() {
    let a = json!("1");
    let b = json!(1);
    assert!(a.js_eq(&b));
}
#[test]
fn test_is_equal_string_float() {
    let a = json!("1");
    let b = json!(1.0);
    assert!(a.js_eq(&b));
}
#[test]
fn test_is_equal_stringfloat_float() {
    let a = json!("1.0");
    let b = json!(1.0);
    assert!(a.js_eq(&b));
}
#[test]
fn test_is_equal_float_float_2() {
    let a = json!(1.0);
    let b = json!(1.0);
    assert!(a.js_eq(&b));
}
#[test]
fn test_is_equal_float_float_approximate() {
    let a = json!(1.000000000000000000000000000000001);
    let b = json!(1.0);
    assert!(a.js_eq(&b));
}
#[test]
fn test_is_equal_stringfloat_float_approximate() {
    let a = json!("1.000000000000000000000000000000001");
    let b = json!(1.0);
    assert!(a.js_eq(&b));
}
#[test]
fn test_is_equal_bool_num() {
    let a = json!(true);
    let b = json!(1);
    assert!(a.js_eq(&b));
}
#[test]
fn test_is_equal_bool_onefloat() {
    let a = json!(true);
    let b = json!(1.0);
    assert!(a.js_eq(&b));
}
#[test]
fn test_is_equal_falsebool_float() {
    let a = json!(false);
    let b = json!(0.1);
    assert!(!a.js_eq(&b));
}

#[test]
fn test_is_equal_truebool_float() {
    let a = json!(true);
    let b = json!(0.1);
    assert!(!a.js_eq(&b));
}
#[test]
fn test_is_equal_falsebool_num() {
    let a = json!(false);
    let b = json!(0);
    assert!(a.js_eq(&b));
}
#[test]
fn test_is_equal_falsebool_zerofloat() {
    let a = json!(false);
    let b = json!(0.0000000000000000000000000);
    assert!(a.js_eq(&b));
}
#[test]
fn test_is_equal_bool_numstring() {
    let a = json!(true);
    let b = json!("1");
    assert!(a.js_eq(&b));
}
#[test]
fn test_is_equal_bool_onefloatstring() {
    let a = json!(true);
    let b = json!("1.0");
    assert!(a.js_eq(&b));
}
#[test]
fn test_is_equal_falsebool_floatstring() {
    let a = json!(false);
    let b = json!("0.1");
    assert!(!a.js_eq(&b));
}

#[test]
fn test_is_equal_truebool_floatstring() {
    let a = json!(true);
    let b = json!("0.1");
    assert!(!a.js_eq(&b));
}
#[test]
fn test_is_equal_falsebool_numstring() {
    let a = json!(false);
    let b = json!("0");
    assert!(a.js_eq(&b));
}
#[test]
fn test_is_equal_falsebool_emptystring() {
    let a = json!(false);
    let b = json!("");
    assert!(a.js_eq(&b));
}
#[test]
fn test_is_equal_falsebool_zerofloatstring() {
    let a = json!(false);
    let b = json!("0.0000000000000000000000000");
    assert!(a.js_eq(&b));
}
#[test]
fn test_is_equal_bool_emptyarray() {
    let a = json!(false);
    let b = json!([]);
    assert!(a.js_eq(&b));
}
#[test]
fn test_is_equal_bool_singleonearray() {
    let a = json!(true);
    let b = json!([1]);
    assert!(a.js_eq(&b));
}
#[test]
fn test_is_equal_bool_singlezeroarray() {
    let a = json!(false);
    let b = json!([0]);
    assert!(a.js_eq(&b));
}
#[test]
fn test_is_equal_bool_singlezerostringarray() {
    let a = json!(false);
    let b = json!(["0"]);
    assert!(a.js_eq(&b));
}
#[test]
fn test_is_equal_num_singlenumarray() {
    let a = json!(2);
    let b = json!([2]);
    assert!(a.js_eq(&b));
}
#[test]
fn test_is_equal_num_singlenumarray_2() {
    let a = json!(3);
    let b = json!([2]);
    assert!(!a.js_eq(&b));
}
#[test]
fn test_is_equal_num_singlenumstringarrays() {
    let a = json!(3);
    let b = json!(["3"]);
    assert!(a.js_eq(&b));
}
#[test]
fn test_is_equal_num_multinumarray() {
    let a = json!(3);
    let b = json!([1, 2]);
    assert!(!a.js_eq(&b));
}
#[test]
fn test_is_equal_array_array() {
    let a = json!([1]);
    let b = json!([1]);
    assert!(!a.js_eq(&b));
}
#[test]
fn test_is_equal_stringarray_stringarray() {
    let a = json!(["1"]);
    let b = json!(["1"]);
    assert!(!a.js_eq(&b));
}
#[test]
fn test_is_equal_emptyarray_emptyarray() {
    let a = json!([]);
    let b = json!([]);
    assert!(!a.js_eq(&b));
}
#[test]
fn test_is_equal_emptyarray_emptystring() {
    let a = json!([]);
    let b = json!("");
    assert!(a.js_eq(&b));
}

#[test]
fn test_is_equal_array_commaseperatedstring() {
    let a = json!([1, 2]);
    let b = json!("1,2");
    assert!(a.js_eq(&b));
}
#[test]
fn test_is_equal_array_string() {
    let a = json!([256]);
    let b = json!("256");
    assert!(a.js_eq(&b));
}
#[test]
fn test_is_equal_nested_array_commaseperatedstring() {
    let a = json!([2, [256]]);
    let b = json!("2,256");
    assert!(a.js_eq(&b));
}
#[test]
fn test_is_equal_emptyarray_commastring() {
    let a = json!([]);
    let b = json!(",");
    assert!(!a.js_eq(&b));
}

#[test]
fn test_is_equal_emptymap_emptymap() {
    let a = json!({});
    let b = json!({});
    assert!(!a.js_eq(&b));
}
#[test]
fn test_is_equal_map_bool() {
    let a = json!({});
    let b = json!(true);
    assert!(!a.js_eq(&b));
}
#[test]
fn test_is_equal_map_falsebool() {
    let a = json!({});
    let b = json!(false);
    assert!(!a.js_eq(&b));
}
#[test]
fn test_is_equal_map_map() {
    let a = json!({"1":1});
    let b = json!({"1":1});
    assert!(!a.js_eq(&b));
}
#[test]
fn test_is_equal_map_array() {
    let a = json!({"1":1});
    let b = json!([1]);
    assert!(!a.js_eq(&b));
}
#[test]
fn test_is_equal_map_num() {
    let a = json!({"1":1});
    let b = json!(1);
    assert!(!a.js_eq(&b));
}
#[test]
fn test_is_equal_map_string() {
    let a = json!({"1":1});
    let b = json!("1");
    assert!(!a.js_eq(&b));
}
#[test]
fn test_is_equal_emptymap_emptyarray() {
    let a = json!({});
    let b = json!([]);
    assert!(!a.js_eq(&b));
}
#[test]
fn test_is_equal_null_bool() {
    let a = json!(null);
    let b = json!(true);
    assert!(!a.js_eq(&b));
}

#[test]
fn test_is_equal_null_falsebool() {
    let a = json!(null);
    let b = json!(false);
    assert!(!a.js_eq(&b));
}

#[test]
fn test_is_equal_null_num() {
    let a = json!(null);
    let b = json!(1);
    assert!(!a.js_eq(&b));
}

#[test]
fn test_is_equal_null_float() {
    let a = json!(null);
    let b = json!(1.0);
    assert!(!a.js_eq(&b));
}

#[test]
fn test_is_equal_null_string() {
    let a = json!(null);
    let b = json!("null");
    assert!(!a.js_eq(&b));
}

#[test]
fn test_is_equal_null_emptystring() {
    let a = json!(null);
    let b = json!("");
    assert!(!a.js_eq(&b));
}

#[test]
fn test_is_equal_null_array() {
    let a = json!(null);
    let b = json!([1, 2, 3]);
    assert!(!a.js_eq(&b));
}

#[test]
fn test_is_equal_null_emptyarray() {
    let a = json!(null);
    let b = json!([]);
    assert!(!a.js_eq(&b));
}

#[test]
fn test_is_equal_null_object() {
    let a = json!(null);
    let b = json!({"key": "value"});
    assert!(!a.js_eq(&b));
}

#[test]
fn test_is_equal_null_emptyobject() {
    let a = json!(null);
    let b = json!({});
    assert!(!a.js_eq(&b));
}

#[test]
fn test_is_equal_null_null() {
    let a = json!(null);
    let b = json!(null);
    assert!(a.js_eq(&b));
}

// less than tests

#[test]
fn test_is_less_than_string_string() {
    let a = json!("abc");
    let b = json!("def");
    assert!(a.js_lt(&b));
}
#[test]
fn test_is_less_than_string_string_2() {
    let a = json!("def");
    let b = json!("abc");
    assert!(!a.js_lt(&b));
}
#[test]
fn test_is_less_than_bool_bool() {
    let a = json!(true);
    let b = json!(false);
    assert!(!a.js_lt(&b));
}
#[test]
fn test_is_less_than_bool_bool_2() {
    let a = json!(false);
    let b = json!(true);
    assert!(a.js_lt(&b));
}
#[test]
fn test_is_less_than_num_num() {
    let a = json!(5);
    let b = json!(10);
    assert!(a.js_lt(&b));
}
#[test]
fn test_is_less_than_num_num_2() {
    let a = json!(10);
    let b = json!(5);
    assert!(!a.js_lt(&b));
}

#[test]
fn test_is_less_than_emptystring_num() {
    let a = json!("");
    let b = json!(1);
    assert!(a.js_lt(&b));
}
#[test]
fn test_is_less_than_string_num() {
    let a = json!("5");
    let b = json!(10);
    assert!(a.js_lt(&b));
}
#[test]
fn test_is_less_than_string_num_2() {
    let a = json!("10");
    let b = json!(5);
    assert!(!a.js_lt(&b));
}
#[test]
fn test_is_less_than_num_string() {
    let a = json!(5);
    let b = json!("10");
    assert!(a.js_lt(&b));
}
#[test]
fn test_is_less_than_num_string_2() {
    let a = json!(10);
    let b = json!("5");
    assert!(!a.js_lt(&b));
}
#[test]
fn test_is_less_than_bool_num() {
    let a = json!(true);
    let b = json!(2);
    assert!(a.js_lt(&b));
}
#[test]
fn test_is_less_than_bool_num_2() {
    let a = json!(false);
    let b = json!(1);
    assert!(a.js_lt(&b));
}
#[test]
fn test_is_less_than_num_bool() {
    let a = json!(0);
    let b = json!(true);
    assert!(a.js_lt(&b));
}
#[test]
fn test_is_less_than_num_bool_2() {
    let a = json!(2);
    let b = json!(false);
    assert!(!a.js_lt(&b));
}

#[test]
fn test_is_less_than_negativenum_bool_2() {
    let a = json!(-1);
    let b = json!(false);
    assert!(a.js_lt(&b));
}

#[test]
fn test_is_less_than_bool_string() {
    let a = json!(false);
    let b = json!("abc");
    assert!(!a.js_lt(&b));
}
#[test]
fn test_is_less_than_string_bool() {
    let a = json!("abc");
    let b = json!(false);
    assert!(!a.js_lt(&b));
}

#[test]
fn test_is_less_than_truebool_string() {
    let a = json!(true);
    let b = json!("abc");
    assert!(!a.js_lt(&b));
}

#[test]
fn test_is_less_than_string_truebool() {
    let a = json!("abc");
    let b = json!(true);
    assert!(!a.js_lt(&b));
}

#[test]
fn test_is_less_than_bool_emptystring() {
    let a = json!(true);
    let b = json!("");
    assert!(!a.js_lt(&b));
}

#[test]
fn test_is_less_than_emptystring_bool() {
    let a = json!("");
    let b = json!(true);
    assert!(a.js_lt(&b));
}

#[test]
fn test_is_less_than_falsebool_emptystring() {
    let a = json!(false);
    let b = json!("");
    assert!(!a.js_lt(&b));
}

#[test]
fn test_is_less_than_emptystring_falsebool() {
    let a = json!("");
    let b = json!(false);
    assert!(!a.js_lt(&b));
}
#[test]
fn test_is_less_than_truebool_numstring() {
    let a = json!(true);
    let b = json!("2");
    assert!(a.js_lt(&b));
}

#[test]
fn test_is_less_than_falsebool_numstring() {
    let a = json!(false);
    let b = json!("1");
    assert!(a.js_lt(&b));
}

#[test]
fn test_is_less_than_numstring_truebool() {
    let a = json!("0");
    let b = json!(true);
    assert!(a.js_lt(&b));
}

#[test]
fn test_is_less_than_numstring_falsebool() {
    let a = json!("2");
    let b = json!(false);
    assert!(!a.js_lt(&b));
}

#[test]
fn test_is_less_than_negativenumstring_falsebool() {
    let a = json!("-1");
    let b = json!(false);
    assert!(a.js_lt(&b));
}
#[test]
fn test_is_less_than_array_bool() {
    let a = json!([1, 2]);
    let b = json!(false);
    assert!(!a.js_lt(&b));
}
#[test]
fn test_is_less_than_array_truebool() {
    let a = json!([1, 2]);
    let b = json!(true);
    assert!(!a.js_lt(&b));
}
#[test]
fn test_is_less_than_array_num() {
    let a = json!([1, 2]);
    let b = json!(-256);
    assert!(!a.js_lt(&b));
}

#[test]
fn test_is_less_than_array_array() {
    let a = json!(["a", "b", "c"]);
    let b = json!(["d", "e", "f"]);
    assert!(a.js_lt(&b));
}
#[test]
fn test_is_less_than_array_array_2() {
    let a = json!(["d", "e", "f"]);
    let b = json!(["a", "b", "c"]);
    assert!(!a.js_lt(&b));
}
#[test]
fn test_is_less_than_array_string() {
    let a = json!(["a", "b", "c"]);
    let b = json!(["d,e,f"]);
    assert!(a.js_lt(&b));
}
#[test]
fn test_is_less_than_string_array() {
    let a = json!(["d,e,f"]);
    let b = json!(["a", "b", "c"]);
    assert!(!a.js_lt(&b));
}
#[test]
fn test_is_less_than_string_nestedarray() {
    let a = json!(["d,e,f"]);
    let b = json!([["a"], "b", "c"]);
    assert!(!a.js_lt(&b));
}
#[test]
fn test_is_less_than_null_null() {
    let a = json!(null);
    let b = json!(null);
    assert!(!a.js_lt(&b));
}

#[test]
fn test_is_less_than_null_num() {
    let a = json!(null);
    let b = json!(1);
    assert!(a.js_lt(&b));
}

#[test]
fn test_is_less_than_num_null() {
    let a = json!(1);
    let b = json!(null);
    assert!(!a.js_lt(&b));
}

#[test]
fn test_is_less_than_null_string() {
    let a = json!(null);
    let b = json!("abc");
    assert!(!a.js_lt(&b));
}

#[test]
fn test_is_less_than_string_null() {
    let a = json!("abc");
    let b = json!(null);
    assert!(!a.js_lt(&b));
}

#[test]
fn test_is_less_than_null_bool() {
    let a = json!(null);
    let b = json!(true);
    assert!(a.js_lt(&b));
}

#[test]
fn test_is_less_than_bool_null() {
    let a = json!(true);
    let b = json!(null);
    assert!(!a.js_lt(&b));
}

#[test]
fn test_is_less_than_null_array() {
    let a = json!(null);
    let b = json!([1, 2, 3]);
    assert!(!a.js_lt(&b));
}

#[test]
fn test_is_less_than_null_singlearray() {
    let a = json!(null);
    let b = json!([1]);
    assert!(a.js_lt(&b));
}
#[test]
fn test_is_less_than_null_singlearray_2() {
    let a = json!(null);
    let b = json!([0]);
    assert!(!a.js_lt(&b));
}
#[test]
fn test_is_less_than_array_null() {
    let a = json!([1, 2, 3]);
    let b = json!(null);
    assert!(!a.js_lt(&b));
}
#[test]
fn test_is_less_than_object_string() {
    let a = json!({"key": "value"});
    let b = json!("Z");
    assert!(!a.js_lt(&b));
}
#[test]
fn test_is_less_than_object_string_2() {
    let a = json!({"key": "value"});
    let b = json!("z");
    assert!(a.js_lt(&b));
}

#[test]
fn test_is_less_than_emptyobject_object() {
    let a = json!({});
    let b = json!({"key": "value"});
    assert!(!a.js_lt(&b));
}

#[test]
fn test_is_less_than_object_num() {
    let a = json!({"key": "value"});
    let b = json!(42);
    assert!(!a.js_lt(&b));
}

#[test]
fn test_is_less_than_num_object() {
    let a = json!(42);
    let b = json!({"key": "value"});
    assert!(!a.js_lt(&b));
}

#[test]
fn test_is_less_than_object_bool() {
    let a = json!({"key": "value"});
    let b = json!(true);
    assert!(!a.js_lt(&b));
}

#[test]
fn test_is_less_than_bool_object() {
    let a = json!(true);
    let b = json!({"key": "value"});
    assert!(!a.js_lt(&b));
}

#[test]
fn test_is_less_than_object_array() {
    let a = json!({"key": "value"});
    let b = json!([1, 2, 3]);
    assert!(!a.js_lt(&b));
}

#[test]
fn test_is_less_than_array_object() {
    let a = json!([1, 2, 3]);
    let b = json!({"key": "value"});
    assert!(a.js_lt(&b));
}

#[test]
fn test_is_less_than_object_null() {
    let a = json!({"key": "value"});
    let b = json!(null);
    assert!(!a.js_lt(&b));
}

#[test]
fn test_is_less_than_null_object() {
    let a = json!(null);
    let b = json!({"key": "value"});
    assert!(!a.js_lt(&b));
}
#[test]
fn sfsdfsdf() {
    let a = json!(0);
    let b = json!(null);
    assert!(!a.js_lt(&b));
}

#[test]
fn test_is_less_than_equalto_emptyarray_string() {
    let a = json!([]);
    let b = json!("abc");
    assert!(a.js_le(&b));
}

#[test]
fn test_is_less_than_equalto_object_nestedarray() {
    let a = json!({"key": "value"});
    let b = json!(["abc"]);
    assert!(a.js_le(&b));
}

#[test]
fn test_is_less_than_equal_string_emptyobject() {
    let a = json!("abc");
    let b = json!({});
    assert!(!a.js_lt(&b));
}
// less than equal to tests
#[test]
fn test_is_less_than_equalto_string_string() {
    let a = json!("abc");
    let b = json!("def");
    assert!(a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_string_string_2() {
    let a = json!("def");
    let b = json!("abc");
    assert!(!a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_bool_bool() {
    let a = json!(true);
    let b = json!(false);
    assert!(!a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_bool_bool_2() {
    let a = json!(false);
    let b = json!(true);
    assert!(a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_num_num() {
    let a = json!(5);
    let b = json!(10);
    assert!(a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_num_num_2() {
    let a = json!(10);
    let b = json!(5);
    assert!(!a.js_le(&b));
}

#[test]
fn test_is_less_than_equalto_emptystring_num() {
    let a = json!("");
    let b = json!(1);
    assert!(a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_string_num() {
    let a = json!("5");
    let b = json!(10);
    assert!(a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_string_num_2() {
    let a = json!("10");
    let b = json!(5);
    assert!(!a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_num_string() {
    let a = json!(5);
    let b = json!("10");
    assert!(a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_num_string_2() {
    let a = json!(10);
    let b = json!("5");
    assert!(!a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_bool_num() {
    let a = json!(true);
    let b = json!(2);
    assert!(a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_bool_num_2() {
    let a = json!(false);
    let b = json!(1);
    assert!(a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_num_bool() {
    let a = json!(0);
    let b = json!(true);
    assert!(a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_num_bool_2() {
    let a = json!(2);
    let b = json!(false);
    assert!(!a.js_le(&b));
}

#[test]
fn test_is_less_than_equalto_negativenum_bool_2() {
    let a = json!(-1);
    let b = json!(false);
    assert!(a.js_le(&b));
}

#[test]
fn test_is_less_than_equalto_bool_string() {
    let a = json!(false);
    let b = json!("abc");
    assert!(!a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_string_bool() {
    let a = json!("abc");
    let b = json!(false);
    assert!(!a.js_le(&b));
}

#[test]
fn test_is_less_than_equalto_truebool_string() {
    let a = json!(true);
    let b = json!("abc");
    assert!(!a.js_le(&b));
}

#[test]
fn test_is_less_than_equalto_string_truebool() {
    let a = json!("abc");
    let b = json!(true);
    assert!(!a.js_le(&b));
}

#[test]
fn test_is_less_than_equalto_bool_emptystring() {
    let a = json!(true);
    let b = json!("");
    assert!(!a.js_le(&b));
}

#[test]
fn test_is_less_than_equalto_emptystring_bool() {
    let a = json!("");
    let b = json!(true);
    assert!(a.js_le(&b));
}

#[test]
fn test_is_less_than_equalto_falsebool_emptystring() {
    let a = json!(false);
    let b = json!("");
    assert!(a.js_le(&b));
}

#[test]
fn test_is_less_than_equalto_emptystring_falsebool() {
    let a = json!("");
    let b = json!(false);
    assert!(a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_truebool_numstring() {
    let a = json!(true);
    let b = json!("2");
    assert!(a.js_le(&b));
}

#[test]
fn test_is_less_than_equalto_falsebool_numstring() {
    let a = json!(false);
    let b = json!("1");
    assert!(a.js_le(&b));
}

#[test]
fn test_is_less_than_equalto_numstring_truebool() {
    let a = json!("0");
    let b = json!(true);
    assert!(a.js_le(&b));
}

#[test]
fn test_is_less_than_equalto_numstring_falsebool() {
    let a = json!("2");
    let b = json!(false);
    assert!(!a.js_le(&b));
}

#[test]
fn test_is_less_than_equalto_negativenumstring_falsebool() {
    let a = json!("-1");
    let b = json!(false);
    assert!(a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_array_bool() {
    let a = json!([1, 2]);
    let b = json!(false);
    assert!(!a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_array_truebool() {
    let a = json!([1, 2]);
    let b = json!(true);
    assert!(!a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_array_num() {
    let a = json!([1, 2]);
    let b = json!(-256);
    assert!(!a.js_le(&b));
}

#[test]
fn test_is_less_than_equalto_array_array() {
    let a = json!(["a", "b", "c"]);
    let b = json!(["d", "e", "f"]);
    assert!(a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_array_array_2() {
    let a = json!(["d", "e", "f"]);
    let b = json!(["a", "b", "c"]);
    assert!(!a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_array_string() {
    let a = json!(["a", "b", "c"]);
    let b = json!(["d,e,f"]);
    assert!(a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_string_array() {
    let a = json!(["d,e,f"]);
    let b = json!(["a", "b", "c"]);
    assert!(!a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_string_nestedarray() {
    let a = json!(["d,e,f"]);
    let b = json!([["a"], "b", "c"]);
    assert!(!a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_null_null() {
    let a = json!(null);
    let b = json!(null);
    assert!(a.js_le(&b));
}

#[test]
fn test_is_less_than_equalto_null_num() {
    let a = json!(null);
    let b = json!(1);
    assert!(a.js_le(&b));
}

#[test]
fn test_is_less_than_equalto_num_null() {
    let a = json!(1);
    let b = json!(null);
    assert!(!a.js_le(&b));
}

#[test]
fn test_is_less_than_equalto_null_string() {
    let a = json!(null);
    let b = json!("abc");
    assert!(!a.js_le(&b));
}

#[test]
fn test_is_less_than_equalto_string_null() {
    let a = json!("abc");
    let b = json!(null);
    assert!(!a.js_le(&b));
}

#[test]
fn test_is_less_than_equalto_null_bool() {
    let a = json!(null);
    let b = json!(true);
    assert!(a.js_le(&b));
}

#[test]
fn test_is_less_than_equalto_bool_null() {
    let a = json!(true);
    let b = json!(null);
    assert!(!a.js_le(&b));
}

#[test]
fn test_is_less_than_equalto_null_array() {
    let a = json!(null);
    let b = json!([1, 2, 3]);
    assert!(!a.js_le(&b));
}

#[test]
fn test_is_less_than_equalto_null_singlearray() {
    let a = json!(null);
    let b = json!([1]);
    assert!(a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_null_singlearray_2() {
    let a = json!(null);
    let b = json!([0]);
    assert!(a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_array_null() {
    let a = json!([1, 2, 3]);
    let b = json!(null);
    assert!(!a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_object_string() {
    let a = json!({"key": "value"});
    let b = json!("Z");
    assert!(!a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_object_string_2() {
    let a = json!({"key": "value"});
    let b = json!("z");
    assert!(a.js_le(&b));
}

#[test]
fn test_is_less_than_equalto_emptyobject_object() {
    let a = json!({});
    let b = json!({"key": "value"});
    assert!(a.js_le(&b));
}

#[test]
fn test_is_less_than_equalto_object_num() {
    let a = json!({"key": "value"});
    let b = json!(42);
    assert!(!a.js_le(&b));
}

#[test]
fn test_is_less_than_equalto_num_object() {
    let a = json!(42);
    let b = json!({"key": "value"});
    assert!(!a.js_le(&b));
}

#[test]
fn test_is_less_than_equalto_object_bool() {
    let a = json!({"key": "value"});
    let b = json!(true);
    assert!(!a.js_le(&b));
}

#[test]
fn test_is_less_than_equalto_bool_object() {
    let a = json!(true);
    let b = json!({"key": "value"});
    assert!(!a.js_le(&b));
}

#[test]
fn test_is_less_than_equalto_object_array() {
    let a = json!({"key": "value"});
    let b = json!([1, 2, 3]);
    assert!(!a.js_le(&b));
}

#[test]
fn test_is_less_than_equalto_array_object() {
    let a = json!([1, 2, 3]);
    let b = json!({"key": "value"});
    assert!(a.js_le(&b));
}

#[test]
fn test_is_less_than_equalto_object_null() {
    let a = json!({"key": "value"});
    let b = json!(null);
    assert!(!a.js_le(&b));
}

#[test]
fn test_is_less_than_equalto_null_object() {
    let a = json!(null);
    let b = json!({"key": "value"});
    assert!(!a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_null_falsebool() {
    let a = json!(null);
    let b = json!(false);
    assert!(a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_null_zeronum() {
    let a = json!(null);
    let b = json!(0);
    assert!(a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_null_emptystring() {
    let a = json!(null);
    let b = json!("");
    assert!(a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_null_emptyarray() {
    let a = json!(null);
    let b = json!([]);
    assert!(a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_num_num_3() {
    let a = json!(256);
    let b = json!(256);
    assert!(a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_string_string_3() {
    let a = json!("abc");
    let b = json!("abc");
    assert!(a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_array_array_4() {
    let a = json!([1, 2, 3]);
    let b = json!([1, 2, 3]);
    assert!(a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_singleelementarray_bool() {
    let a = json!([0]);
    let b = json!(false);
    assert!(a.js_le(&b));
}
#[test]
fn test_is_less_than_equalto_object_object() {
    let a = json!({});
    let b = json!({});
    assert!(a.js_le(&b));
}
