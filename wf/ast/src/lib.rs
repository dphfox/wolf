#![allow(dead_code)]

use std::{collections::HashMap, marker::PhantomData};
use std::{
    hash::{Hash, Hasher},
    rc::Rc,
};

use crate::literal::{LiteralString, Name, Tuple};

mod capture;
mod evaluatable;
mod literal;

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct AstRef<Node> {
    id: usize,
    _phantom: PhantomData<Node>,
}

impl<Node> PartialEq for AstRef<Node> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<Node> Eq for AstRef<Node> {}
impl<Node> Hash for AstRef<Node> {
    fn hash<State: Hasher>(&self, state: &mut State) {
        self.id.hash(state);
    }
}

pub trait AstStorage<Node> {
    fn register(&mut self, node: Node) -> AstRef<Node>;
    fn get(&mut self, id: AstRef<Node>) -> Rc<Node>;
}

macro_rules! ast_can_store {
    ($map:ident, $ty:ty) => {
        impl AstStorage<$ty> for Ast {
            fn register(&mut self, node: $ty) -> AstRef<$ty> {
                let id = self.next_id();
                self.$map.insert(id, Rc::new(node));
                AstRef {
                    id,
                    _phantom: PhantomData,
                }
            }

            fn get(&mut self, id: AstRef<$ty>) -> Rc<$ty> {
                self.$map[&id.id].clone()
            }
        }
    };
}

pub struct Ast {
    next_id: usize,
    literal_names: HashMap<usize, Rc<Name>>,
    literal_strings: HashMap<usize, Rc<LiteralString>>,
    literal_tuples: HashMap<usize, Rc<Tuple>>,
}
ast_can_store!(literal_names, Name);
ast_can_store!(literal_strings, LiteralString);
ast_can_store!(literal_tuples, Tuple);

impl Ast {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            literal_names: HashMap::new(),
            literal_strings: HashMap::new(),
            literal_tuples: HashMap::new(),
        }
    }

    fn next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

impl Default for Ast {
    fn default() -> Self {
        Self::new()
    }
}
