use std::borrow::Cow;
use std::ops::Deref;
use std::rc::Rc;

use antlr_rust::common_token_stream::CommonTokenStream;

use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::Token;
use antlr_rust::token_factory::ArenaCommonFactory;
use antlr_rust::tree::{ParseTree, ParseTreeVisitor, TerminalNode, Tree, Visitable, VisitableDyn};
use antlr_rust::InputStream;

use crate::grammar::ast::{
    CompilationUnit, Expression, File, FunctionCall, FunctionDefinition, IntegerLiteral, Keyword,
    LineColumn, LocationRange, PrintStatement, Script, SetStatement, Statement, StringLiteral,
    TopStatement, Type, Variable,
};
use crate::grammar::quickbmslexer::*;
use crate::grammar::quickbmsparser::{
    quickbmsParser, quickbmsParserContextType, Function_call_statementContext,
    Function_definitionContext, Integer_literalContext, Print_statementContext, ScriptContext,
    Set_statementContext, String_literalContext, VariableContext,
};
use crate::grammar::quickbmsvisitor::quickbmsVisitor;

// Some macros to get the locations of tokens. It might be possible to define these as functions,
// but after trying for quite a while I wasn't able to get the trait bounds right, so I just made
// them macros instead.
#[macro_export]
macro_rules! ctx_location_range {
    ($ctx:expr) => {{
        let start = token_location_range![$ctx.start().deref()].start;
        let end = token_location_range![$ctx.stop().deref()].end;

        LocationRange { start, end }
    }};
}

#[macro_export]
macro_rules! token_location_range {
    ($token:expr) => {{
        let start = LineColumn {
            line: $token.line,
            column: $token.column,
        };

        // TODO: How does this work with multi-line tokens?
        let end = LineColumn {
            line: $token.line,
            column: $token.column + ($token.stop - $token.start),
        };

        LocationRange { start, end }
    }};
}

struct QuickBMSVisitorImpl {
    return_stack: Vec<CompilationUnit>,
    keywords_by_location: Vec<(LocationRange, Keyword)>,
}

impl QuickBMSVisitorImpl {
    pub fn get_result(&self) -> Option<File> {
        if self.return_stack.len() > 0 {
            match self.return_stack.last().unwrap() {
                CompilationUnit::CUScript(script) => Some(File {
                    script: script.clone(),
                    keywords_by_location: self.keywords_by_location.clone(),
                }),
                _ => panic!(),
            }
        } else {
            panic!()
        }
    }

    fn keyword<'i>(
        &mut self,
        node: &TerminalNode<'i, quickbmsParserContextType>,
        location: LocationRange,
    ) {
        if let Cow::Borrowed(s) = node.symbol.text {
            let keyword = Keyword {
                content: s.to_string(),
                location,
            };
            self.return_stack
                .push(CompilationUnit::CUKeyword(keyword.clone()));

            self.keywords_by_location
                .push((keyword.location.clone(), keyword));
        } else {
            panic!();
        }
    }
}

impl<'i> ParseTreeVisitor<'i, quickbmsParserContextType> for QuickBMSVisitorImpl {
    fn visit_terminal(&mut self, node: &TerminalNode<'i, quickbmsParserContextType>) {
        let location = token_location_range![node.symbol];
        match node.symbol.get_token_type() {
            PRINT => self.keyword(node, location),
            SET => self.keyword(node, location),
            START_FUNCTION => self.keyword(node, location),
            END_FUNCTION => self.keyword(node, location),
            CALL_FUNCTION => self.keyword(node, location),
            LONG => self.return_stack.push(CompilationUnit::CUType(Type::Long)),
            STRING_LITERAL => {
                if let Cow::Borrowed(s) = node.symbol.text {
                    self.return_stack
                        .push(CompilationUnit::CUStringLiteral(StringLiteral {
                            content: s[1..(s.len() - 1)].to_string(),
                            location,
                        }));
                } else {
                    panic!();
                }
            }
            _ => {}
        }
    }
}

