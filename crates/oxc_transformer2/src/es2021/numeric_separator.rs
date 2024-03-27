use oxc_ast::AstKind;
use oxc_semantic::AstNode;

use crate::{context::TransformerCtx, transformer::Transformer};

pub struct NumericSeparator;

impl Transformer for NumericSeparator {
    fn transform<'a>(&mut self, node: &mut AstNode<'a>, _ctx: &TransformerCtx<'a>) {
        if let AstKind::NumericLiteral(lit) = node.kind() {
            if lit.raw.contains('_') {
                //
            }
        }
    }
}
