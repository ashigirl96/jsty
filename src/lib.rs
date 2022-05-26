use serde_json::Value;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::io::{BufWriter, Read, Write};
use std::{env, io};

pub fn fmt(value: &Value) -> String {
    let mut result = String::new();
    match value {
        Value::Null => result.push_str("null"),
        Value::Bool(_) => result.push_str("boolean"),
        Value::Number(_) => result.push_str("number"),
        Value::String(_) => result.push_str("string"),
        Value::Array(ref array) => {
            if array.len() > 0 {
                result.push_str(format!("{}[]", fmt(&array[0])).as_str())
            }
        }
        Value::Object(ref v) => {
            result.push_str("{");
            for (key, value) in v {
                result.push_str(format!("{}: {};", key, fmt(value)).as_str())
            }
            result.push_str("}");
        }
    };
    result
}

pub fn fmt_pretty(value: &Value, depth: usize) -> String {
    let mut result = String::new();
    match value {
        Value::Null => result.push_str("null"),
        Value::Bool(_) => result.push_str("boolean"),
        Value::Number(_) => result.push_str("number"),
        Value::String(_) => result.push_str("string"),
        Value::Array(ref array) => {
            if array.len() > 0 {
                result.push_str(format!("{}", fmt_pretty(&array[0], depth)).as_str())
            }
            result.push_str("[]");
        }
        Value::Object(ref v) => {
            result.push_str("{\n");
            for (key, value) in v {
                result.push_str(
                    format!(
                        "{}{}: {};\n",
                        space(depth),
                        key,
                        fmt_pretty(value, depth + 1)
                    )
                    .as_str(),
                )
            }
            result.push_str(space(depth - 1).as_str());
            result.push_str("}");
        }
    };
    result
}

fn space(depth: usize) -> String {
    " ".repeat(depth * 4)
}
