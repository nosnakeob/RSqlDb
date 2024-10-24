use crate::types::DataType;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Create { table_name: String, columns: Vec<Column> },
    Insert {
        table_name: String,
        columns: Option<Vec<String>>,
        values: Vec<Vec<Expression>>,
    },
    Select { table_name: String },
}

#[derive(Debug, PartialEq)]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
    pub nullable: Option<bool>,
    pub default: Option<Expression>,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Constant(Constant)
}

impl From<Constant> for Expression {
    fn from(c: Constant) -> Self {
        Self::Constant(c)
    }
}

#[derive(Debug, PartialEq)]
pub enum Constant {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}