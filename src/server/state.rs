use std::collections::HashMap;

use tree_sitter::{Query, QueryCursor, Tree};

use lsp_types::{
    DidChangeTextDocumentParams, DidOpenTextDocumentParams, DocumentSymbolParams,
    DocumentSymbolResponse, FoldingRange, FoldingRangeKind, FoldingRangeParams,
    GotoDefinitionParams, GotoDefinitionResponse, Hover, HoverContents, HoverParams, Location,
    MarkupContent, MarkupKind, ReferenceParams, SymbolInformation, SymbolKind, Url,
};

use crate::grammar::parsing::{get_quickbms_language, parse, PointLike, RangeLike};

use crate::server::keyword_docs::get_keyword_docs;

pub struct ServerState {
    files: HashMap<Url, (String, Tree)>,
    keyword_docs: HashMap<String, String>,
}

impl Default for ServerState {
    fn default() -> Self {
        Self::new()
    }
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

    pub fn did_change(&mut self, request: &DidChangeTextDocumentParams) {
        let text_document = &request.text_document;

        let text = request.content_changes[0].text.clone();
        let tree = parse(&text);

        match tree {
            Some(t) => {
                self.files.insert(text_document.uri.clone(), (text, t));
            }
            None => eprintln!("Parsing failed due to timeout or cancellation flag."),
        }
    }

    pub fn document_symbol(
        &self,
        request: &DocumentSymbolParams,
    ) -> Option<DocumentSymbolResponse> {
        let url = &request.text_document.uri;

        let (source, tree) = self.files.get(url).unwrap();

        // Find function definitions
        let mut functions = vec![];
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

            let func_name = function_declaration
                .child_by_field_name("name")
                .unwrap()
                .utf8_text(source.as_bytes())
                .unwrap()
                .to_string();

            let location = function_declaration.range().to_location(url);

            functions.push((func_name, location));
        }

        // Return symbols
        let symbols: Vec<SymbolInformation> = functions
            .iter()
            .map(|(name, location)| SymbolInformation {
                name: name.clone(),
                kind: SymbolKind::Function,
                tags: None,
                deprecated: None,
                location: location.clone(),
                container_name: None,
            })
            .collect();

        Some(DocumentSymbolResponse::Flat(symbols))
    }

    pub fn hover(&self, request: &HoverParams) -> Option<Hover> {
        let text_document_position_params = &request.text_document_position_params;
        let url = &text_document_position_params.text_document.uri;
        let point = text_document_position_params.position.to_point();

        let (_source, tree) = self.files.get(url).unwrap();

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
                        function_declaration.range().to_location(url),
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
                    function_references.push(decl_name.range().to_location(url));
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
                    function_references.push(call_name.range().to_location(url));
                }
            }

            // Return the references we found
            return Some(function_references);
        }

        None
    }

    pub fn folding_range(&self, request: &FoldingRangeParams) -> Option<Vec<FoldingRange>> {
        let url = &request.text_document.uri;

        let (_, tree) = self.files.get(url).unwrap();

        let mut folding_ranges = vec![];

        // Find function definitions
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
            let location = function_declaration.range().to_location(url);

            folding_ranges.push(FoldingRange {
                start_line: location.range.start.line,
                start_character: None,
                end_line: location.range.end.line,
                end_character: None,
                kind: Some(FoldingRangeKind::Region),
            });
        }

        // Find if statements
        let query = Query::new(get_quickbms_language(), r#"(if_statement) @if_statement"#).unwrap();

        let mut query_cursor = QueryCursor::new();
        let text_callback = |node: tree_sitter::Node| format!("{:?}", node); // TODO: placeholder

        let matches = query_cursor.captures(&query, tree.root_node(), text_callback);
        for (m, _) in matches {
            let if_statement = m.captures[0].node;
            let location = if_statement.range().to_location(url);

            let if_body = if_statement.child_by_field_name("body");
            if let Some(if_body) = if_body {
                let if_body_location = if_body.range().to_location(url);

                // If statement body
                folding_ranges.push(FoldingRange {
                    start_line: location.range.start.line,
                    start_character: None,
                    end_line: if_body_location.range.end.line,
                    end_character: None,
                    kind: Some(FoldingRangeKind::Region),
                });
            }

            for else_clause in
                if_statement.children_by_field_name("else_clauses", &mut if_statement.walk())
            {
                let clause_location = else_clause.range().to_location(url);

                // Else or ElseIf statement body
                folding_ranges.push(FoldingRange {
                    start_line: clause_location.range.start.line,
                    start_character: None,
                    end_line: clause_location.range.end.line,
                    end_character: None,
                    kind: Some(FoldingRangeKind::Region),
                });
            }
        }

        // Find for loops
        let query =
            Query::new(get_quickbms_language(), r#"(for_statement) @for_statement"#).unwrap();

        let mut query_cursor = QueryCursor::new();
        let text_callback = |node: tree_sitter::Node| format!("{:?}", node); // TODO: placeholder

        let matches = query_cursor.captures(&query, tree.root_node(), text_callback);
        for (m, _) in matches {
            let for_statement = m.captures[0].node;
            let location = for_statement.range().to_location(url);

            let bottom_location = for_statement
                .children_by_field_name("body", &mut for_statement.walk())
                .map(|c| c.range().to_location(url))
                .max_by_key(|l| l.range.end.line);
            if let Some(bottom_location) = bottom_location {
                // For statement body
                folding_ranges.push(FoldingRange {
                    start_line: location.range.start.line,
                    start_character: None,
                    end_line: bottom_location.range.end.line,
                    end_character: None,
                    kind: Some(FoldingRangeKind::Region),
                });
            }
        }

        Some(folding_ranges)
    }
}
