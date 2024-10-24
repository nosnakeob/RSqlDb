use derive_new::new;
use common::ast::Statement;
use common::schema::Table;
use crate::{Node, Plan};

#[derive(Debug, new)]
pub struct Planner;

impl Planner {
    pub fn build(&mut self, stmt: Statement) -> Plan {
        Plan(self.build_statement(stmt))
    }

    fn build_statement(&self, stmt: Statement) -> Node {
        match stmt {
            Statement::Create { table_name, columns } => Node::Create {
                schema: Table {
                    name: table_name,
                    columns: columns.into_iter().map(Into::into).collect(),
                }
            },
            Statement::Insert { table_name, columns, values } => Node::Insert {
                table_name,
                columns: columns.unwrap_or_default(),
                values: values.into_iter().map(|v| v.into_iter().map(Into::into).collect()).collect(),
            },
            Statement::Select { table_name } => Node::Scan { table_name },
        }
    }
}