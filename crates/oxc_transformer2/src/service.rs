use std::{cell::RefCell, rc::Rc};

use oxc_allocator::Allocator;
use oxc_ast::{ast::Program, AstBuilder, AstKind, Visit, VisitMut};
use oxc_semantic::{AstNode, ScopeFlags, Semantic, SemanticBuilder};
use oxc_span::SourceType;

use crate::{
    context::TransformerCtx,
    options::{TransformOptions, TransformTarget},
    transformer::BoxedTransformer,
};

pub struct TransformerService<'a> {
    ctx: TransformerCtx<'a>,
    options: TransformOptions,

    semantic_builder: SemanticBuilder<'a>,
    transformers: Vec<BoxedTransformer>,
}

impl<'a> TransformerService<'a> {
    pub fn new(
        allocator: &'a Allocator,
        source_type: SourceType,
        semantic: Semantic<'a>,
        options: TransformOptions,
    ) -> Self {
        let semantic_builder = SemanticBuilder::new(semantic.source_text(), source_type);

        let ast = Rc::new(AstBuilder::new(allocator));
        let ctx = TransformerCtx::new(Rc::clone(&ast), Rc::new(RefCell::new(semantic)));

        // Order is important!
        let mut transformers = vec![];
        transformers.extend(crate::typescript::preset());

        if options.target < TransformTarget::ES2021 {
            transformers.extend(crate::es2021::preset());
        }

        Self { ctx, options, semantic_builder, transformers }
    }

    pub fn run(mut self, program: &mut Program<'a>) -> Result<(), Vec<Error>> {
        self.visit_program(program);

        Ok(())
    }

    fn run_transformers(&mut self, node: &AstNode) {}
}

impl<'a> VisitMut<'a> for TransformerService<'a> {
    fn enter_node(&mut self, kind: AstKind<'a>) {
        self.semantic_builder.enter_node(kind);
    }

    fn leave_node(&mut self, kind: AstKind<'a>) {
        self.semantic_builder.leave_node(kind);
    }

    fn enter_scope(&mut self, flags: ScopeFlags) {
        self.semantic_builder.enter_scope(flags);
    }

    fn leave_scope(&mut self) {
        self.semantic_builder.leave_scope();
    }
}
