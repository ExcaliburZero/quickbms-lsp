use std::io::{BufRead, Write};

use jsonrpc_core::{IoHandler, Value};

pub fn run<R, W>(mut input: R, output: W)
where
    R: BufRead,
    W: Write,
{
    let mut io = IoHandler::new();

    io.add_sync_method("hello", |_| Ok(Value::String("Hello World!".into())));

    let request = r#"{"jsonrpc": "2.0", "method": "hello", "params": [42, 23], "id": 1}"#;
    let response = r#"{"jsonrpc":"2.0","result":"Hello World!","id":1}"#;

    assert_eq!(io.handle_request_sync(request), Some(response.to_string()));
}
