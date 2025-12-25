use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use nu_protocol::{Record, Span, Value};
use toon::serde_json::{Map, Number, Value as JsonValue};

pub fn nu_value_to_toon_string_value(val: &Value) -> Value {
    let json = nu_value_to_json(val);
    let encoded = toon::encode(&json, None);
    Value::string(encoded, Span::unknown())
}

pub fn nu_value_to_json(val: &Value) -> JsonValue {
    match val {
        Value::Bool { val, .. } => JsonValue::Bool(*val),
        Value::Int { val, .. } => JsonValue::Number(Number::from(*val)),
        Value::Float { val, .. } => Number::from_f64(*val)
            .map(JsonValue::Number)
            .unwrap_or(JsonValue::Null),
        Value::String { val, .. } => JsonValue::String(val.to_string()),
        Value::Glob { val, .. } => JsonValue::String(val.to_string()),
        Value::Filesize { val, .. } => JsonValue::String(val.to_string()),
        Value::Duration { val, .. } => JsonValue::String(val.to_string()),
        Value::Date { val, .. } => JsonValue::String(val.to_string()),
        Value::Range { val, .. } => JsonValue::String(val.to_string()),
        Value::Record { val, .. } => record_to_json(val),
        Value::List { vals, .. } => {
            JsonValue::Array(vals.iter().map(nu_value_to_json).collect())
        }
        Value::Closure { .. } => JsonValue::Null,
        Value::Error { error, .. } => JsonValue::String(error.to_string()),
        Value::Binary { val, .. } => {
            let b64 = BASE64_STANDARD.encode(val);
            JsonValue::String(b64)
        }
        Value::CellPath { val, .. } => JsonValue::String(val.to_string()),
        Value::Custom { .. } => JsonValue::Null,
        Value::Nothing { .. } => JsonValue::Null,
    }
}

fn record_to_json(record: &Record) -> JsonValue {
    let mut map = Map::new();
    for (key, value) in record.iter() {
        map.insert(key.to_string(), nu_value_to_json(value));
    }
    JsonValue::Object(map)
}
