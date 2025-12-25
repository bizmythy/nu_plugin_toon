use json2toon_rs::DecoderOptions;
use nu_protocol::{Record, Span, Value};
use toon::serde_json::Value as JsonValue;

pub fn toon_to_nu_value(input: &str) -> Result<Value, String> {
    let json = json2toon_rs::decode(input, &DecoderOptions::default())
        .map_err(|e| e.to_string())?;
    Ok(json_to_nu_value(&json, Span::unknown()))
}

pub fn json_to_nu_value(val: &JsonValue, span: Span) -> Value {
    match val {
        JsonValue::Null => Value::nothing(span),
        JsonValue::Bool(val) => Value::bool(*val, span),
        JsonValue::Number(val) => {
            if let Some(i) = val.as_i64() {
                Value::int(i, span)
            } else if let Some(u) = val.as_u64() {
                Value::int(u as i64, span)
            } else if let Some(f) = val.as_f64() {
                Value::float(f, span)
            } else {
                Value::nothing(span)
            }
        }
        JsonValue::String(val) => Value::string(val.to_string(), span),
        JsonValue::Array(vals) => {
            Value::list(vals.iter().map(|x| json_to_nu_value(x, span)).collect(), span)
        }
        JsonValue::Object(map) => {
            let mut record = Record::new();
            for (key, value) in map {
                record.push(key, json_to_nu_value(value, span));
            }
            Value::record(record, span)
        }
    }
}
