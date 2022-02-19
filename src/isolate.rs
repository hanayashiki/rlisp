use crate::ast::Location;
use crate::value::Value;
use std::collections::HashMap;
use std::fmt;

pub struct Isolate {

}

pub struct Namespace {
    variables: HashMap<String, Value>,
}

#[derive(fmt::Debug)]
pub enum RuntimeError {
    AlreadyBound {
        location: Location,
        name: String,
    },
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AlreadyBound { location, name } => {
                write!(
                    f,
                    "",
                )
            }
        }

    }
}

impl Namespace {
    pub fn bind(&mut self, name: String, value: Value) -> Result<(), RuntimeError> {
        self.variables.insert(name, value);
        Ok(())
    }
}
