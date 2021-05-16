use lsp_types::{Location, Position, Url};
use tree_sitter::{Language, Parser, Point, Query, QueryCursor, Range, Tree};

extern "C" {
    fn tree_sitter_quickbms() -> Language;
}

pub fn get_quickbms_language() -> Language {
    unsafe { tree_sitter_quickbms() }
}

pub fn parse(text: &str) -> Option<Tree> {
    let mut parser = Parser::new();

    let language = get_quickbms_language();
    parser.set_language(language).unwrap();

    parser.parse(text, None)
}

pub trait PointLike {
    fn to_point(&self) -> Point;
    fn to_position(&self) -> Position;
}

impl PointLike for Point {
    fn to_point(&self) -> Point {
        *self
    }

    fn to_position(&self) -> Position {
        Position {
            line: self.row as u32,
            character: self.column as u32,
        }
    }
}

impl PointLike for Position {
    fn to_point(&self) -> Point {
        Point {
            row: self.line as usize,
            column: self.character as usize,
        }
    }

    fn to_position(&self) -> Position {
        *self
    }
}

pub trait RangeLike {
    fn to_location(&self, url: &Url) -> Location;
    fn to_lsp_range(&self) -> lsp_types::Range;
}

impl RangeLike for Range {
    fn to_location(&self, url: &Url) -> Location {
        Location {
            uri: url.clone(),
            range: self.to_lsp_range(),
        }
    }

    fn to_lsp_range(&self) -> lsp_types::Range {
        lsp_types::Range::new(self.start_point.to_position(), self.end_point.to_position())
    }
}

#[test]
fn test_parsing() {
    let tree = parse("set myVar 2");

    assert!(tree.is_some());
}
