use oxc_ast::AstOwnedKind;
use oxc_semantic::{AstNode, SymbolId};
use oxc_span::CompactStr;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Default)]
pub struct TransformCtx<'a> {
    _marker: std::marker::PhantomData<&'a ()>,

    removed_symbols: FxHashSet<SymbolId>,
    renamed_symbols: FxHashMap<SymbolId, CompactStr>,
    replaced_node: Option<AstOwnedKind<'a>>,
}

impl<'a> TransformCtx<'a> {
    pub fn remove_symbol(&mut self, symbol_id: SymbolId) -> &mut Self {
        self.removed_symbols.insert(symbol_id);
        self
    }

    pub fn rename_symbol(
        &mut self,
        symbol_id: SymbolId,
        new_name: impl Into<CompactStr>,
    ) -> &mut Self {
        self.renamed_symbols.insert(symbol_id, new_name.into());
        self
    }

    pub fn replace_node(&mut self, node: AstOwnedKind<'a>) -> &mut Self {
        self.replaced_node = Some(node);
        self
    }

    pub fn get_replaced_node(&mut self) -> Option<AstOwnedKind<'a>> {
        self.replaced_node.take()
    }
}

pub trait Transformer {
    fn transform<'a>(&mut self, _node: &AstOwnedKind<'a>, _ctx: &mut TransformCtx<'a>) {}

    fn transform_on_leave<'a>(&mut self, _node: &AstOwnedKind<'a>, _ctx: &mut TransformCtx<'a>) {}
}

pub type BoxedTransformer = Box<dyn Transformer>;
