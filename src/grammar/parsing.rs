use antlr_rust::tree::{
    ParseTree, ParseTreeListener, ParseTreeVisitor, ParseTreeWalker, TerminalNode, Tree,
    VisitChildren, Visitable,
};

use crate::grammar::quickbmsparser::{
    quickbmsParserContext, quickbmsParserContextType, ExpressionContext, Print_statementContext,
    ScriptContext, StatementContext,
};
use crate::grammar::quickbmsvisitor::quickbmsVisitor;

struct QuickBMSVisitor<'i, T>(Vec<&'i str>, T);

impl<'i, T> ParseTreeVisitor<'i, quickbmsParserContextType> for QuickBMSVisitor<'i, T> {
    fn visit_terminal(&mut self, node: &TerminalNode<'i, quickbmsParserContextType>) {
        /*if node.symbol.get_token_type() == csvparser::TEXT {
            if let Cow::Borrowed(s) = node.symbol.text {
                self.0.push(s);
            }
        }*/

        panic!()
    }
}

impl<'i, T> quickbmsVisitor<'i> for QuickBMSVisitor<'i, T> {
    fn visit_script(&mut self, ctx: &ScriptContext<'i>) {
        self.visit_children(ctx)
    }

    fn visit_statement(&mut self, ctx: &StatementContext<'i>) {
        self.visit_children(ctx)
    }

    fn visit_expression(&mut self, ctx: &ExpressionContext<'i>) {
        self.visit_children(ctx)
    }

    fn visit_print_statement(&mut self, ctx: &Print_statementContext<'i>) {
        self.visit_children(ctx)
    }
}
