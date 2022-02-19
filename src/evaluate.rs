use crate::ast::*;

pub trait Evaluatable {
    fn evaluate(&self);
}

impl Evaluatable for DefineExpr {
  fn evaluate(&self) {
  }
}

impl Evaluatable for CallExpr {
  fn evaluate(&self) {
  }
}

impl Evaluatable for IdentifierExpr {
  fn evaluate(&self) {
  }
}

impl Evaluatable for IntegerLiteral {
  fn evaluate(&self) {
  }
}

impl Evaluatable for Program {
  fn evaluate(&self) {
  }
}