impl<'i> quickbmsVisitor<'i> for QuickBMSVisitorImpl {
    fn visit_script(&mut self, ctx: &ScriptContext<'i>) {
        let location = ctx_location_range![ctx];

        let mut statements = vec![];
        for child in ctx.get_children() {
            child.as_ref().accept_dyn(self);
            let statement = match self.return_stack.pop().unwrap() {
                CompilationUnit::CUStatement(statement) => TopStatement::TStmStatement(statement),
                CompilationUnit::CUFunctionDefinition(function_definition) => {
                    TopStatement::TStmFunctionDefinition(function_definition)
                }
                _ => panic!(),
            };

            statements.push(statement);
        }

        let script = Script {
            statements,
            location,
        };

        self.return_stack.push(CompilationUnit::CUScript(script));
    }

    fn visit_string_literal(&mut self, ctx: &String_literalContext<'i>) {
        ctx.get_child(0).unwrap().as_ref().accept_dyn(self);
        let string_literal = match self.return_stack.pop().unwrap() {
            CompilationUnit::CUStringLiteral(string_literal) => string_literal,
            _ => panic!(),
        };

        self.return_stack
            .push(CompilationUnit::CUExpression(Expression::ExpStringLiteral(
                string_literal,
            )));
    }

    fn visit_integer_literal(&mut self, ctx: &Integer_literalContext<'i>) {
        let location = ctx_location_range![ctx];

        self.return_stack.push(CompilationUnit::CUExpression(
            Expression::ExpIntegerLiteral(IntegerLiteral {
                value: ctx.get_text(),
                location,
            }),
        ));
    }

    fn visit_function_call_statement(&mut self, ctx: &Function_call_statementContext<'i>) {
        let location = ctx_location_range![ctx];

        ctx.get_child(0).unwrap().as_ref().accept_dyn(self);
        let call_function_keyword = match self.return_stack.pop().unwrap() {
            CompilationUnit::CUKeyword(keyword) => keyword,
            _ => panic!(),
        };

        let function = ctx.get_child(1).unwrap().get_text();

        let value = FunctionCall {
            call_function_keyword,
            function,
            location,
        };
        self.return_stack
            .push(CompilationUnit::CUStatement(Statement::StmFunctionCall(
                value,
            )));
    }

    fn visit_print_statement(&mut self, ctx: &Print_statementContext<'i>) {
        let location = ctx_location_range![ctx];

        ctx.get_child(0).unwrap().as_ref().accept_dyn(self);
        let print_keyword = match self.return_stack.pop().unwrap() {
            CompilationUnit::CUKeyword(keyword) => keyword,
            _ => panic!(),
        };

        ctx.get_child(1).unwrap().as_ref().accept_dyn(self);
        let expression = match self.return_stack.pop().unwrap() {
            CompilationUnit::CUExpression(expression) => expression,
            _ => panic!(),
        };

        let value = PrintStatement {
            print_keyword,
            expression,
            location,
        };
        self.return_stack
            .push(CompilationUnit::CUStatement(Statement::StmPrintStatement(
                value,
            )));
    }

    fn visit_set_statement(&mut self, ctx: &Set_statementContext<'i>) {
        let location = ctx_location_range![ctx];

        ctx.get_child(0).unwrap().as_ref().accept_dyn(self);
        let set_keyword = match self.return_stack.pop().unwrap() {
            CompilationUnit::CUKeyword(keyword) => keyword,
            _ => panic!(),
        };

        ctx.get_child(1).unwrap().as_ref().accept_dyn(self);
        let variable = match self.return_stack.pop().unwrap() {
            CompilationUnit::CUVariable(variable) => variable,
            _ => panic!(),
        };

        ctx.get_child(2).unwrap().as_ref().accept_dyn(self);
        let mut type_name = None;
        let expression = match self.return_stack.pop().unwrap() {
            CompilationUnit::CUExpression(expression) => expression,
            CompilationUnit::CUType(t) => {
                type_name = Some(t);

                ctx.get_child(3).unwrap().as_ref().accept_dyn(self);
                match self.return_stack.pop().unwrap() {
                    CompilationUnit::CUExpression(expression) => expression,
                    _ => panic!(),
                }
            }
            _ => panic!(),
        };

        let value = SetStatement {
            set_keyword,
            type_name,
            variable,
            value: expression,
            location,
        };
        self.return_stack
            .push(CompilationUnit::CUStatement(Statement::StmSetStatement(
                value,
            )));
    }

