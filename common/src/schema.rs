use crate::ast;
use crate::types::{DataType, Value};

#[derive(Debug,PartialEq)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
}

#[derive(Debug,PartialEq)]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
    pub nullable: bool,
    pub default: Option<Value>,
}

impl From<ast::Column> for Column {
    fn from(value: ast::Column) -> Self {
        let nullable = value.nullable.unwrap_or(false);
        Self {
            name: value.name,
            data_type: value.data_type,
            nullable,
            default: match value.default {
                Some(expr) => Some(expr.into()),
                // 允许为空时,默认值可为空
                None if nullable => Some(Value::Null),
                None => None,
            },
        }
    }
}