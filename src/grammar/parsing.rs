use std::borrow::{Borrow, Cow};
use std::fmt::Write;
use std::io::Read;
use std::iter::FromIterator;
use std::rc::Rc;

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::int_stream::IntStream;
use antlr_rust::lexer::Lexer;

use antlr_rust::token::{Token, TOKEN_EOF};
use antlr_rust::token_factory::{ArenaCommonFactory, OwningTokenFactory};
use antlr_rust::token_stream::{TokenStream, UnbufferedTokenStream};
use antlr_rust::tree::{
    ParseTree, ParseTreeListener, ParseTreeVisitor, ParseTreeWalker, TerminalNode, Tree,
    VisitChildren, Visitable, VisitableDyn,
};
use antlr_rust::InputStream;

use crate::grammar::ast::{
    CompilationUnit, Expression, Keyword, PrintStatement, Script, Statement, StringLiteral,
};
use crate::grammar::quickbmslexer::*;
use crate::grammar::quickbmslistener::*;
use crate::grammar::quickbmsparser::{
    quickbmsParser, quickbmsParserContext, quickbmsParserContextType, ExpressionContext,
    Print_statementContext, ScriptContext, StatementContext, String_literalContext,
};
use crate::grammar::quickbmsvisitor::quickbmsVisitor;

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
        match node.symbol.get_token_type() {
            PRINT => {
                if let Cow::Borrowed(s) = node.symbol.text {
                    self.return_stack.push(CompilationUnit::CUKeyword(Keyword {
                        content: s.to_string(),
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
        let mut statements = vec![];
        for child in ctx.get_children() {
            child.as_ref().accept_dyn(self);
            let statement = match self.return_stack.pop().unwrap() {
                CompilationUnit::CUStatement(statement) => statement,
                _ => panic!(),
            };

            statements.push(statement);
        }

        let script = Script { statements };

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
        ctx.get_child(0).unwrap().as_ref().accept_dyn(self);
        let print_keyword = match self.return_stack.pop().unwrap() {
            CompilationUnit::CUKeyword(keyword) => keyword,
            _ => panic!(),
        };
        println!("{:?}", print_keyword);

        ctx.get_child(1).unwrap().as_ref().accept_dyn(self);
        let expression = match self.return_stack.pop().unwrap() {
            CompilationUnit::CUExpression(expression) => expression,
            _ => panic!(),
        };
        println!("{:?}", expression);

        let value = PrintStatement {
            print_keyword,
            expression,
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
                        }
                    },
                    expression: Expression::ExpStringLiteral(StringLiteral {
                        content: "Hello, World!".to_string(),
                    })
                })]
            })
        );

        result
    }
    let tf = ArenaCommonFactory::default();

    let _result = parse(&tf);
}
