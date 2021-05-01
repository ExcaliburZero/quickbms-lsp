use std::io::{BufRead, Write};

use jsonrpc_core::{IoHandler, Params};
use lsp_types::{InitializeParams, InitializeResult};
use serde_json::{self, from_value, to_value, Value};

pub fn run<R, W>(input: R, mut output: W)
where
    R: BufRead,
    W: Write,
{
    let io = setup_handler();

    for line in input.lines() {
        let line = line.unwrap();
        let response = io.handle_request_sync(&line).unwrap();
        write!(output, "{}", response).unwrap();
    }
}

fn setup_handler() -> IoHandler {
    let mut io = IoHandler::new();

    io.add_sync_method("initialize", |params| {
        let value = params_to_value(params);
        let request = from_value::<InitializeParams>(value);

        let response = InitializeResult::default();

        Ok(to_value(response).unwrap())
    });

    io
}

fn params_to_value(params: Params) -> Value {
    match params {
        Params::Map(map) => Value::Object(map),
        Params::Array(arr) => Value::Array(arr),
        Params::None => Value::Null,
    }
}
