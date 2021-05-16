use std::collections::HashMap;

use tree_sitter::{Language, Parser, Query, QueryCursor, Tree};

use lsp_types::{
    DidOpenTextDocumentParams, GotoDefinitionParams, GotoDefinitionResponse, Hover, HoverContents,
    HoverParams, Location, MarkupContent, MarkupKind, ReferenceParams, Url,
};

use crate::grammar::parsing::{get_quickbms_language, parse, PointLike, RangeLike};

pub struct ServerState {
    files: HashMap<Url, (String, Tree)>,
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

        let tree = parse(&text_document.text);

        match tree {
            Some(t) => {
                self.files
                    .insert(text_document.uri.clone(), (text_document.text.clone(), t));
            }
            None => eprintln!("Parsing failed due to timeout or cancellation flag."),
        }
    }

    pub fn hover(&self, request: &HoverParams) -> Option<Hover> {
        let text_document_position_params = &request.text_document_position_params;
        let url = &text_document_position_params.text_document.uri;
        let point = text_document_position_params.position.to_point();

        let (source, tree) = self.files.get(url).unwrap();

        let node = tree
            .root_node()
            .named_descendant_for_point_range(point, point)
            .unwrap();

        // Handle if the user is pointing at a keyword
        if let Some(docs) = self.keyword_docs.get(node.kind()) {
            return Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::PlainText,
                    value: docs.to_string(),
                }),
                range: None,
            });
        }

        None
    }

    pub fn goto_definition(
        &self,
        request: &GotoDefinitionParams,
    ) -> Option<GotoDefinitionResponse> {
        let text_document_position_params = &request.text_document_position_params;
        let url = &text_document_position_params.text_document.uri;
        let point = text_document_position_params.position.to_point();

        let (source, tree) = self.files.get(url).unwrap();

        let node = tree
            .root_node()
            .named_descendant_for_point_range(point, point)
            .unwrap();

        let parent = node.parent().unwrap();

        // Handle if the user is pointing at a function call
        if parent.kind() == "function_call_statement" {
            let function_name_lc = node.utf8_text(source.as_bytes()).unwrap().to_lowercase();

            let query = Query::new(
                get_quickbms_language(),
                r#"(function_declaration) @declaration"#,
            )
            .unwrap();

            let mut query_cursor = QueryCursor::new();
            let text_callback = |node: tree_sitter::Node| format!("{:?}", node); // TODO: placeholder

            let matches = query_cursor.captures(&query, tree.root_node(), text_callback);
            for (m, _) in matches {
                let function_declaration = m.captures[0].node;

                let decl_func_name_lc = function_declaration
                    .child_by_field_name("name")
                    .unwrap()
                    .utf8_text(source.as_bytes())
                    .unwrap()
                    .to_lowercase();
                if decl_func_name_lc == function_name_lc {
                    return Some(GotoDefinitionResponse::Scalar(
                        function_declaration.range().to_location(&url),
                    ));
                }
            }
        }

        None
    }

    pub fn goto_references(&self, request: &ReferenceParams) -> Option<Vec<Location>> {
        let text_document_position = &request.text_document_position;
        let url = &text_document_position.text_document.uri;
        let point = text_document_position.position.to_point();

        let (source, tree) = self.files.get(url).unwrap();

        let node = tree
            .root_node()
            .named_descendant_for_point_range(point, point)
            .unwrap();

        let parent = node.parent().unwrap();

        // Handle if the user is pointing at a mention of a function from either a call or definition
        if parent.kind() == "function_call_statement" || parent.kind() == "function_declaration" {
            let function_name_lc = node.utf8_text(source.as_bytes()).unwrap().to_lowercase();
            let mut function_references = vec![];

            // Find the definition
            let query = Query::new(
                get_quickbms_language(),
                r#"(function_declaration) @declaration"#,
            )
            .unwrap();

            let mut query_cursor = QueryCursor::new();
            let text_callback = |node: tree_sitter::Node| format!("{:?}", node); // TODO: placeholder

            let matches = query_cursor.captures(&query, tree.root_node(), text_callback);
            for (m, _) in matches {
                let function_declaration = m.captures[0].node;

                let decl_name = function_declaration.child_by_field_name("name").unwrap();
                let decl_func_name_lc = decl_name
                    .utf8_text(source.as_bytes())
                    .unwrap()
                    .to_lowercase();
                if decl_func_name_lc == function_name_lc {
                    function_references.push(decl_name.range().to_location(&url));
                }
            }

            // Find the calls
            let query = Query::new(
                get_quickbms_language(),
                r#"(function_call_statement) @call"#,
            )
            .unwrap();

            let mut query_cursor = QueryCursor::new();
            let text_callback = |node: tree_sitter::Node| format!("{:?}", node); // TODO: placeholder

            let matches = query_cursor.captures(&query, tree.root_node(), text_callback);
            for (m, _) in matches {
                let function_call = m.captures[0].node;

                let call_name = function_call.child_by_field_name("name").unwrap();
                let call_func_name_lc = call_name
                    .utf8_text(source.as_bytes())
                    .unwrap()
                    .to_lowercase();
                if call_func_name_lc == function_name_lc {
                    function_references.push(call_name.range().to_location(&url));
                }
            }

            // Return the references we found
            return Some(function_references);
        }

        None
    }
}

pub fn get_keyword_docs() -> HashMap<String, String> {
    [
        (
            "print".to_string(),
            include_str!("keyword_docs/print.txt").to_string(),
        ),
        (
            "set".to_string(),
            include_str!("keyword_docs/set.txt").to_string(),
        ),
        (
            "startfunction".to_string(),
            include_str!("keyword_docs/functions.txt").to_string(),
        ),
        (
            "endfunction".to_string(),
            include_str!("keyword_docs/functions.txt").to_string(),
        ),
        (
            "callfunction".to_string(),
            include_str!("keyword_docs/functions.txt").to_string(),
        ),
    ]
    .iter()
    .cloned()
    .collect()
}
