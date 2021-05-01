use std::io::{BufRead, Write};
use std::sync::{Arc, Mutex};

use jsonrpc_core::{IoHandler, Params};
use lsp_types::{DidOpenTextDocumentParams, InitializeParams, InitializeResult};
use serde_json::{self, from_value, to_value, Value};

use crate::server::state::ServerState;

pub fn run<R, W>(input: R, mut output: W)
where
    R: BufRead,
    W: Write,
{
    let io = setup_handler();

    for line in input.lines() {
        let line = line.unwrap();
        let response = io.handle_request_sync(&line);
        if let Some(response) = response {
            writeln!(output, "{}", response).unwrap();
        }
    }
}

fn setup_handler() -> IoHandler {
    let mut io = IoHandler::new();

    let state = Arc::new(Mutex::new(ServerState::new()));

    io.add_sync_method("initialize", |params| {
        let value = params_to_value(params);
        let request = from_value::<InitializeParams>(value).unwrap();

        let response = InitializeResult::default();

        Ok(to_value(response).unwrap())
    });

    io.add_notification("textDocument/didOpen", move |params| {
        let value = params_to_value(params);
        let notification = from_value::<DidOpenTextDocumentParams>(value).unwrap();

        state.lock().unwrap().did_open(&notification);
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
