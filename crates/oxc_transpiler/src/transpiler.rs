use std::{
    cell::{Ref, RefCell, RefMut},
    mem,
    rc::Rc,
};

use oxc_allocator::Allocator;
use oxc_ast::{ast::Program, visit::walk_mut::*, AstBuilder, AstKind, Visit, VisitMut};
use oxc_diagnostics::Error;
use oxc_semantic::{
    AstNode, ScopeFlags, ScopeId, ScopeTree, Semantic, SemanticBuilder, SymbolId, SymbolTable,
};
use oxc_span::{CompactStr, SourceType};

use crate::{
    options::{TransformOptions, TransformTarget},
    transformer::{BoxedTransformer, TransformCtx},
};

#[derive(Clone)]
pub struct TranspilerCtx<'a> {
    pub ast: Rc<AstBuilder<'a>>,
    semantic: Rc<RefCell<Semantic<'a>>>,
    errors: Rc<RefCell<Vec<Error>>>,
}

impl<'a> TranspilerCtx<'a> {
    pub fn new(ast: Rc<AstBuilder<'a>>, semantic: Rc<RefCell<Semantic<'a>>>) -> Self {
        Self { ast, semantic, errors: Rc::new(RefCell::new(vec![])) }
    }
}

pub struct Transpiler<'a> {
    ctx: TranspilerCtx<'a>,
    options: TransformOptions,

    semantic_builder: SemanticBuilder<'a>,
    transformers: Vec<BoxedTransformer>,
}

impl<'a> Transpiler<'a> {
    pub fn new(
        allocator: &'a Allocator,
        source_type: SourceType,
        semantic: Semantic<'a>,
        options: TransformOptions,
    ) -> Self {
        let semantic_builder = SemanticBuilder::new(semantic.source_text(), source_type);

        let ast = Rc::new(AstBuilder::new(allocator));
        let ctx = TranspilerCtx::new(Rc::clone(&ast), Rc::new(RefCell::new(semantic)));

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

    fn run_transformers<'t>(&mut self, mut node: AstKind<'t>, on_leave: bool) -> AstKind<'t> {
        for transformer in &mut self.transformers {
            let mut ctx = TransformCtx::default();

            if on_leave {
                transformer.transform_on_leave(&node, &mut ctx);
            } else {
                transformer.transform(&node, &mut ctx);
            }

            if let Some(new_node) = ctx.get_replaced_node() {
                node = new_node;
            }

            // TODO apply semantic changes
        }

        node
    }
}

impl<'a> VisitMut<'a> for Transpiler<'a> {
    fn visit_program(&mut self, program: &mut Program<'a>) {
        let cloned_program = program.clone();
        // let mut node = AstKind::Program(cloned_program);

        // node = self.run_transformers(node, false);

        // if let AstKind::Program(inner) = &mut node {
        //     walk_program_mut(self, inner);
        // } else {
        //     panic!("Invalid node kind returned!");
        // }

        // node = self.run_transformers(node, true);
    }
}
