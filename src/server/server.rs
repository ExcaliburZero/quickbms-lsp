use std::io::{BufRead, Write};

use jsonrpc_core::{IoHandler, Params};
use serde_json::{self, from_value, Value};

pub fn run<R, W>(mut input: R, output: W)
where
    R: BufRead,
    W: Write,
{
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

    let request =
        r#"{"jsonrpc": "2.0", "method": "initialize", "params": {"capabilities": {}}, "id": 1}"#;
    let response = r#"{"jsonrpc":"2.0","result":"Hello World!","id":1}"#;

    assert_eq!(io.handle_request_sync(request), Some(response.to_string()));
}
