use crate::ast::Expr;

pub enum Value {
    Integer(i32),
    Thunk(Thunk)
}

pub struct Thunk {
    source: Box<dyn Expr>,
    
}