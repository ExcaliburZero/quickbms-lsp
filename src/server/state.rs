use std::collections::HashMap;

use lsp_types::{
    DidOpenTextDocumentParams, Hover, HoverContents, HoverParams, MarkupContent, MarkupKind, Url,
};

use crate::grammar::ast::{File, LineColumn};
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

    pub fn hover(&self, request: &HoverParams) -> Option<Hover> {
        let text_document_position_params = &request.text_document_position_params;

        let url = &text_document_position_params.text_document.uri;

        let file = self.files.get(url).unwrap();

        // Handle if the user is pointing at a keyword
        let line_column = LineColumn::from_position(&text_document_position_params.position);
        for (loc_range, keyword) in file.keywords_by_location.iter() {
            if loc_range.contains(&line_column) {
                return Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: keyword.content.clone(),
                    }),
                    range: None,
                });
            }
        }

        None
    }
}
