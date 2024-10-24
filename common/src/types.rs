use crate::ast::{Const, Expression};

#[derive(Debug, PartialEq)]
pub enum DataType {
    Integer,
    Float,
    String,
    Boolean,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}

impl From<Expression> for Value {
    fn from(expr: Expression) -> Self {
        match expr {
            Expression::Const(Const::Null) => Value::Null,
            Expression::Const(Const::Boolean(v)) => Value::Boolean(v),
            Expression::Const(Const::Integer(v)) => Value::Integer(v),
            Expression::Const(Const::Float(v)) => Value::Float(v),
            Expression::Const(Const::String(v)) => Value::String(v),
        }
    }
}