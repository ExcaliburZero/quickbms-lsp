#[derive(Debug)]
pub enum CompilationUnit {
    CUPrintStatement(PrintStatement),
    CUKeyword(Keyword),
    CUExpression(Expression),
    CUStringLiteral(StringLiteral),
}

#[derive(Debug)]
pub struct PrintStatement {
    pub print_keyword: Keyword,
    pub expression: Expression,
}

#[derive(Debug)]
pub enum Expression {
    ExpStringLiteral(StringLiteral),
}

#[derive(Debug)]
pub struct StringLiteral {
    pub content: String,
}

#[derive(Debug)]
pub struct Keyword {
    pub content: String,
}
