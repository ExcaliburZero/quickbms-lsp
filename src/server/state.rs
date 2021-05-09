use std::collections::HashMap;

use lsp_types::{
    DidOpenTextDocumentParams, Hover, HoverContents, HoverParams, MarkupContent, MarkupKind, Url,
};

use crate::grammar::ast::{File, LineColumn};
use crate::grammar::parsing::parse_str;

pub struct ServerState {
    files: HashMap<Url, File>,
    keyword_docs: HashMap<String, String>,
}

impl ServerState {
    pub fn new() -> ServerState {
        ServerState {
            files: HashMap::new(),
            keyword_docs: get_keyword_docs(),
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
            eprintln!("{:?}, {:?}", loc_range, keyword);
            if loc_range.contains(&line_column) {
                let keyword_lower = keyword.content.to_lowercase();
                let keyword_docs = self.keyword_docs.get(&keyword_lower);
                return Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: keyword_docs.unwrap().to_string(),
                    }),
                    range: None,
                });
            }
        }

        eprintln!("Failed to find hover keyword");

        None
    }
}

pub fn get_keyword_docs() -> HashMap<String, String> {
    [(
        "print".to_string(),
        include_str!("keyword_docs/print.txt").to_string(),
    )]
    .iter()
    .cloned()
    .collect()
}
