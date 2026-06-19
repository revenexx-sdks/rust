use serde_json::{json, Value};

fn to_array(value: Value) -> Value {
    match value {
        Value::Null => Value::Array(vec![]),
        Value::Array(_) => value,
        other => Value::Array(vec![other]),
    }
}

fn build(method: &str, attribute: Option<&str>, values: Option<Value>) -> String {
    let mut obj = serde_json::Map::new();
    obj.insert("method".to_string(), json!(method));
    if let Some(attribute) = attribute {
        obj.insert("attribute".to_string(), json!(attribute));
    }
    if let Some(values) = values {
        obj.insert("values".to_string(), values);
    }
    Value::Object(obj).to_string()
}

fn parse_queries(queries: &[String]) -> Value {
    let parsed: Vec<Value> = queries
        .iter()
        .filter_map(|q| serde_json::from_str(q).ok())
        .collect();
    Value::Array(parsed)
}

pub fn equal(attribute: &str, value: Value) -> String {
    build("equal", Some(attribute), Some(to_array(value)))
}

pub fn not_equal(attribute: &str, value: Value) -> String {
    build("notEqual", Some(attribute), Some(to_array(value)))
}

pub fn less_than(attribute: &str, value: Value) -> String {
    build("lessThan", Some(attribute), Some(to_array(value)))
}

pub fn less_than_equal(attribute: &str, value: Value) -> String {
    build("lessThanEqual", Some(attribute), Some(to_array(value)))
}

pub fn greater_than(attribute: &str, value: Value) -> String {
    build("greaterThan", Some(attribute), Some(to_array(value)))
}

pub fn greater_than_equal(attribute: &str, value: Value) -> String {
    build("greaterThanEqual", Some(attribute), Some(to_array(value)))
}

pub fn search(attribute: &str, value: Value) -> String {
    build("search", Some(attribute), Some(to_array(value)))
}

pub fn is_null(attribute: &str) -> String {
    build("isNull", Some(attribute), None)
}

pub fn is_not_null(attribute: &str) -> String {
    build("isNotNull", Some(attribute), None)
}

pub fn between(attribute: &str, start: Value, end: Value) -> String {
    build("between", Some(attribute), Some(json!([start, end])))
}

pub fn starts_with(attribute: &str, value: Value) -> String {
    build("startsWith", Some(attribute), Some(to_array(value)))
}

pub fn ends_with(attribute: &str, value: Value) -> String {
    build("endsWith", Some(attribute), Some(to_array(value)))
}

pub fn contains(attribute: &str, value: Value) -> String {
    build("contains", Some(attribute), Some(to_array(value)))
}

pub fn contains_any(attribute: &str, values: Vec<Value>) -> String {
    build("containsAny", Some(attribute), Some(Value::Array(values)))
}

pub fn contains_all(attribute: &str, values: Vec<Value>) -> String {
    build("containsAll", Some(attribute), Some(Value::Array(values)))
}

pub fn not_contains(attribute: &str, value: Value) -> String {
    build("notContains", Some(attribute), Some(to_array(value)))
}

pub fn not_search(attribute: &str, value: Value) -> String {
    build("notSearch", Some(attribute), Some(to_array(value)))
}

pub fn not_between(attribute: &str, start: Value, end: Value) -> String {
    build("notBetween", Some(attribute), Some(json!([start, end])))
}

pub fn not_starts_with(attribute: &str, value: Value) -> String {
    build("notStartsWith", Some(attribute), Some(to_array(value)))
}

pub fn not_ends_with(attribute: &str, value: Value) -> String {
    build("notEndsWith", Some(attribute), Some(to_array(value)))
}

pub fn created_before(value: Value) -> String {
    less_than("$createdAt", value)
}

pub fn created_after(value: Value) -> String {
    greater_than("$createdAt", value)
}

pub fn created_between(start: Value, end: Value) -> String {
    between("$createdAt", start, end)
}

pub fn updated_before(value: Value) -> String {
    less_than("$updatedAt", value)
}

pub fn updated_after(value: Value) -> String {
    greater_than("$updatedAt", value)
}

pub fn updated_between(start: Value, end: Value) -> String {
    between("$updatedAt", start, end)
}

pub fn select(attributes: Vec<String>) -> String {
    build("select", None, Some(json!(attributes)))
}

pub fn order_asc(attribute: &str) -> String {
    build("orderAsc", Some(attribute), None)
}

pub fn order_desc(attribute: &str) -> String {
    build("orderDesc", Some(attribute), None)
}

pub fn order_random() -> String {
    build("orderRandom", None, None)
}

pub fn cursor_before(document_id: &str) -> String {
    build("cursorBefore", None, Some(json!([document_id])))
}

pub fn cursor_after(document_id: &str) -> String {
    build("cursorAfter", None, Some(json!([document_id])))
}

pub fn limit(limit: i64) -> String {
    build("limit", None, Some(json!([limit])))
}

pub fn offset(offset: i64) -> String {
    build("offset", None, Some(json!([offset])))
}

pub fn or(queries: &[String]) -> String {
    build("or", None, Some(parse_queries(queries)))
}

pub fn and(queries: &[String]) -> String {
    build("and", None, Some(parse_queries(queries)))
}

pub fn distance_equal(attribute: &str, values: Value, distance: f64, meters: bool) -> String {
    build("distanceEqual", Some(attribute), Some(json!([[values, distance, meters]])))
}

pub fn distance_not_equal(attribute: &str, values: Value, distance: f64, meters: bool) -> String {
    build("distanceNotEqual", Some(attribute), Some(json!([[values, distance, meters]])))
}

pub fn distance_greater_than(attribute: &str, values: Value, distance: f64, meters: bool) -> String {
    build("distanceGreaterThan", Some(attribute), Some(json!([[values, distance, meters]])))
}

pub fn distance_less_than(attribute: &str, values: Value, distance: f64, meters: bool) -> String {
    build("distanceLessThan", Some(attribute), Some(json!([[values, distance, meters]])))
}

pub fn intersects(attribute: &str, values: Value) -> String {
    build("intersects", Some(attribute), Some(json!([values])))
}

pub fn not_intersects(attribute: &str, values: Value) -> String {
    build("notIntersects", Some(attribute), Some(json!([values])))
}

pub fn crosses(attribute: &str, values: Value) -> String {
    build("crosses", Some(attribute), Some(json!([values])))
}

pub fn not_crosses(attribute: &str, values: Value) -> String {
    build("notCrosses", Some(attribute), Some(json!([values])))
}

pub fn overlaps(attribute: &str, values: Value) -> String {
    build("overlaps", Some(attribute), Some(json!([values])))
}

pub fn not_overlaps(attribute: &str, values: Value) -> String {
    build("notOverlaps", Some(attribute), Some(json!([values])))
}

pub fn touches(attribute: &str, values: Value) -> String {
    build("touches", Some(attribute), Some(json!([values])))
}

pub fn not_touches(attribute: &str, values: Value) -> String {
    build("notTouches", Some(attribute), Some(json!([values])))
}

pub fn regex(attribute: &str, pattern: &str) -> String {
    build("regex", Some(attribute), Some(json!([pattern])))
}

pub fn exists(attributes: Vec<String>) -> String {
    build("exists", None, Some(json!(attributes)))
}

pub fn not_exists(attributes: Vec<String>) -> String {
    build("notExists", None, Some(json!(attributes)))
}

pub fn elem_match(attribute: &str, queries: &[String]) -> String {
    build("elemMatch", Some(attribute), Some(parse_queries(queries)))
}
