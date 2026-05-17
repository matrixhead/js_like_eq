use serde_json::{Value, json};

pub trait JavaScriptEquality {
    fn equals(&self, other: &Self) -> bool;
    fn less_than(&self, other: &Self) -> bool;
    fn less_than_equal_to(&self, other: &Self) -> bool;
    fn greater(&self, other: &Self) -> bool;
    fn is_strict_equal(&self, other: &Self) -> bool;
    fn is_not_equal(&self, other: &Self) -> bool;
    fn is_not_strict_equal(&self, other: &Self) -> bool;
    fn coerce_and_compare(
        &self,
        rhs: &Self,
        bool_compare: impl Fn(&bool, &bool) -> bool,
        string_compare: impl Fn(&str, &str) -> bool,
        num_compare: impl Fn(&f64, &f64) -> bool,
    ) -> bool;
}

pub fn good_enough_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < std::f64::EPSILON
}

pub trait JsonTruthy {
    fn is_truthy(&self, include_zero: bool) -> bool;
}

impl JsonTruthy for Value {
    fn is_truthy(&self, _include_zero: bool) -> bool {
        match *self {
            Value::Bool(ref i) => *i,
            Value::Number(ref n) => n.as_f64().is_some_and(|f| f == 1.0),

            Value::Null => false,
            Value::String(ref i) => !i.is_empty(),
            Value::Array(ref i) => !i.is_empty(),
            Value::Object(ref i) => !i.is_empty(),
        }
    }
}

impl JavaScriptEquality for Value {
    fn equals(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Null, Value::Null) => true,
            // In JavaScript, when we compare null, arrays, or objects to
            // anything else, including their own type with identical values (not null), it
            // returns false.
            (Value::Null, _) | (_, Value::Null) => false,
            (Value::Array(_), Value::Array(_)) => false,
            (_, Value::Object(_)) | (Value::Object(_), _) => false,
            // These types can be safely compared with their own type without any hidden caveats.
            (Value::Bool(_), Value::Bool(_)) | (Value::String(_), Value::String(_)) => {
                self == other
            }

