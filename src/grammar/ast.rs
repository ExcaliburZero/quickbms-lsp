use std::collections::BTreeMap;

use lsp_types::{Location, Position, Range, Url};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct File {
    pub script: Script,
    pub keywords_by_location: Vec<(LocationRange, Keyword)>,
    //pub function_call_locations: BTreeMap<String, Vec<LocationRange>>,
    pub function_call_locations: Vec<(LocationRange, Function)>,
    pub function_definition_locations: BTreeMap<String, LocationRange>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CompilationUnit {
    CUScript(Script),
    CUPrintStatement(PrintStatement),
    CUSetStatement(SetStatement),
    CUFunctionCall(FunctionCall),
    CUFunctionDefinition(FunctionDefinition),
    CUKeyword(Keyword),
    CUFunction(Function),
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
    StmFunctionCall(FunctionCall),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FunctionDefinition {
    // TODO: track the spelling of the start and end keywords
    pub name: String,               // TODO: maybe wrap this in an ID class?
    pub statements: Vec<Statement>, // No nested function definitions allowed
    pub location: LocationRange,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FunctionCall {
    pub call_function_keyword: Keyword,
    pub function: Function, // TODO: make Function struct
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
pub struct Function {
    pub name: String,
    pub location: LocationRange,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Keyword {
    pub content: String,
    pub location: LocationRange,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Variable {
    pub name: String, // TODO: wrap this in an ID class?
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

    pub fn to_location(&self, file: &Url) -> Location {
        Location::new(file.clone(), self.to_range())
    }

    pub fn to_range(&self) -> Range {
        Range::new(
            self.start.to_position(),
            self.end.plus_columns(1).to_position(),
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LineColumn {
    pub line: isize,
    pub column: isize,
}

impl LineColumn {
    pub fn new(line: isize, column: isize) -> LineColumn {
        LineColumn { line, column }
    }

    pub fn from_position(position: &Position) -> LineColumn {
        LineColumn {
            line: position.line as isize + 1,
            column: position.character as isize,
        }
    }

    pub fn to_position(&self) -> Position {
        Position::new((self.line - 1) as u32, self.column as u32)
    }

    pub fn plus_columns(&self, num_columns: isize) -> LineColumn {
        LineColumn::new(self.line, self.column + num_columns)
    }
}
