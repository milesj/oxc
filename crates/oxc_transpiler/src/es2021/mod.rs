use crate::transformer::BoxedTransformer;

mod numeric_separator;

pub fn preset() -> Vec<BoxedTransformer> {
    // vec![Box::new(numeric_separator::NumericSeparator)]
    vec![]
}