            // If the values are numbers, compare them as numbers. with precision tolerance.
            (Value::Number(a), Value::Number(b)) => {
                if let (Some(a), Some(b)) = (a.as_f64(), b.as_f64()) {
                    return good_enough_equal(a, b);
                }
                false
            }
            // If one value is a number and the other is a string, attempt to parse the string as a number.
            (Value::Number(n), Value::String(s)) | (Value::String(s), Value::Number(n)) => {
                if let Ok(parsed) = s.parse::<f64>() {
                    if let Some(num) = n.as_f64() {
                        return good_enough_equal(parsed, num);
                    }
                }
                false
            }
            // If one value is a boolean and the other is a number, compare the boolean as a number.
            (Value::Bool(b), Value::Number(n)) | (Value::Number(n), Value::Bool(b)) => {
                match (n.as_f64(), b) {
                    (Some(num), true) => num == 1.0,
                    (Some(num), false) => num == 0.0,
                    _ => false,
                }
            }
            // If one value is a boolean and the other is a string, attempt to parse the string as a number.
            (Value::Bool(b), Value::String(s)) | (Value::String(s), Value::Bool(b)) => {
                // empty strings has an intrinsic value of false
                if s.is_empty() {
                    return !b;
                }
                match (s.parse::<f64>(), b) {
                    (Ok(parsed), true) => parsed == 1.0,
                    (Ok(parsed), false) => parsed == 0.0,
                    _ => false,
                }
            }
            // in JS comma separated strings are treated as arrays
            (Value::String(s), Value::Array(values)) | (Value::Array(values), Value::String(s)) => {
                // empty strings and empty arrays are equal
                if s.is_empty() && values.is_empty() {
                    return true;
                }
                let mut s_elements = s.split(",");
                let mut array_elements = values.iter();
                loop {
                    match (array_elements.next(), s_elements.next()) {
                        (Some(array_value), Some(string_element)) => {
                            if !array_value.equals(&Value::String(string_element.to_string())) {
                                return false;
                            }
                        }
                        (None, None) => return true,
                        _ => return false,
                    }
                }
            }
            // In JavaScript, appart from empty arrays all arrays are considered
            // nor false nor true, when compared with another value it is
            // considered as false, unless they have a single element. In that
            // case, the array is treated as that element and compared.
            (other, Value::Array(values)) | (Value::Array(values), other) => {
                if values.len() == 1 {
                    return other.equals(&values[0]);
                } else if let Value::Bool(b) = other {
                    // empty arrays have an intrinsic value of false
                    if values.is_empty() {
                        return !b;
                    }
                }
                false
            }
        }
    }

    /// the assumption is values will be compared most primitive to higher order datatypes
    /// if mixed data type boolen wiil get converted to number always
    fn less_than(self: &Value, rhs: &Self) -> bool {
        self.coerce_and_compare(
            rhs,
            |lhs, rhs| lhs < rhs,
            |lhs, rhs| lhs < rhs,
            |lhs, rhs| lhs < rhs,
        )
    }
    fn less_than_equal_to(&self, rhs: &Self) -> bool {
        self.coerce_and_compare(
            rhs,
            |lhs, rhs| lhs <= rhs,
            |lhs, rhs| lhs <= rhs,
            |lhs, rhs| lhs <= rhs,
        )
    }

    fn greater(&self, other: &Self) -> bool {
        other.less_than(self)
    }

    fn is_strict_equal(&self, _other: &Self) -> bool {
        todo!()
    }

    fn is_not_strict_equal(&self, _other: &Self) -> bool {
        todo!()
    }

    fn is_not_equal(&self, other: &Self) -> bool {
        !Self::equals(self, other)
    }

    /// Coerces and compares two JSON `Value` objects based on JavaScript-like type coercion rules.
    ///
    /// # Coercion Rules assumptions
    /// ## we are trying to mimic the JS coercion rules in this function. we are making lot of assumptions about how the coercion works.
    /// 1.  All values get coerced into either `number`, or `string` for comparison.
    /// 1.  number and  string could be compared with their own type.
    /// 1.  `null` get coerced into `0` before comparison.
    /// 1.  `bool` values are coerced into `number` values before comparison.
    /// 1.  'array
    /// 1.  if we encounter a number and a string, we will try to parse the string
    ///     as a number. if it fails,that means we comparing `Nan` with a `number`.
    ///     we will return false.
    /// 1.  one exception to the above rule is when a string is empty, in that case
    ///     we parse the string to number `0` and compare.
    /// 1.  empty arrays are also considered as `0` for comparison. unless the other
    ///     value is a string or an array in that case it's treated as a normal
    ///     array with multiple elements.
    /// 1.  if the array has a single element, we will treat the array as that
    ///     single element and compare but just like the empty array if the other
    ///     value is a string or an array we would treat it as a normal array with
    ///     multiple elements.
    /// 1.  normal arrays with multiple elements are treated as strings for comparison. for that JS calls `.toString()` on arrays it would essentially convert array into a string of comma seperated values.
    fn coerce_and_compare(
        &self,
        rhs: &Self,
        bool_compare: impl Fn(&bool, &bool) -> bool,
        string_compare: impl Fn(&str, &str) -> bool,
        num_compare: impl Fn(&f64, &f64) -> bool,
    ) -> bool {
        let lhs = self;
        match (lhs, rhs) {
            // when comparing with relational operators, null is coerced to 0
            (Value::Null, _) => {
                json!(0).coerce_and_compare(rhs, bool_compare, string_compare, num_compare)
            }
            (_, Value::Null) => {
                lhs.coerce_and_compare(&json!(0), bool_compare, string_compare, num_compare)
            }
            // These types can be safely compared with their own type without any hidden caveats.
            (Value::Bool(lhs_bool), Value::Bool(rhs_bool)) => bool_compare(lhs_bool, rhs_bool),
            // we are doing a lexicographical comparison
            (Value::String(lhs_string), Value::String(rhs_string)) => {
                string_compare(lhs_string, rhs_string)
            }
            (Value::Number(lhs_num), Value::Number(rhs_num)) => {
                if let (Some(lhs_num), Some(rhs_num)) = (lhs_num.as_f64(), rhs_num.as_f64()) {
                    return num_compare(&lhs_num, &rhs_num);
                }
                false
            }
            (Value::Number(_), Value::String(string))
            | (Value::String(string), Value::Number(_)) => {
                let s_parsed: Value = if string.is_empty() {
                    json!(0)
                } else {
                    match string.parse::<f64>() {
                        Ok(parsed) => json!(parsed),
                        // nan compared with anything is false
                        _ => return false,
                    }
                };
                if matches!((lhs, rhs), (Value::Number(_), Value::String(_))) {
                    lhs.coerce_and_compare(&s_parsed, bool_compare, string_compare, num_compare)
                } else {
                    s_parsed.coerce_and_compare(rhs, bool_compare, string_compare, num_compare)
                }
            }
            // If one value is a boolean and the other is a number, compare the boolean as a number.
            (_, Value::Bool(rhs_bool)) => match rhs_bool {
                true => {
                    lhs.coerce_and_compare(&json!(1), bool_compare, string_compare, num_compare)
                }
                false => {
                    lhs.coerce_and_compare(&json!(0), bool_compare, string_compare, num_compare)
                }
            },
            // If one value is a boolean and the other is a number, compare the boolean as a number.
            (Value::Bool(lhs_bool), _) => match lhs_bool {
                true => json!(1).coerce_and_compare(rhs, bool_compare, string_compare, num_compare),
                false => {
                    json!(0).coerce_and_compare(rhs, bool_compare, string_compare, num_compare)
                }
            },
            (Value::Object(_), _) | (_, Value::Object(_)) => {
                if matches!(lhs, Value::Object(_)) {
                    return json!("[object Object]").coerce_and_compare(
                        rhs,
                        bool_compare,
                        string_compare,
                        num_compare,
                    );
                } else {
                    return lhs.coerce_and_compare(
                        &json!("[object Object]"),
                        bool_compare,
                        string_compare,
                        num_compare,
                    );
                }
            }
            (Value::Array(values), other) | (other, Value::Array(values)) => {
                let array_coerced = {
                    if values.len() == 1 && !matches!(other, Value::String(_) | Value::Array(_)) {
                        values[0].clone()
                    } else if values.is_empty()
                        && !matches!(other, Value::String(_) | Value::Array(_))
                    {
                        json!(false)
                    } else {
                        match other {
                            Value::Bool(_)
                            | Value::Number(_)
                            | Value::String(_)
                            | Value::Array(_) => {
                                json!(array_to_string(values))
                            }
                            Value::Object(_) | Value::Null => unreachable!(),
                        }
                    }
                };
                if matches!(lhs, Value::Array(_)) {
                    return array_coerced.coerce_and_compare(
                        other,
                        bool_compare,
                        string_compare,
                        num_compare,
                    );
                } else {
                    return other.coerce_and_compare(
                        &array_coerced,
                        bool_compare,
                        string_compare,
                        num_compare,
                    );
                }
            }
        }
    }
}

pub fn array_to_string(vec: &Vec<Value>) -> String {
    vec.iter()
        .map(|item| match item {
            Value::String(string) => string.clone(),
            Value::Array(values) => array_to_string(values),
            Value::Object(_) => "[object Object]".to_string(),
            _ => item.to_string(),
        })
        .collect::<Vec<String>>()
        .join(",")
}



#[cfg(test)]
mod node_cross_test;

#[cfg(test)]
mod tests;
