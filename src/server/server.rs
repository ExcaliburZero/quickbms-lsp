use std::fmt;
use std::io::{BufRead, Write};
use std::str::from_utf8;
use std::sync::{Arc, Mutex};

use jsonrpc_core::{IoHandler, Params};
use lsp_types::{
    DidChangeTextDocumentParams, DidOpenTextDocumentParams, DocumentSymbolParams,
    FoldingProviderOptions, FoldingRangeParams, FoldingRangeProviderCapability,
    GotoDefinitionParams, GotoDefinitionResponse, HoverParams, HoverProviderCapability,
    InitializeParams, InitializeResult, ReferenceParams, ServerCapabilities,
    TextDocumentSyncCapability, TextDocumentSyncKind,
};
use regex::Regex;
use serde_json::{self, from_value, to_value, Value};

use crate::server::state::ServerState;

const EXIT_STRING: &str = "quickbms-lsp EXIT (-.-) zzz";

pub fn run<R, W>(mut input: R, mut output: W) -> i32
where
    R: BufRead,
    W: Write,
{
    let io = setup_handler();

    eprintln!("Started server");

    loop {
        let message = Message::read_from_buffered_reader(&mut input);

        let response = io.handle_request_sync(&message.content);
        if let Some(response) = response {
            // TODO: Do this by using state instead
            if response.contains(EXIT_STRING) {
                return 0;
            }

            let response_message = Message::from_content(&response);
            write!(output, "{}", response_message).unwrap();
            output.flush().unwrap();
        }
    }
}

fn setup_handler() -> IoHandler {
    let mut io = IoHandler::new();

    let state = Arc::new(Mutex::new(ServerState::new()));

    io.add_sync_method("initialize", |params| {
        let value = params_to_value(params);
        let _request = from_value::<InitializeParams>(value).unwrap();

        let mut response = InitializeResult::default();
        response.capabilities.hover_provider = Some(HoverProviderCapability::Simple(true));

        let response = InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::Full,
                )),
                hover_provider: Some(true.into()),
                definition_provider: Some(lsp_types::OneOf::Left(true)),
                references_provider: Some(lsp_types::OneOf::Left(true)),
                document_symbol_provider: Some(lsp_types::OneOf::Left(true)),
                folding_range_provider: Some(FoldingRangeProviderCapability::FoldingProvider(
                    FoldingProviderOptions {},
                )),
                ..ServerCapabilities::default()
            },
            ..InitializeResult::default()
        };

        Ok(to_value(response).unwrap())
    });

    let state_c = state.clone();
    io.add_notification("textDocument/didOpen", move |params| {
        let value = params_to_value(params);
        let notification = from_value::<DidOpenTextDocumentParams>(value).unwrap();

        state_c.lock().unwrap().did_open(&notification);
    });

    let state_c = state.clone();
    io.add_notification("textDocument/didChange", move |params| {
        let value = params_to_value(params);
        let notification = from_value::<DidChangeTextDocumentParams>(value).unwrap();

        state_c.lock().unwrap().did_change(&notification);
    });

    let state_c = state.clone();
    io.add_sync_method("textDocument/documentSymbol", move |params| {
        let value = params_to_value(params);
        let request = from_value::<DocumentSymbolParams>(value).unwrap();

        let response = state_c.lock().unwrap().document_symbol(&request);

        match response {
            None => Ok(Value::Null),
            Some(response) => Ok(to_value(response).unwrap()),
        }
    });

    let state_c = state.clone();
    io.add_sync_method("textDocument/hover", move |params| {
        let value = params_to_value(params);
        let request = from_value::<HoverParams>(value).unwrap();

        let response = state_c.lock().unwrap().hover(&request);

        match response {
            None => Ok(Value::Null),
            Some(response) => Ok(to_value(response).unwrap()),
        }
    });

    let state_c = state.clone();
    io.add_sync_method("textDocument/definition", move |params| {
        let value = params_to_value(params);
        let request = from_value::<GotoDefinitionParams>(value).unwrap();

        let response = match state_c.lock().unwrap().goto_definition(&request) {
            Some(resp) => resp,
            None => GotoDefinitionResponse::Array(vec![]),
        };

        Ok(to_value(response).unwrap())
    });

    let state_c = state.clone();
    io.add_sync_method("textDocument/references", move |params| {
        let value = params_to_value(params);
        let request = from_value::<ReferenceParams>(value).unwrap();

        let response = state_c.lock().unwrap().goto_references(&request);

        Ok(to_value(response).unwrap())
    });

    let state_c = state.clone();
    io.add_sync_method("textDocument/foldingRange", move |params| {
        let value = params_to_value(params);
        let request = from_value::<FoldingRangeParams>(value).unwrap();

        let response = state_c.lock().unwrap().folding_range(&request);

        match response {
            None => Ok(Value::Null),
            Some(response) => Ok(to_value(response).unwrap()),
        }
    });

    // let state_c = state.clone();
    io.add_sync_method("shutdown", move |_| {
        eprintln!("Shutting down...");
        Ok(Value::Null)
    });

    // let state_c = state.clone();
    io.add_sync_method("exit", move |_| {
        eprintln!("Exiting...");
        Ok(Value::String(EXIT_STRING.to_string()))
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

#[derive(Debug, Eq, PartialEq)]
struct Header {
    content_length: u64,
    content_type: Option<String>,
}

impl Header {
    fn read_from_buffered_reader<R: BufRead>(input: &mut R) -> Header {
        let mut content_length_line = String::new();
        input.read_line(&mut content_length_line).unwrap();

        let mut content_type_line = String::new();
        input.read_line(&mut content_type_line).unwrap();

        let content_length = Header::parse_content_length(&content_length_line);
        let content_type = Header::parse_content_type(&content_type_line);

        Header {
            content_length,
            content_type,
        }
    }

    fn parse_content_length(line: &str) -> u64 {
        let re = Regex::new(r"^Content-Length: (\d+)").unwrap();
        let length_string = re.captures(line).unwrap()[1].to_string();

        length_string.parse::<u64>().unwrap()
    }

    fn parse_content_type(line: &str) -> Option<String> {
        if line == "\r\n" {
            None
        } else {
            Some(line.to_string())
        }
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Content-Length: {}\r\n\r\n", self.content_length)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Message {
    header: Header,
    content: String,
}

impl Message {
    fn read_from_buffered_reader<R: BufRead>(input: &mut R) -> Message {
        let header = Header::read_from_buffered_reader(input);

        let mut content_buf: Vec<u8> = vec![0; header.content_length as usize];
        input.read_exact(&mut content_buf).unwrap();

        let content = from_utf8(&content_buf).unwrap().to_string();

        Message { header, content }
    }

    fn from_content(content: &str) -> Message {
        let content_length = content.as_bytes().len() as u64;

        Message {
            header: Header {
                content_length,
                content_type: None,
            },
            content: content.to_string(),
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.header, self.content)
    }
}

#[test]
fn test_message_from_buffered_reader() {
    use std::io::BufReader;

    let text =
        "Content-Length: 52\r\n\r\n{\"jsonrpc\":\"2.0\",\"method\":\"initialized\",\"params\":{}}";

    let mut reader = BufReader::new(text.as_bytes());

    let actual = Message::read_from_buffered_reader(&mut reader);
    let expected = Message {
        header: Header {
            content_length: 52,
            content_type: None,
        },
        content: r#"{"jsonrpc":"2.0","method":"initialized","params":{}}"#.to_string(),
    };

    assert_eq!(expected, actual);
}
