use oxc_semantic::AstNode;

use crate::context::TransformerCtx;

pub trait Transformer {
    fn transform<'a>(&mut self, _node: &mut AstNode<'a>, _ctx: &TransformerCtx<'a>) {}

    fn transform_leave<'a>(&mut self, _node: &mut AstNode<'a>, _ctx: &TransformerCtx<'a>) {}
}

pub type BoxedTransformer = Box<dyn Transformer>;
