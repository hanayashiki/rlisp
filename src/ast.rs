use std::fmt::Debug;
use crate::evaluate;

#[derive(Debug, Clone)]
pub struct Location {
    pub offset: i32,
    pub col: i32,
    pub row: i32,
}

pub trait Node: Debug + evaluate::Evaluatable {
    fn location(&self) -> &Location;
}

pub trait Expr: Node {
}

#[derive(Debug)]
pub struct DefineExpr {
    pub location: Location,
    pub identifier: Box<IdentifierExpr>,
    pub value: Box<dyn Expr>,
}

impl Node for DefineExpr {
    fn location(&self) -> &Location {
        &self.location
    }
}

impl Expr for DefineExpr {
}

#[derive(Debug)]
pub struct CallExpr {
    pub location: Location,
}

impl Expr for CallExpr {
}

impl Node for CallExpr {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Debug)]
pub struct IdentifierExpr {
    pub location: Location,
    pub identifer: String,
}

impl Expr for IdentifierExpr {
}

impl Node for IdentifierExpr {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Debug)]
pub struct IntegerLiteral {
    pub location: Location,
    pub value: i32,
}

impl Expr for IntegerLiteral {
}

impl Node for IntegerLiteral {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Debug)]
pub struct Program {
    pub location: Location,
    pub exprs: Vec<Box<dyn Expr>>,
}

impl Node for Program {
    fn location(&self) -> &Location {
        &self.location
    }
}
