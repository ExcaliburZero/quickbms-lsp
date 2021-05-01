use std::collections::HashMap;

use lsp_types::{DidOpenTextDocumentParams, Url};

use crate::grammar::ast::File;
use crate::grammar::parsing::parse_str;

pub struct ServerState {
    files: HashMap<Url, File>,
}

impl ServerState {
    pub fn new() -> ServerState {
        ServerState {
            files: HashMap::new(),
        }
    }

    pub fn did_open(&mut self, request: &DidOpenTextDocumentParams) {
        let text_document = &request.text_document;

        let file = parse_str(&text_document.text);

        self.files.insert(text_document.uri.clone(), file);
    }
}
