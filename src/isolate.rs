use crate::value::{NativeThunk, Thunk, Value};
use std::collections::HashMap;
use std::fmt;

#[derive(fmt::Debug)]
pub enum RuntimeError {
    AlreadyBound { name: String },
    Unbound { name: String },
    NotCallable { name: String },
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AlreadyBound { name } => {
                write!(f, "{:?} is already bound. ", name)
            },
            Self::Unbound { name } => {
                write!(f, "{:?} is not bound to any value. ", name)
            },
            Self::NotCallable { name } => {
                write!(f, "{:?} is not callable. ", name)
            }
        }
    }
}

pub struct Namespace {
    pub variables: HashMap<String, Value>,
}

impl Namespace {
    pub fn new() -> Namespace {
        Namespace {
            variables: HashMap::new(),
        }
    }

    pub fn bind(&mut self, name: &String, value: Value) -> Result<(), RuntimeError> {
        if self.variables.contains_key(name) {
            Err(RuntimeError::AlreadyBound { name: name.clone() })
        } else {
            self.variables.insert(name.clone(), value);
            Ok(())
        }
    }
}

pub struct Isolate {
    stack: Vec<Thunk>,
    pub namespaces: Vec<Namespace>,
}

impl Isolate {
    pub fn new() -> Isolate {
        let mut global_namespace = Namespace::new();

        global_namespace
            .bind(
                &String::from("debug"),
                Value::NativeThunk(NativeThunk {
                    function: |input| {
                        println!("{:?}", input.parameters);
                        Ok(Value::None)
                    },
                }),
            )
            .unwrap();

        Isolate {
            stack: vec![],
            namespaces: vec![global_namespace],
        }
    }

    pub fn bind(&mut self, name: &String, value: Value) -> Result<(), RuntimeError> {
        self.namespaces.last_mut().unwrap().bind(name, value)
    }

    pub fn resolve(&self, name: &String) -> Option<&Value> {
        for ns in self.namespaces.iter().rev() {
            if ns.variables.contains_key(name) {
                return ns.variables.get(name);
            }
        }

        None
    }
}
