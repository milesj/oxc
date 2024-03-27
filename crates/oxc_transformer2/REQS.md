Requirements

- [ ] Traverse upwards through parents
- [ ] Be able to mutate parents
- [ ] Cannot mutate a child and parent together
- [ ] Scopes/symbols/etc need to be updated when something is mutated
- [ ] Avoid ownership issues (ditch `Visit[Mut]`?)

Nice to haves

- [ ] Traverse sideways through siblings (can be done via parent downwards)
- [ ] Parallelization (per file)

# Different patterns for mutating

### Immutable AST builds a mutable AST on-demand

With this pattern, we would have 2 ASTs, the 1st is the original immutable AST, and the 2nd is a new mutable AST (starts at `Program`) that gets built on-demand while visiting the original AST. Nodes are cloned from the 1st into the 2nd, and only mutated in the 2nd. Cloning should be cheap since we use lifetimes/references for inner string data.

Pros

- Easy to understand
- Doesn't mutate the original AST

Cons

- Traversing/mutating parent still very difficult
- How to keep semantic in sync???

```rust
impl Transformer for X {
    fn transform(&mut self, old_node: &AstNode) -> Result<AstNode> {
        let mut new_node = old_node.clone();
        // Mutate node

        Ok(new_node)
    }
```

### Enter/Leave specific transform methods

Instead of having a single `transform(in) -> out` method, we would have 2 methods, `enter(in) -> out` and `leave(in) -> out`, which would fire when traversing down the tree, and then back upwards. With this pattern, if a child must mutate its parent, it can set a flag in `enter`, and then in `leave`, it can handle the mutation based on the flag.

Pros

- Easy to understand
- Mutates in place
- Traversing/mutating parent is easier

Cons

- If the parent mutates the children in `leave`, it will need to be re-traversed. Potentially problematic?
- How to keep semantic in sync???

```rust
impl Transformer for X {
    fn transform(&mut self, node: &AstNode) -> Result<Option<AstNode>> {
        match node.kind() {
            SomeNode => {
                if self.ctx.is_within_parent(...) {
                    // Flag parent changes within child
                    self.in_function_body = true;
                }
            }
        };

        Ok(None)
    }

    fn transform_on_leave(&mut self, node: &AstNode) -> Result<Option<AstNode>> {
        if self.in_function_body && node.kind() == Function {
            let mut node = node.clone();
            // Mutate parent and return a replacement node

            return Ok(Some(node));
        }

        Ok(None)
    }
}
```

### No child -> parent traversal, instead parent -> child visit pre-checks

This pattern goes against our requirement of "traversing up to the parent from the child", but instead offers an alternative. If we have situations where a child needs to detect if it's within a certain parent, or that parent needs to be mutated base on the status of a child, we can solve these by implementing a custom `Visit` and running it within the parent.

Pros

- Existing transformers will still work the same
- No reverse traversal necessary
- Mutating the parent _before_ mutating the child, avoids re-traversals

Cons

- Requires traversing the children more than necessary
- More work in the parent instead of the child
- Possibly more overhead
- How to keep semantic in sync???

```rust
struct IsChildInParent {
    pub yes: bool,
}

impl Visit for IsChildInParent {
    fn enter_node(&mut self, kind: &AstKind) {
        match kind {
            Child1 | Child2 => {
                self.yes = true;
            }
            _ => {}
        };
    }
}

impl Transformer for X {
    fn transform(&mut self, node: &AstNode) -> Result<Option<AstNode>> {
        match node.kind() {
            ParentNode => {
                let visitor = IsChildInParent { yes: false };
                visitor.visit_parent_node(&ast.node);

                if visitor.yes {
                    // Child exists within this parent,
                    // so mutate the parent from this context
                }
            }
            ChildNode => {
                // Mutate child if need be
            }
        };

        Ok(None)
    }
}
```
