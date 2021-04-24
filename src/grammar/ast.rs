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
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Statement {
    StmPrintStatement(PrintStatement),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrintStatement {
    pub print_keyword: Keyword,
    pub expression: Expression,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LineColumn {
    pub line: isize,
    pub column: isize,
}