    fn visit_variable(&mut self, ctx: &VariableContext<'i>) {
        let location = ctx_location_range![ctx];

        let name = ctx.get_text();

        let value = Variable { name, location };
        self.return_stack.push(CompilationUnit::CUVariable(value));
    }

    fn visit_function_definition(&mut self, ctx: &Function_definitionContext<'i>) {
        let location = ctx_location_range![ctx];
        let children: Vec<_> = ctx.get_children().collect();

        // StartFunction
        children[0].as_ref().accept_dyn(self);
        let start_keyword = match self.return_stack.pop().unwrap() {
            CompilationUnit::CUKeyword(keyword) => keyword,
            _ => panic!(),
        };

        // Function name
        let name_token = &children[1];
        let name = name_token.get_text();

        // Function statements
        let mut statements = vec![];
        for i in 2..(children.len() - 1) {
            let child = &children[i];
            child.as_ref().accept_dyn(self);
            let statement = match self.return_stack.pop().unwrap() {
                CompilationUnit::CUStatement(statement) => statement,
                _ => panic!(),
            };

            statements.push(statement);
        }

        // EndFunction
        children.last().unwrap().as_ref().accept_dyn(self);
        let end_keyword = match self.return_stack.pop().unwrap() {
            CompilationUnit::CUKeyword(keyword) => keyword,
            _ => panic!(),
        };

        let value = FunctionDefinition {
            name,
            statements,
            location,
        };

        self.return_stack
            .push(CompilationUnit::CUFunctionDefinition(value));
    }
}

pub fn parse_str(contents: &str) -> File {
    let tf = ArenaCommonFactory::default();

    let mut _lexer = quickbmsLexer::new_with_token_factory(InputStream::new(contents), &tf);
    let token_source = CommonTokenStream::new(_lexer);
    let mut parser = quickbmsParser::new(token_source);
    let result = parser.script().expect("parsed unsuccessfully");

    let mut visitor = QuickBMSVisitorImpl {
        return_stack: vec![],
        keywords_by_location: vec![],
    };
    result.accept(&mut visitor);

    visitor.get_result().unwrap()
}

#[test]
fn test_visitor() {
    fn parse<'a>(tf: &'a ArenaCommonFactory<'a>) -> Rc<ScriptContext<'a>> {
        let mut _lexer = quickbmsLexer::new_with_token_factory(
            InputStream::new("print \"Hello, World!\"\n"),
            tf,
        );
        let token_source = CommonTokenStream::new(_lexer);
        let mut parser = quickbmsParser::new(token_source);
        let result = parser.script().expect("parsed unsuccessfully");

        let mut visitor = QuickBMSVisitorImpl {
            return_stack: vec![],
            keywords_by_location: vec![],
        };
        result.accept(&mut visitor);

        if let Some(file) = visitor.get_result() {
            assert_eq!(
                file.script,
                Script {
                    statements: vec![TopStatement::TStmStatement(Statement::StmPrintStatement(
                        PrintStatement {
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
                                        column: 20
                                    },
                                },
                            }),
                            location: LocationRange {
                                start: LineColumn { line: 1, column: 0 },
                                end: LineColumn {
                                    line: 1,
                                    column: 20
                                },
                            }
                        }
                    ))],
                    location: LocationRange {
                        start: LineColumn { line: 1, column: 0 },
                        end: LineColumn {
                            line: 1,
                            column: 20
                        },
                    }
                }
            );

            assert_eq!(
                file.keywords_by_location,
                vec![(
                    LocationRange {
                        start: LineColumn { line: 1, column: 0 },
                        end: LineColumn { line: 1, column: 4 }
                    },
                    Keyword {
                        content: "print".to_string(),
                        location: LocationRange {
                            start: LineColumn { line: 1, column: 0 },
                            end: LineColumn { line: 1, column: 4 }
                        }
                    }
                )]
            );
        } else {
            assert!(false);
        }

        result
    }
    let tf = ArenaCommonFactory::default();

    let _result = parse(&tf);
}
