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
    VisitChildren, Visitable,
};
use antlr_rust::InputStream;

use crate::grammar::ast::{CompilationUnit, Expression, PrintStatement};
use crate::grammar::quickbmslexer::*;
use crate::grammar::quickbmslistener::*;
use crate::grammar::quickbmsparser::{
    quickbmsParser, quickbmsParserContext, quickbmsParserContextType, ExpressionContext,
    Print_statementContext, ScriptContext, StatementContext,
};
use crate::grammar::quickbmsvisitor::quickbmsVisitor;

struct QuickBMSVisitorImpl<'i, CompilationUnit>(Vec<&'i str>, CompilationUnit);

impl<'i, CompilationUnit> ParseTreeVisitor<'i, quickbmsParserContextType>
    for QuickBMSVisitorImpl<'i, CompilationUnit>
{
    fn visit_terminal(&mut self, node: &TerminalNode<'i, quickbmsParserContextType>) {
        /*if node.symbol.get_token_type() == csvparser::TEXT {
            if let Cow::Borrowed(s) = node.symbol.text {
                self.0.push(s);
            }
        }*/

        panic!()
    }
}

impl<'i, CompilationUnit> quickbmsVisitor<'i> for QuickBMSVisitorImpl<'i, CompilationUnit> {
    /*fn visit_script(&mut self, ctx: &ScriptContext<'i>) {
        self.visit_children(ctx)
    }

    fn visit_statement(&mut self, ctx: &StatementContext<'i>) {
        self.visit_children(ctx)
    }

    fn visit_expression(&mut self, ctx: &ExpressionContext<'i>) {
        self.visit_children(ctx)
    }*/

    fn visit_print_statement(&mut self, ctx: &Print_statementContext<'i>) {
        println!("{:?}", self.visit_children(ctx))
    }
}

#[test]
fn test_visitor() {
    fn parse<'a>(tf: &'a ArenaCommonFactory<'a>) -> Rc<ScriptContext<'a>> {
        let mut _lexer = quickbmsLexer::new_with_token_factory(
            InputStream::new("print \"Hello, World!\"\n".into()),
            tf,
        );
        let token_source = CommonTokenStream::new(_lexer);
        let mut parser = quickbmsParser::new(token_source);
        let result = parser.script().expect("parsed unsuccessfully");

        let mut test = 5;
        let mut visitor = QuickBMSVisitorImpl(Vec::new(), &mut test);
        result.accept(&mut visitor);
        assert_eq!(visitor.0, vec!["d1", "d2"]);

        result
    }
    let tf = ArenaCommonFactory::default();

    let _result = parse(&tf);
}
