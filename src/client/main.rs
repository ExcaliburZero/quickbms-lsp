use jsonrpc_core::types::id::Id;
use jsonrpc_core::types::params::Params;
use jsonrpc_core::types::request::{Call, MethodCall, Request};
use lsp_types::{ClientCapabilities, InitializeParams};
use serde_json::{self, to_string, to_value, Value};

fn main() {
    let requests = vec![(
        "initialize",
        InitializeParams {
            capabilities: ClientCapabilities {
                experimental: None,
                general: None,
                text_document: None,
                window: None,
                workspace: None,
            },
            client_info: None,
            initialization_options: None,
            locale: None,
            process_id: None,
            root_path: None,
            root_uri: None,
            trace: None,
            workspace_folders: None,
        },
    )];

    for (method, request) in requests {
        let value = to_value(request).unwrap();
        let params = match value {
            Value::Object(map) => Params::Map(map),
            Value::Array(arr) => Params::Array(arr),
            Value::Null => Params::None,
            _ => panic!(),
        };

        println!("{:?}", params);

        println!(
            "{}",
            to_string(&Request::Single(Call::MethodCall(MethodCall {
                jsonrpc: Some(jsonrpc_core::Version::V2),
                method: method.to_string(),
                id: Id::Null,
                params,
            })))
            .unwrap()
        );
    }
}
