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
    CUSetStatement(SetStatement),
    CUFunctionDefinition(FunctionDefinition),
    CUKeyword(Keyword),
    CUVariable(Variable),
    CUExpression(Expression),
    CUStatement(Statement),
    CUTopStatement(TopStatement),
    CUStringLiteral(StringLiteral),
    CUIntegerLiteral(IntegerLiteral),
    CUType(Type),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Script {
    pub statements: Vec<TopStatement>,
    pub location: LocationRange,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TopStatement {
    TStmStatement(Statement),
    TStmFunctionDefinition(FunctionDefinition),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Statement {
    StmPrintStatement(PrintStatement),
    StmSetStatement(SetStatement),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FunctionDefinition {
    pub name: String,
    pub statements: Vec<Statement>, // No nested function definitions allowed
    pub location: LocationRange,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrintStatement {
    pub print_keyword: Keyword,
    pub expression: Expression,
    pub location: LocationRange,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetStatement {
    pub set_keyword: Keyword,
    pub variable: Variable,
    pub type_name: Option<Type>,
    pub value: Expression,
    pub location: LocationRange,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    ExpStringLiteral(StringLiteral),
    ExpIntegerLiteral(IntegerLiteral),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StringLiteral {
    pub content: String,
    pub location: LocationRange,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntegerLiteral {
    pub value: String,
    pub location: LocationRange,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Keyword {
    pub content: String,
    pub location: LocationRange,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Variable {
    pub name: String,
    pub location: LocationRange,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Type {
    Long,
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
