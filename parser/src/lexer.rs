use anyhow::{bail, Result};
use std::iter::Peekable;
use std::str::{Chars, FromStr};
use crate::token::{Keyword, Symbol, Token};

/// support sql:
/// 1.
/// ```sql
/// CREATE TABLE table_name (
/// [ column_name data_type[column_constraint [...]]
/// [, ...]
/// );
/// where data_type is:
///     - BOOLEAN(BOOL) : true false
///     - FLOAT(DOUBLE)
///     - INTEGER(INT)
///     - STRING(TEXT,VARCHAR)
///
/// where column_constraint is:
/// [NOT NULL | NULL | DEFAULT expr]
/// ```
///
/// 2.
/// ```sql
/// INSERT INTO table_name (column1, column2,...)
/// VALUES (value1, value2,...)
/// ```
///
/// 3.
/// ```sql
/// SELECT * FROM table_name;
/// ```
pub struct Lexer<'a> {
    inner: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: input.chars().peekable(),
        }
    }


    fn scan(&mut self) -> Result<Option<Token>> {
        while self.inner.next_if(|c| c.is_whitespace()).is_some() {}

        Ok(match self.inner.peek() {
            Some('\'') => self.scan_string(),
            Some(c) if c.is_ascii_digit() => self.scan_number(),
            Some(c) if c.is_alphabetic() => self.scan_keyword_or_ident(),
            Some(c) if c.is_ascii_punctuation() => self.scan_symbol(),
            _ => bail!("Unexpected EOF"),
        })
    }

    // 'xxx' -> xxx
    fn scan_string(&mut self) -> Option<Token> {
        if self.inner.next_if(|&c| c == '\'').is_none() {
            return None;
        }

        let mut val = String::new();

        loop {
            match self.inner.next()? {
                '\'' => break,
                c => val.push(c),
            }
        }

        Some(Token::String(val))
    }

    // 1.23
    fn scan_number(&mut self) -> Option<Token> {
        let mut num = String::new();

        while let Some(c) = self.inner.next_if(|&c| c.is_numeric()) {
            num.push(c);
        }

        if let Some(sep) = self.inner.next_if(|&c| c == '.') {
            num.push(sep);

            while let Some(c) = self.inner.next_if(|&c| c.is_numeric()) {
                num.push(c);
            }
        }

        Some(Token::Number(num))
    }

    // tbl_name true
    fn scan_keyword_or_ident(&mut self) -> Option<Token> {
        let mut val = String::new();

        while let Some(c) = self.inner.next_if(|&c| c.is_alphabetic()) {
            val.push(c);
        }

        while let Some(c) = self.inner.next_if(|&c| c.is_alphanumeric()) {
            val.push(c);
        }

        Some(Keyword::from_str(&val).map_or(Token::Ident(val), |k| Token::Keyword(k)))
    }

    fn scan_symbol(&mut self) -> Option<Token> {
        let val = self.inner.peek()?;

        let symbol = Symbol::try_from(val).ok()?;

        self.inner.next();

        Some(Token::Symbol(symbol))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.scan().ok()?
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let input = "CREATE TABLE tbl (
            id1 INT PRIMARY KEY,
            id21 INTEGER,
            c1 BOOL NULL,
            c2 FLOAT NOT NULL,
            c3 VARCHAR DEFAULT 'abc'
        );";

        let tokens = Lexer::new(input)
            .peekable().collect::<Vec<_>>();

        assert_eq!(tokens, vec![
            Token::Keyword(Keyword::Create),
            Token::Keyword(Keyword::Table),
            Token::Ident("tbl".to_string()),
            Token::Symbol(Symbol::OpenParen),
            Token::Ident("id1".to_string()),
            Token::Keyword(Keyword::Int),
            Token::Keyword(Keyword::Primary),
            Token::Keyword(Keyword::Key),
            Token::Symbol(Symbol::Comma),
            Token::Ident("id21".to_string()),
            Token::Keyword(Keyword::Integer),
            Token::Symbol(Symbol::Comma),
            Token::Ident("c1".to_string()),
            Token::Keyword(Keyword::Bool),
            Token::Keyword(Keyword::Null),
            Token::Symbol(Symbol::Comma),
            Token::Ident("c2".to_string()),
            Token::Keyword(Keyword::Float),
            Token::Keyword(Keyword::Not),
            Token::Keyword(Keyword::Null),
            Token::Symbol(Symbol::Comma),
            Token::Ident("c3".to_string()),
            Token::Keyword(Keyword::Varchar),
            Token::Keyword(Keyword::Default),
            Token::String("abc".to_string()),
            Token::Symbol(Symbol::CloseParen),
            Token::Symbol(Symbol::Semicolon),
        ])
    }

    #[test]
    fn test_insert() {
        let input = "INSERT INTO tbl (id1, id2, c1, c2, c3) VALUES (1, 2, true, 3.14, 'abc');";

        let tokens = Lexer::new(input)
            .peekable().collect::<Vec<_>>();

        assert_eq!(tokens, vec![
            Token::Keyword(Keyword::Insert),
            Token::Keyword(Keyword::Into),
            Token::Ident("tbl".to_string()),
            Token::Symbol(Symbol::OpenParen),
            Token::Ident("id1".to_string()),
            Token::Symbol(Symbol::Comma),
            Token::Ident("id2".to_string()),
            Token::Symbol(Symbol::Comma),
            Token::Ident("c1".to_string()),
            Token::Symbol(Symbol::Comma),
            Token::Ident("c2".to_string()),
            Token::Symbol(Symbol::Comma),
            Token::Ident("c3".to_string()),
            Token::Symbol(Symbol::CloseParen),
            Token::Keyword(Keyword::Values),
            Token::Symbol(Symbol::OpenParen),
            Token::Number("1".to_string()),
            Token::Symbol(Symbol::Comma),
            Token::Number("2".to_string()),
            Token::Symbol(Symbol::Comma),
            Token::Keyword(Keyword::True),
            Token::Symbol(Symbol::Comma),
            Token::Number("3.14".to_string()),
            Token::Symbol(Symbol::Comma),
            Token::String("abc".to_string()),
            Token::Symbol(Symbol::CloseParen),
            Token::Symbol(Symbol::Semicolon),
        ])
    }

    #[test]
    fn test_select() {
        let tokens1 = Lexer::new("select * from tbl;")
            .peekable()
            .collect::<Vec<_>>();

        assert_eq!(
            tokens1,
            vec![
                Token::Keyword(Keyword::Select),
                Token::Symbol(Symbol::Asterisk),
                Token::Keyword(Keyword::From),
                Token::Ident("tbl".to_string()),
                Token::Symbol(Symbol::Semicolon),
            ]
        );
    }
}