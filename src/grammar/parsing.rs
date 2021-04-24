use std::borrow::{Borrow, Cow};
use std::fmt::Debug;
use std::fmt::Write;
use std::io::Read;
use std::iter::FromIterator;
use std::ops::Deref;
use std::rc::Rc;

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::int_stream::IntStream;
use antlr_rust::lexer::Lexer;

use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext};
use antlr_rust::rule_context::CustomRuleContext;
use antlr_rust::token::GenericToken;
use antlr_rust::token::{Token, TOKEN_EOF};
use antlr_rust::token_factory::{ArenaCommonFactory, OwningTokenFactory};
use antlr_rust::token_stream::{TokenStream, UnbufferedTokenStream};
use antlr_rust::tree::{
    ParseTree, ParseTreeListener, ParseTreeVisitor, ParseTreeWalker, TerminalNode, Tree,
    VisitChildren, Visitable, VisitableDyn,
};
use antlr_rust::InputStream;
use antlr_rust::TidAble;

use crate::grammar::ast::{
    CompilationUnit, Expression, Keyword, LineColumn, LocationRange, PrintStatement, Script,
    Statement, StringLiteral,
};
use crate::grammar::quickbmslexer::*;
use crate::grammar::quickbmslistener::*;
use crate::grammar::quickbmsparser::{
    quickbmsParser, quickbmsParserContext, quickbmsParserContextType, ExpressionContext,
    Print_statementContext, ScriptContext, StatementContext, String_literalContext,
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
}

impl QuickBMSVisitorImpl {
    pub fn get_result(&self) -> Option<Script> {
        if self.return_stack.len() > 0 {
            match self.return_stack.last().unwrap() {
                CompilationUnit::CUScript(script) => Some(script.clone()),
                _ => panic!(),
            }
        } else {
            panic!()
        }
    }
}

impl<'i> ParseTreeVisitor<'i, quickbmsParserContextType> for QuickBMSVisitorImpl {
    fn visit_terminal(&mut self, node: &TerminalNode<'i, quickbmsParserContextType>) {
        let location = token_location_range![node.symbol];
        match node.symbol.get_token_type() {
            PRINT => {
                if let Cow::Borrowed(s) = node.symbol.text {
                    self.return_stack.push(CompilationUnit::CUKeyword(Keyword {
                        content: s.to_string(),
                        location,
                    }));
                } else {
                    panic!();
                }
            }
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
                CompilationUnit::CUStatement(statement) => statement,
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
        };
        result.accept(&mut visitor);

        assert_eq!(
            visitor.get_result(),
            Some(Script {
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
                })],
                location: LocationRange {
                    start: LineColumn { line: 1, column: 0 },
                    end: LineColumn {
                        line: 1,
                        column: 20
                    },
                }
            })
        );

        result
    }
    let tf = ArenaCommonFactory::default();

    let _result = parse(&tf);
}
