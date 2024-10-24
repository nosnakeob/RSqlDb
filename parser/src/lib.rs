use std::iter::Peekable;
use crate::ast::{Column, Constant, Expression, Statement};
use crate::lexer::Lexer;
use anyhow::{anyhow, bail, Result};
use crate::token::{Token, Keyword, Symbol};
use crate::types::DataType;

mod lexer;
mod token;
mod ast;
mod types;

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input).peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Statement> {
        let stmt = self.parse_statement()?;

        // 语句后面必须是分号
        self.next_expect(&Token::Symbol(Symbol::Semicolon))?;

        // 分号之后还有东西
        if let Ok(token) = self.peek() {
            bail!("Unexpected token: {:?}", token);
        }

        Ok(stmt)
    }

    fn parse_statement(&mut self) -> Result<Statement> {
        match self.peek()? {
            Token::Keyword(Keyword::Create) => self.parse_ddl(),
            Token::Keyword(Keyword::Select) => self.parse_select(),
            Token::Keyword(Keyword::Insert) => self.parse_insert(),
            token => bail!("Unexpected token: {:?}", token),
        }
    }

    fn parse_ddl(&mut self) -> Result<Statement> {
        match (self.next()?, self.next()?) {
            (Token::Keyword(Keyword::Create), Token::Keyword(Keyword::Table)) => self.parse_ddl_create_table(),
            (token1, token2) => bail!("Not a ddl statement: {:?}, {:?}", token1, token2),
        }
    }

    fn parse_select(&mut self) -> Result<Statement> {
        // select * from
        self.next_expect(&Token::Keyword(Keyword::Select))?;
        self.next_expect(&Token::Symbol(Symbol::Asterisk))?;
        self.next_expect(&Token::Keyword(Keyword::From))?;

        let table_name = self.next_ident()?;

        Ok(Statement::Select { table_name })
    }

    fn parse_insert(&mut self) -> Result<Statement> {
        // insert into tbl (col1, col2) values (1, 'abc'), (2, 'def')
        self.next_expect(&Token::Keyword(Keyword::Insert))?;
        self.next_expect(&Token::Keyword(Keyword::Into))?;

        let table_name = self.next_ident()?;

        let columns = if self.next_expect(&Token::Symbol(Symbol::OpenParen)).is_ok() {
            let mut cols = vec![];

            loop {
                cols.push(self.next_ident()?);

                match self.next()? {
                    Token::Symbol(Symbol::CloseParen) => break,
                    Token::Symbol(Symbol::Comma) => continue,
                    token => bail!("Unexpected token: {:?}", token),
                }
            }
            Some(cols)
        } else {
            None
        };

        self.next_expect(&Token::Keyword(Keyword::Values))?;

        let mut values = vec![];

        loop {
            self.next_expect(&Token::Symbol(Symbol::OpenParen))?;

            let mut exprs = vec![];

            loop {
                exprs.push(self.parse_expression()?);

                match self.next()? {
                    Token::Symbol(Symbol::CloseParen) => break,
                    Token::Symbol(Symbol::Comma) => continue,
                    token => bail!("Unexpected token: {:?}", token),
                }
            }
            values.push(exprs);

            if self.next_expect(&Token::Symbol(Symbol::Comma)).is_err() {
                break;
            }
        }

        Ok(Statement::Insert { table_name, columns, values })
    }

    fn parse_ddl_create_table(&mut self) -> Result<Statement> {
        let table_name = self.next_ident()?;

        self.next_expect(&Token::Symbol(Symbol::OpenParen))?;

        let mut columns = vec![];

        loop {
            columns.push(self.parse_ddl_column()?);

            if self.next_expect(&Token::Symbol(Symbol::Comma)).is_err() {
                break;
            }
        }

        self.next_expect(&Token::Symbol(Symbol::CloseParen))?;

        Ok(Statement::Create { table_name, columns })
    }

    fn parse_ddl_column(&mut self) -> Result<Column> {
        let mut col = Column {
            name: self.next_ident()?,
            data_type: match self.next()? {
                Token::Keyword(Keyword::Integer) | Token::Keyword(Keyword::Int) => DataType::Integer,
                Token::Keyword(Keyword::Bool) | Token::Keyword(Keyword::Boolean) => DataType::Boolean,
                Token::Keyword(Keyword::Float) => DataType::Float,
                Token::Keyword(Keyword::String) | Token::Keyword(Keyword::Text) | Token::Keyword(Keyword::Varchar) => DataType::String,
                token => bail!("Unexpected token: {:?}", token),
            },
            nullable: None,
            default: None,
        };

        while let Some(Token::Keyword(keyword)) = self.lexer.next_if(|token| matches!(token, Token::Keyword(_))) {
            match keyword {
                Keyword::Null => col.nullable = Some(true),
                Keyword::Not => {
                    self.next_expect(&Token::Keyword(Keyword::Null))?;
                    col.nullable = Some(false);
                }
                Keyword::Default => col.default = Some(self.parse_expression()?),
                k => bail!("Unexpected keyword: {:?}", k),
            }
        }

        Ok(col)
    }

    fn parse_expression(&mut self) -> Result<Expression> {
        Ok(match self.next()? {
            Token::Number(n) => {
                if n.chars().all(|c| c.is_ascii_digit()) {
                    Constant::Integer(n.parse()?).into()
                } else {
                    Constant::Float(n.parse()?).into()
                }
            }
            Token::String(s) => Constant::String(s).into(),
            Token::Keyword(Keyword::True) => Constant::Boolean(true).into(),
            Token::Keyword(Keyword::False) => Constant::Boolean(false).into(),
            Token::Keyword(Keyword::Null) => Constant::Null.into(),
            exp => bail!("Unexpected expression token: {:?}", exp),
        })
    }

    fn peek(&mut self) -> Result<&Token> {
        self.lexer.peek().ok_or(anyhow!("Unexpected end of input"))
    }

    fn next(&mut self) -> Result<Token> {
        self.lexer.next().ok_or(anyhow!("Unexpected end of input"))
    }

    fn next_ident(&mut self) -> Result<String> {
        match self.next()? {
            Token::Ident(ident) => Ok(ident),
            token => bail!("Expected ident, got {:?}", token),
        }
    }

    // 匹配下一个token,成功则消耗并返回匹配的token;否则错误
    fn next_expect(&mut self, expected: &Token) -> Result<Token> {
        match self.peek()? {
            token if token == expected => Ok(self.next()?),
            token => bail!("Expected {:?}, got {:?}", expected, token),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_create_table() -> Result<()> {
        let mut sql = "
            create table users (
                a int default 0 not null,
                b float not null,
                c varchar null,
                d bool default true
                );
        ";

        assert_eq!(Parser::new(sql).parse()?, Statement::Create {
            table_name: "users".to_string(),
            columns: vec![
                Column {
                    name: "a".to_string(),
                    data_type: DataType::Integer,
                    nullable: Some(false),
                    default: Some(Constant::Integer(0).into()),
                },
                Column {
                    name: "b".to_string(),
                    data_type: DataType::Float,
                    nullable: Some(false),
                    default: None,
                },
                Column {
                    name: "c".to_string(),
                    data_type: DataType::String,
                    nullable: Some(true),
                    default: None,
                },
                Column {
                    name: "d".to_string(),
                    data_type: DataType::Boolean,
                    nullable: None,
                    default: Some(Constant::Boolean(true).into()),
                },
            ],
        });

        sql = "
            create tabe users (
                a int default 0 not null,
                b float not null,
                c varchar null,
                d bool default true
                );
        ";

        assert_eq!(Parser::new(sql).parse().unwrap_err().to_string(), r#"Not a ddl statement: Keyword(Create), Ident("tabe")"#);

        sql = "
            create table users (
                a int default 0 not null,
                b float not null,
                c varchar null,
                d bool default true
                );create
        ";
        assert_eq!(Parser::new(sql).parse().unwrap_err().to_string(), r#"Unexpected token: Keyword(Create)"#);

        sql = "
            create table users (
                a int default 0 not null,
                b float not null,
                c varchar null,
                d bool default true
                )
        ";

        // println!("{:#?}", Parser::new(sql).parse().unwrap_err());
        assert_eq!(Parser::new(sql).parse().unwrap_err().to_string(), r#"Unexpected end of input"#);

        Ok(())
    }

    #[test]
    fn test_parse_insert() -> Result<()> {
        let mut sql = " insert into users (c1,c2,c3,c4) values (1, 2.3, 'abc', true), (2, 4.5, 'def', false);";
        assert_eq!(Parser::new(sql).parse()?, Statement::Insert {
            table_name: "users".to_string(),
            columns: Some(vec!["c1".to_string(), "c2".to_string(), "c3".to_string(), "c4".to_string()]),
            values: vec![
                vec![
                    Constant::Integer(1).into(),
                    Constant::Float(2.3).into(),
                    Constant::String("abc".to_string()).into(),
                    Constant::Boolean(true).into(),
                ],
                vec![
                    Constant::Integer(2).into(),
                    Constant::Float(4.5).into(),
                    Constant::String("def".to_string()).into(),
                    Constant::Boolean(false).into(),
                ]
            ],
        });

        sql = " insert into users (a, b, c, d) values (1, 2.3, 'abc', true), (2, 4.5, 'def', false);create ";
        assert_eq!(Parser::new(sql).parse().unwrap_err().to_string(), r#"Unexpected token: Keyword(Create)"#);

        Ok(())
    }

    #[test]
    fn test_parse_select() -> Result<()> {
        let mut sql = " select * from users; ";
        assert_eq!(Parser::new(sql).parse()?, Statement::Select {
            table_name: "users".to_string(),
        });
        Ok(())
    }
}
