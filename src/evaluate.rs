use crate::ast::*;
use crate::isolate::{Isolate, RuntimeError};
use crate::value::{NativeThunkInput, Value};

pub trait Evaluatable {
    fn evaluate(&self, isolate: &mut Isolate) -> Result<Value, RuntimeError>;
}

impl Evaluatable for DefineExpr {
    fn evaluate(&self, isolate: &mut Isolate) -> Result<Value, RuntimeError> {
        let value = self.value.evaluate(isolate)?;
        isolate.bind(&self.identifier.identifer, value)?;
        Ok(Value::None)
    }
}

impl Evaluatable for CallExpr {
    fn evaluate(&self, isolate: &mut Isolate) -> Result<Value, RuntimeError> {
        let function_value = self.function.evaluate(isolate)?;

        match function_value {
            Value::NativeThunk(native_thunk) => {
                let parameters_result: Result<Vec<_>, _> = self
                    .parameters
                    .iter()
                    .map(|expr| expr.evaluate(isolate))
                    .collect();

                let parameters = parameters_result?;

                let input = NativeThunkInput { parameters };

                (native_thunk.function)(input)
            }
            _ => Err(RuntimeError::NotCallable { name: "function".to_string() }), // TODO: restore the AST to string
        }
    }
}

impl Evaluatable for IdentifierExpr {
    fn evaluate(&self, isolate: &mut Isolate) -> Result<Value, RuntimeError> {
        if let Some(v) = isolate.resolve(&self.identifer) {
            Ok(v.clone())
        } else {
            Err(RuntimeError::Unbound { name: self.identifer.clone() })
        }
    }
}

impl Evaluatable for IntegerLiteral {
    fn evaluate(&self, _isolate: &mut Isolate) -> Result<Value, RuntimeError> {
        Ok(Value::Integer(self.value))
    }
}

impl Evaluatable for StringLiteral {
    fn evaluate(&self, _isolate: &mut Isolate) -> Result<Value, RuntimeError> {
        Ok(Value::String(self.value.clone()))
    }
}

impl Evaluatable for Program {
    fn evaluate(&self, isolate: &mut Isolate) -> Result<Value, RuntimeError> {
        let result: Result<Vec<_>, _> = self
            .exprs
            .iter()
            .map(|expr| expr.evaluate(isolate))
            .collect();

        result?;

        Ok(Value::None)
    }
}
