pub trait CompilationUnit {}

pub struct PrintStatement {
    print_string: String,
    expression: dyn Expression,
}
impl CompilationUnit for PrintStatement {}

pub trait Expression {}
impl CompilationUnit for dyn Expression {}
