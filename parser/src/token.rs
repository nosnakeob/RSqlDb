use std::str::FromStr;
use anyhow::bail;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Keyword {
    Create,
    Table,
    Int,
    Integer,
    Boolean,
    Bool,
    String,
    Text,
    Varchar,
    Float,
    Double,
    Select,
    From,
    Insert,
    Into,
    Values,
    True,
    False,
    Default,
    Not,
    Null,
    Primary,
    Key,
}

impl FromStr for Keyword {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let keyword = match s.to_uppercase().as_str() {
            "CREATE" => Keyword::Create,
            "TABLE" => Keyword::Table,
            "INT" => Keyword::Int,
            "INTEGER" => Keyword::Integer,
            "BOOLEAN" => Keyword::Boolean,
            "BOOL" => Keyword::Bool,
            "STRING" => Keyword::String,
            "TEXT" => Keyword::Text,
            "VARCHAR" => Keyword::Varchar,
            "FLOAT" => Keyword::Float,
            "DOUBLE" => Keyword::Double,
            "SELECT" => Keyword::Select,
            "FROM" => Keyword::From,
            "INSERT" => Keyword::Insert,
            "INTO" => Keyword::Into,
            "VALUES" => Keyword::Values,
            "TRUE" => Keyword::True,
            "FALSE" => Keyword::False,
            "DEFAULT" => Keyword::Default,
            "NOT" => Keyword::Not,
            "NULL" => Keyword::Null,
            "PRIMARY" => Keyword::Primary,
            "KEY" => Keyword::Key,
            _ => bail!("Unknown keyword: {}", s),
        };

        Ok(keyword)
    }
}

#[derive(Debug, PartialEq)]
pub enum Symbol {
    //左括号(
    OpenParen,
    //右括号)
    CloseParen,
    //逗号,
    Comma,
    //分号;
    Semicolon,
    //星号
    Asterisk,
    // 加号+
    Plus,
    //减号-
    Minus,
    // 斜杠/
    Slash,
}

impl TryFrom<char> for Symbol {
    type Error = anyhow::Error;

    fn try_from(c: char) -> anyhow::Result<Self> {
        let symbol = match c {
            '(' => Symbol::OpenParen,
            ')' => Symbol::CloseParen,
            ',' => Symbol::Comma,
            ';' => Symbol::Semicolon,
            '*' => Symbol::Asterisk,
            '+' => Symbol::Plus,
            '-' => Symbol::Minus,
            '/' => Symbol::Slash,
            _ => bail!("Unknown symbol: {}", c),
        };

        Ok(symbol)
    }
}

impl TryFrom<&char> for Symbol {
    type Error = anyhow::Error;

    fn try_from(c: &char) -> anyhow::Result<Self> {
        let symbol = match c {
            '(' => Symbol::OpenParen,
            ')' => Symbol::CloseParen,
            ',' => Symbol::Comma,
            ';' => Symbol::Semicolon,
            '*' => Symbol::Asterisk,
            '+' => Symbol::Plus,
            '-' => Symbol::Minus,
            '/' => Symbol::Slash,
            _ => bail!("Unknown symbol: {}", c),
        };

        Ok(symbol)
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    // 标识符 表名、列名
    Ident(String),
    String(String),
    Number(String),
    Symbol(Symbol),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str2keyword() {
        assert_eq!(Keyword::from_str("CREATE").unwrap(), Keyword::Create);
        assert_eq!(Keyword::from_str("TABLE").unwrap(), Keyword::Table);
        assert_eq!(Keyword::from_str("INT").unwrap(), Keyword::Int);
        assert_eq!(Keyword::from_str("INTEGER").unwrap(), Keyword::Integer);
        assert_eq!(Keyword::from_str("BOOLEAN").unwrap(), Keyword::Boolean);
        assert_eq!(Keyword::from_str("BOOL").unwrap(), Keyword::Bool);
        assert_eq!(Keyword::from_str("STRING").unwrap(), Keyword::String);
        assert_eq!(Keyword::from_str("TEXT").unwrap(), Keyword::Text);
        assert_eq!(Keyword::from_str("VARCHAR").unwrap(), Keyword::Varchar);
        assert_eq!(Keyword::from_str("FLOAT").unwrap(), Keyword::Float);
        assert_eq!(Keyword::from_str("DOUBLE").unwrap(), Keyword::Double);
        assert_eq!(Keyword::from_str("SELECT").unwrap(), Keyword::Select);
        assert_eq!(Keyword::from_str("FROM").unwrap(), Keyword::From);
        assert_eq!(Keyword::from_str("INSERT").unwrap(), Keyword::Insert);
        assert_eq!(Keyword::from_str("INTO").unwrap(), Keyword::Into);
        assert_eq!(Keyword::from_str("VALUES").unwrap(), Keyword::Values);
        assert_eq!(Keyword::from_str("TRUE").unwrap(), Keyword::True);
        assert_eq!(Keyword::from_str("FALSE").unwrap(), Keyword::False);
        assert_eq!(Keyword::from_str("DEFAULT").unwrap(), Keyword::Default);
        assert_eq!(Keyword::from_str("NOT").unwrap(), Keyword::Not);
        assert_eq!(Keyword::from_str("NULL").unwrap(), Keyword::Null);
        assert_eq!(Keyword::from_str("PRIMARY").unwrap(), Keyword::Primary);
        assert_eq!(Keyword::from_str("KEY").unwrap(), Keyword::Key);

        assert!(Keyword::from_str("KEY1").is_err());
    }

    #[test]
    fn test_char2symbol() {
        assert_eq!(Symbol::try_from('(').unwrap(), Symbol::OpenParen);
        assert_eq!(Symbol::try_from(')').unwrap(), Symbol::CloseParen);
        assert_eq!(Symbol::try_from(',').unwrap(), Symbol::Comma);
        assert_eq!(Symbol::try_from(';').unwrap(), Symbol::Semicolon);
        assert_eq!(Symbol::try_from('*').unwrap(), Symbol::Asterisk);
        assert_eq!(Symbol::try_from('+').unwrap(), Symbol::Plus);
        assert_eq!(Symbol::try_from('-').unwrap(), Symbol::Minus);
        assert_eq!(Symbol::try_from('/').unwrap(), Symbol::Slash);

        assert!(Symbol::try_from('=').is_err());
    }
}
