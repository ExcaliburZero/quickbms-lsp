use lsp_types::Position;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct File {
    pub script: Script,
    pub keywords_by_location: Vec<(LocationRange, Keyword)>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CompilationUnit {
    CUScript(Script),
    CUPrintStatement(PrintStatement),
    CUKeyword(Keyword),
    CUExpression(Expression),
    CUStatement(Statement),
    CUStringLiteral(StringLiteral),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Script {
    pub statements: Vec<Statement>,
    pub location: LocationRange,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Statement {
    StmPrintStatement(PrintStatement),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrintStatement {
    pub print_keyword: Keyword,
    pub expression: Expression,
    pub location: LocationRange,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    ExpStringLiteral(StringLiteral),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StringLiteral {
    pub content: String,
    pub location: LocationRange,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Keyword {
    pub content: String,
    pub location: LocationRange,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocationRange {
    pub start: LineColumn,
    pub end: LineColumn,
}

impl LocationRange {
    pub fn contains(&self, line_column: &LineColumn) -> bool {
        if self.start.line == self.end.line {
            self.start.line == line_column.line
                && self.start.column <= line_column.column
                && line_column.column <= self.end.column
        } else {
            panic!()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LineColumn {
    pub line: isize,
    pub column: isize,
}

impl LineColumn {
    pub fn from_position(position: &Position) -> LineColumn {
        LineColumn {
            line: position.line as isize + 1,
            column: position.character as isize,
        }
    }
}

#[test]
fn test_get_hover_message() {
    let file = File {
        script: Script {
            statements: vec![Statement::StmPrintStatement(PrintStatement {
                print_keyword: {
                    Keyword {
                        content: "print".to_string(),
                        location: LocationRange {
                            start: LineColumn { line: 1, column: 0 },
                            end: LineColumn { line: 1, column: 4 },
                        },
                    }
                },
                expression: Expression::ExpStringLiteral(StringLiteral {
                    content: "Hello, World!".to_string(),
                    location: LocationRange {
                        start: LineColumn { line: 1, column: 6 },
                        end: LineColumn {
                            line: 1,
                            column: 20,
                        },
                    },
                }),
                location: LocationRange {
                    start: LineColumn { line: 1, column: 0 },
                    end: LineColumn {
                        line: 1,
                        column: 20,
                    },
                },
            })],
            location: LocationRange {
                start: LineColumn { line: 1, column: 0 },
                end: LineColumn {
                    line: 1,
                    column: 20,
                },
            },
        },
        keywords_by_location: vec![(
            LocationRange {
                start: LineColumn { line: 1, column: 0 },
                end: LineColumn { line: 1, column: 4 },
            },
            Keyword {
                content: "print".to_string(),
                location: LocationRange {
                    start: LineColumn { line: 1, column: 0 },
                    end: LineColumn { line: 1, column: 4 },
                },
            },
        )],
    };
}
