use serde_json::Value;
use std::io;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut json = String::new();
    io::stdin()
        .read_to_string(&mut json)
        .expect("Failed to read line.");
    let mut result = String::from("");
    let data: Value = serde_json::from_str(&json)?;
    pretty_deeper(&mut result, &data, 1);
    println!("{}", result);

    Ok(())
}

fn pretty_deeper(result: &mut String, x: &Value, depth: usize) {
    let small_space = " ".repeat((depth - 1) * 4);
    let space = " ".repeat(depth * 4);
    match x {
        Value::Null => result.push_str("null"),
        Value::Bool(_) => result.push_str("boolean"),
        Value::Number(_) => result.push_str("number"),
        Value::String(_) => result.push_str("string"),
        Value::Array(array) => {
            if array.len() > 0 {
                pretty_deeper(result, &array[0], depth);
            } else {
                result.push_str("undefined");
            }
            result.push_str("[]");
        }
        Value::Object(values) => {
            result.push_str("{\n");
            for (key, value) in values {
                result.push_str(&format!("{}{}: ", &space.clone(), key));
                pretty_deeper(result, value, depth + 1);
                result.push_str(",\n");
            }
            result.push_str(&format!("{}}}", small_space));
        }
    }
}
