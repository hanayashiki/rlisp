use std::fmt::Debug;
use std::rc::Rc;
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
    pub identifier: Rc<IdentifierExpr>,
    pub value: Rc<dyn Expr>,
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
    pub function: Rc<dyn Expr>,
    pub parameters: Vec<Rc<dyn Expr>>,
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
pub struct StringLiteral {
    pub location: Location,
    pub value: String,
}

impl Expr for StringLiteral {
}

impl Node for StringLiteral {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Debug)]
pub struct Program {
    pub location: Location,
    pub exprs: Vec<Rc<dyn Expr>>,
}

impl Node for Program {
    fn location(&self) -> &Location {
        &self.location
    }
}
