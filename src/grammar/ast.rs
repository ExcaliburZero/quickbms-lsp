struct PrintStatement {
    print_string: String,
    expression: dyn Expression,
}

trait Expression {}
