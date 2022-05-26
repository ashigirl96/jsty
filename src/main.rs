use serde_json::Value;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::io;
use std::io::{BufWriter, Read, Write};

fn fmt(value: &Value) -> String {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut json = String::new();
    io::stdin()
        .read_to_string(&mut json)
        .expect("Failed to read line.");
    let data = serde_json::from_str(&json)?;

    let stdout = io::stdout();
    let mut output = BufWriter::new(stdout.lock());
    writeln!(output, "type Hello = {}", fmt(&data));

    Ok(())
}
