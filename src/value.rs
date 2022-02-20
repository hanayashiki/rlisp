use std::collections::HashMap;
use std::fmt::Debug;

use crate::{ast::Expr, isolate::RuntimeError};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i32),
    String(String),
    None,
    Thunk(Thunk),
    NativeThunk(NativeThunk),
}

#[derive(Debug, Clone)]
pub struct Thunk {
    source: Rc<dyn Expr>,
    closure: HashMap<String, Value>,
}

pub struct NativeThunkInput {
    pub parameters: Vec<Value>,
}

#[derive(Debug, Clone)]
pub struct NativeThunk {
    pub function: fn(input: NativeThunkInput) -> Result<Value, RuntimeError>,
}
