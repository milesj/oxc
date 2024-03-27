pub struct TransformOptions {
    pub target: TransformTarget,
}

/// See <https://www.typescriptlang.org/tsconfig#target>
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum TransformTarget {
    ES3,
    ES5,
    ES2015,
    ES2016,
    ES2017,
    ES2018,
    ES2019,
    ES2020,
    ES2021,
    ES2022,
    ES2024,
    #[default]
    ESNext,
}
