use serde_json::{json, Value};

/// Condition names accepted by [`array_filter`].
pub const EQUAL: &str = "equal";
pub const NOT_EQUAL: &str = "notEqual";
pub const GREATER_THAN: &str = "greaterThan";
pub const GREATER_THAN_EQUAL: &str = "greaterThanEqual";
pub const LESS_THAN: &str = "lessThan";
pub const LESS_THAN_EQUAL: &str = "lessThanEqual";
pub const CONTAINS: &str = "contains";
pub const IS_NULL: &str = "isNull";
pub const IS_NOT_NULL: &str = "isNotNull";

fn build(method: &str, values: Value) -> String {
    json!({ "method": method, "values": values }).to_string()
}

pub fn increment(value: Value, max: Option<Value>) -> String {
    let values = match max {
        Some(max) => json!([value, max]),
        None => json!([value]),
    };
    build("increment", values)
}

pub fn decrement(value: Value, min: Option<Value>) -> String {
    let values = match min {
        Some(min) => json!([value, min]),
        None => json!([value]),
    };
    build("decrement", values)
}

pub fn multiply(factor: Value, max: Option<Value>) -> String {
    let values = match max {
        Some(max) => json!([factor, max]),
        None => json!([factor]),
    };
    build("multiply", values)
}

pub fn divide(divisor: Value, min: Option<Value>) -> String {
    let values = match min {
        Some(min) => json!([divisor, min]),
        None => json!([divisor]),
    };
    build("divide", values)
}

pub fn modulo(divisor: Value) -> String {
    build("modulo", json!([divisor]))
}

pub fn power(exponent: Value, max: Option<Value>) -> String {
    let values = match max {
        Some(max) => json!([exponent, max]),
        None => json!([exponent]),
    };
    build("power", values)
}

pub fn array_append(values: Vec<Value>) -> String {
    build("arrayAppend", Value::Array(values))
}

pub fn array_prepend(values: Vec<Value>) -> String {
    build("arrayPrepend", Value::Array(values))
}

pub fn array_insert(index: i64, value: Value) -> String {
    build("arrayInsert", json!([index, value]))
}

pub fn array_remove(value: Value) -> String {
    build("arrayRemove", json!([value]))
}

pub fn array_unique() -> String {
    build("arrayUnique", json!([]))
}

pub fn array_intersect(values: Vec<Value>) -> String {
    build("arrayIntersect", Value::Array(values))
}

pub fn array_diff(values: Vec<Value>) -> String {
    build("arrayDiff", Value::Array(values))
}

pub fn array_filter(condition: &str, value: Option<Value>) -> String {
    let value = value.unwrap_or(Value::Null);
    build("arrayFilter", json!([condition, value]))
}

pub fn string_concat(value: Value) -> String {
    build("stringConcat", json!([value]))
}

pub fn string_replace(search: &str, replace: &str) -> String {
    build("stringReplace", json!([search, replace]))
}

pub fn toggle() -> String {
    build("toggle", json!([]))
}

pub fn date_add_days(days: i64) -> String {
    build("dateAddDays", json!([days]))
}

pub fn date_sub_days(days: i64) -> String {
    build("dateSubDays", json!([days]))
}

pub fn date_set_now() -> String {
    build("dateSetNow", json!([]))
}
