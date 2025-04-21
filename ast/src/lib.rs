use std::rc::Rc;

pub struct File {
    pub definitions: Vec<ConstCapture>
}

pub struct ConstCapture {
    pub pattern: Box<PatternCapture>,
    pub expression: Box<ConstExpression>
}

pub enum PatternCapture {
    Single {
        capture_as: Option<Word>,
        shape: Box<ConstExpression>,
        inner: Option<PatternCapture>
    },
    Tuple {
        capture_as: Option<Word>,
        inner: Vec<PatternCapture>
    },
    Map {
        capture_as: Option<Word>,
        inner: Vec<PatternCapture>
    }
}

pub enum ConstExpression {
    Omitted,
    Unique(Unique),
    Number(f64),
    String {
        prefix: String,
        interpolants_suffixes: Vec<(ConstExpression, String)>
    },
    Function(ConstFunction),
    Alias(Word),
    PrefixOperator {
        operation: ConstOperation,
        right: Box<ConstExpression>
    },
    InfixOperator {
        operation: ConstOperation,
        left: Box<ConstExpression>,
        right: Box<ConstExpression>
    },
    PostfixOperator {
        operation: ConstOperation,
        left: Box<ConstExpression>
    }
}

pub enum ConstOperation {
    Alias(Word),
    Function(ConstFunction),
}

pub enum Function {
    Const(ConstFunction)
}

pub struct ConstFunction {
    pub pattern: Box<PatternCapture>,
    pub definitions: Vec<ConstCapture>,
    pub evaluates_to: Box<ConstExpression>
}

pub struct Unique(usize);
pub struct Word(String);

