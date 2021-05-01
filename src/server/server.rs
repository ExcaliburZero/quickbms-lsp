use std::io::{BufRead, BufReader, Write};

use jsonrpc_core::{IoHandler, Params};
use serde_json::{self, from_value, Value};

pub fn run<R, W>(mut input: R, mut output: W)
where
    R: BufRead,
    W: Write,
{
    let io = setup_handler();

    for line in input.lines() {
        let line = line.unwrap();
        let response = io.handle_request_sync(&line).unwrap();
        write!(output, "{}", response);
    }
}

fn setup_handler() -> IoHandler {
    let mut io = IoHandler::new();

    io.add_sync_method("initialize", |params| {
        println!("{:?}", params);
        let value = match params {
            Params::Map(map) => Value::Object(map),
            Params::Array(arr) => Value::Array(arr),
            Params::None => Value::Null,
        };
        println!("{:?}", value);

        let request = from_value::<lsp_types::InitializeParams>(value);
        println!("{:?}", request);

        Ok(Value::String("Hello World!".into()))
    });

    io
}
