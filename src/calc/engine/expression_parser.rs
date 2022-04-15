use logos::{Lexer, Logos};
use crate::calc::engine::{CellRef, Expression, Value};

#[derive(Logos, Debug, PartialEq, Clone)]
enum Token {
    #[token("(")]
    Open,
    #[token(")")]
    Close,

    #[token(":")]
    Colon,

    #[token(",")]
    Comma,

    #[regex(r"\$?[A-Z]+\$?\d+")]
    Reference,

    #[regex("[a-zA-Z]+")]
    Identifier,

    #[regex(r"\d+(\.\d*)")]
    Number,
    #[regex(r"(TRUE)|(FALSE)")]
    Boolean,

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}


pub fn parse(content: &str) -> Expression {
    let mut lex = Token::lexer(content);
    parse_expression(&mut lex)
}

fn parse_expression(lex: &mut Lexer<Token>) -> Expression {
    match lex.next() {
        Some(t) => {
            match t {
                Token::Identifier => parse_call(lex),
                Token::Reference => parse_reference(lex),
                _ => Expression::Literal(Value::Error("EOF"))
            }
        }
        None => Expression::Literal(Value::Error("EOF"))
    }
}

fn parse_reference(lex: &mut Lexer<Token>) -> Expression {
    let start = CellRef::parse(lex.slice());
    if !matches!(peek(lex), Some(Token::Colon)){
        return Expression::Reference(start);
    }
    lex.next();
    if !matches!(peek(lex), Some(Token::Reference)){
        return Expression::Literal(Value::Error("NULL!"));
    }
    lex.next();
    let end = CellRef::parse(lex.slice());
    return Expression::Range(start, end);
}

fn peek(lex: &mut Lexer<Token>) -> Option<Token> {
    lex.clone().next()
}

fn parse_call(lex: &mut Lexer<Token>) -> Expression {
    let name = String::from(lex.slice()).to_uppercase();
    let mut args = Vec::new();

    if !matches!(lex.next(), Some(Token::Open)) {
        return Expression::Literal(Value::Error("NAME"))
    }

    loop {
        match peek(lex) {
            Some(t) => {
                match t {
                    Token::Comma => { lex.next(); },
                    Token::Close => {
                        lex.next();
                        return Expression::Call(name, args);
                    },
                    _ => args.push(parse_expression(lex))
                }
            }
            None => return Expression::Literal(Value::Error("EOF"))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::calc::engine::{Expression};
    use crate::calc::engine::expression_parser::parse;

    #[test]
    fn reference() {
        let parsed = parse("B2");
        assert!(matches!(parsed, Expression::Reference(_)), "{:?}", parsed)
    }

    #[test]
    fn ranges() {
        let parsed = parse("B2:B3");
        assert!(matches!(parsed, Expression::Range(_, _)), "{:?}", parsed)
    }
}