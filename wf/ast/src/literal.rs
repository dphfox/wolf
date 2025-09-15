use crate::{AstRef, evaluatable::AnyEvaluatable};

#[derive(Debug, Clone)]
pub enum AnyLiteral {
    Name(AstRef<Name>),
    String(AstRef<LiteralString>),
    Tuple(AstRef<Tuple>),
}

#[derive(Debug, Clone)]
pub struct Name {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct LiteralString {
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct Tuple {
    pub content: Vec<AstRef<TupleEntry>>,
}

#[derive(Debug, Clone)]
pub struct TupleEntry {
	pub name: Option<AstRef<Name>>,
    pub value: AnyEvaluatable,
    pub flatten: bool,
}
