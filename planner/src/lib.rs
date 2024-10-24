mod planner;

use common::ast::Statement;
use common::schema::Table;
use common::types::Value;
use crate::planner::Planner;

// 执行节点
#[derive(Debug, PartialEq)]
pub enum Node {
    Create {
        schema: Table,
    },

    Insert {
        table_name: String,
        columns: Vec<String>,
        values: Vec<Vec<Value>>,
    },

    Scan {
        table_name: String,
    },
}

#[derive(Debug, PartialEq)]
pub struct Plan(pub Node);

impl Plan {
    pub fn build(stmt: Statement) -> Self {
        Planner::new().build(stmt)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use common::schema::Column;
    use common::types::DataType;
    use parser::Parser;

    #[test]
    fn test_plan_create_table() -> Result<()> {
        let mut sql = "
            create table users (
                a int default 0 not null,
                b float not null,
                c varchar null,
                d bool default true
                );
        ";

        let stmt = Parser::new(sql).parse()?;

        let plan = Plan::build(stmt);

        assert_eq!(plan, Plan(Node::Create {
            schema: Table {
                name: "users".to_string(),
                columns: vec![
                    Column {
                        name: "a".to_string(),
                        data_type: DataType::Integer,
                        nullable: false,
                        default: Some(Value::Integer(0)),
                    },
                    Column {
                        name: "b".to_string(),
                        data_type: DataType::Float,
                        nullable: false,
                        default: None,
                    },
                    Column {
                        name: "c".to_string(),
                        data_type: DataType::String,
                        nullable: true,
                        default: Some(Value::Null),
                    },
                    Column {
                        name: "d".to_string(),
                        data_type: DataType::Boolean,
                        nullable: false,
                        default: Some(Value::Boolean(true)),
                    },
                ],
            }
        }
        ));

        Ok(())
    }

    #[test]
    fn test_plan_insert() -> Result<()> {
        let mut sql = " insert into users values (1, 2.3, 'abc', true);";
        let mut stmt = Parser::new(sql).parse()?;
        let mut plan = Plan::build(stmt);

        assert_eq!(plan, Plan(Node::Insert {
            table_name: "users".to_string(),
            columns: vec![],
            values: vec![vec![Value::Integer(1), Value::Float(2.3), Value::String("abc".to_string()), Value::Boolean(true)]],
        }));

        sql = " insert into users (c1,c2,c3,c4) values (1, 2.3, 'abc', true), (2, 4.5, 'def', false);";
        stmt = Parser::new(sql).parse()?;
        plan = Plan::build(stmt);

        assert_eq!(plan, Plan(Node::Insert {
            table_name: "users".to_string(),
            columns: vec!["c1".to_string(), "c2".to_string(), "c3".to_string(), "c4".to_string()],
            values: vec![
                vec![Value::Integer(1), Value::Float(2.3), Value::String("abc".to_string()), Value::Boolean(true)],
                vec![Value::Integer(2), Value::Float(4.5), Value::String("def".to_string()), Value::Boolean(false)],
            ],
        }));
        Ok(())
    }

    #[test]
    fn test_plan_select() -> Result<()> {
        let sql = " select * from users;";
        let stmt = Parser::new(sql).parse()?;
        let plan = Plan::build(stmt);

        assert_eq!(plan, Plan(Node::Scan {
            table_name: "users".to_string(),
        }));

        Ok(())
    }
}
