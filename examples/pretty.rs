use jsty::fmt_pretty;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::io::{BufWriter, Read, Write};
use std::{env, io};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let interface_name = if args.len() == 2 {
        args[1].clone()
    } else {
        String::from("Hello")
    };
    let mut json = String::new();
    io::stdin()
        .read_to_string(&mut json)
        .expect("Failed to read line.");
    let data = serde_json::from_str(&json)?;

    let stdout = io::stdout();
    let mut output = BufWriter::new(stdout.lock());
    writeln!(
        output,
        "interface {} {}",
        interface_name,
        fmt_pretty(&data, 1)
    );

    Ok(())
}
