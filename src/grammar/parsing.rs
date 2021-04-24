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

use crate::grammar::ast::{CompilationUnit, Expression, Keyword, PrintStatement, StringLiteral};
use crate::grammar::quickbmslexer::*;
use crate::grammar::quickbmslistener::*;
use crate::grammar::quickbmsparser::{
    quickbmsParser, quickbmsParserContext, quickbmsParserContextType, ExpressionContext,
    Print_statementContext, ScriptContext, StatementContext,
};
use crate::grammar::quickbmsvisitor::quickbmsVisitor;

struct QuickBMSVisitorImpl {
    return_stack: Vec<CompilationUnit>,
}

impl<'i> ParseTreeVisitor<'i, quickbmsParserContextType> for QuickBMSVisitorImpl {
    fn visit_terminal(&mut self, node: &TerminalNode<'i, quickbmsParserContextType>) {
        /*if node.symbol.get_token_type() == PRINT {
            if let Cow::Borrowed(s) = node.symbol.text {
                self.return_stack.push(CompilationUnit::CUKeyword(Keyword {
                    content: s.to_string(),
                }));
            }
        }*/
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
                            content: s.to_string(),
                        }));
                } else {
                    panic!();
                }
            }
            _ => {}
        }
        /*if node.symbol.get_token_type() == csvparser::TEXT {
            if let Cow::Borrowed(s) = node.symbol.text {
                self.0.push(s);
            }
        }*/

        //panic!()
    }
}

impl<'i> quickbmsVisitor<'i> for QuickBMSVisitorImpl {
    /*fn visit_script(&mut self, ctx: &ScriptContext<'i>) {
        self.visit_children(ctx)
    }

    fn visit_statement(&mut self, ctx: &StatementContext<'i>) {
        self.visit_children(ctx)
    }*/

    fn visit_expression(&mut self, ctx: &ExpressionContext<'i>) {
        //self.visit_children(ctx)
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
        println!("{:?}", ctx);
        println!("{:?}", ctx.get_child(0));
        println!("{:?}", ctx.get_child(1));
        //println!("{:?}", self.visit_children(ctx));

        //ctx.accept(ctx.get_child(0).unwrap());

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
            .push(CompilationUnit::CUPrintStatement(value));
    }
}

#[test]
fn test_visitor() {
    // TODO: Actually get this to test something. Right now it exists to make sure things can compile.
    fn parse<'a>(tf: &'a ArenaCommonFactory<'a>) -> Rc<ScriptContext<'a>> {
        let mut _lexer = quickbmsLexer::new_with_token_factory(
            InputStream::new("print \"Hello, World!\"\n".into()),
            tf,
        );
        let token_source = CommonTokenStream::new(_lexer);
        let mut parser = quickbmsParser::new(token_source);
        let result = parser.script().expect("parsed unsuccessfully");

        let mut visitor = QuickBMSVisitorImpl {
            return_stack: vec![],
        };
        result.accept(&mut visitor);

        println!("{:?}", visitor.return_stack);
        //assert_eq!(visitor.0, vec!["d1", "d2"]);

        result
    }
    let tf = ArenaCommonFactory::default();

    let _result = parse(&tf);
}
