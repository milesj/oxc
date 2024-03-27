use std::{
    cell::{Ref, RefCell, RefMut},
    mem,
    rc::Rc,
};

use oxc_ast::AstBuilder;
use oxc_diagnostics::Error;
use oxc_semantic::{ScopeId, ScopeTree, Semantic, SymbolId, SymbolTable};
use oxc_span::{CompactStr, SourceType};

#[derive(Clone)]
pub struct TransformerCtx<'a> {
    pub ast: Rc<AstBuilder<'a>>,
    semantic: Rc<RefCell<Semantic<'a>>>,
    errors: Rc<RefCell<Vec<Error>>>,
}

impl<'a> TransformerCtx<'a> {
    pub fn new(ast: Rc<AstBuilder<'a>>, semantic: Rc<RefCell<Semantic<'a>>>) -> Self {
        Self { ast, semantic, errors: Rc::new(RefCell::new(vec![])) }
    }
}
