pub trait CompilationUnit {}

pub struct PrintStatement {
    pub print_string: String,
    pub expression: dyn Expression,
}
impl CompilationUnit for PrintStatement {}

pub trait Expression {}
impl CompilationUnit for dyn Expression {}
