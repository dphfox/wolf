use std::collections::{HashMap, HashSet};

use crate::{
    AstRef,
    literal::{AnyLiteral, Name, Tuple},
};

#[derive(Debug, Clone)]
pub struct Block {
    pub let_captures: Vec<AstRef<LetCapture>>,
    pub evaluates_to: AnyEvaluatable,
}

#[derive(Debug, Clone)]
pub enum AnyEvaluatable {
    AssociatedValueOf(AnyLiteral),
    ApplyFunction(AstRef<AstApplyFunction>),
    Chain(AstRef<AstChainExpression>),
}

#[derive(Debug, Clone)]
pub struct AstAccessName {
    pub accessee: AnyEvaluatable,
    pub name: AstRef<Name>,
}

#[derive(Debug, Clone)]
pub struct AstApplyFunction {
    pub function: AstRef<Name>,
    pub datum: AstRef<Tuple>,
}

#[derive(Debug, Clone)]
pub struct AstChainExpression {
    pub looping: bool,
    pub block: AstRef<AstBlock>,
}
