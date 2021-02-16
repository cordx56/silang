use super::{
    FactorKind,
    Factor,
    Expression,
    Statement,
};

extern crate nom;

use nom::{
    IResult,
    character::complete::{
        space0,
        space1,
        multispace0,
        none_of,
        char,
        line_ending,
    },
    bytes::complete::{
        is_not,
        escaped_transform,
        take_while_m_n,
    },
    number::complete::{
        double,
    },
    branch::{
        alt,
        permutation,
    },
    combinator::{
        opt,
        map,
        value,
        all_consuming,
    },
    multi::{
        many0,
        many1,
    },
    sequence::delimited,
};
use std::char::{
    decode_utf16,
    REPLACEMENT_CHARACTER,
};
use std::u16;

pub fn program_all_consuming(s: &str) -> IResult<&str, Vec<Statement>> {
    all_consuming(program)(s)
}
pub fn program(s: &str) -> IResult<&str, Vec<Statement>> {
    many1(
        delimited(
            multispace0,
            statement,
            multispace0,
        )
    )(s)
}

pub fn statement_all_consuming(s: &str) -> IResult<&str, Statement> {
    all_consuming(statement)(s)
}
pub fn statement(s: &str) -> IResult<&str, Statement> {
    alt((
        map(
            permutation((
                multispace0,
                expression,
                space0,
                line_ending,
            )),
            |expr: (&str, Expression, &str, &str)| -> Statement {
                Statement { expression: expr.1, statements: Vec::new() }
            }
        ),
        map(
            permutation((
                opt(
                    permutation((
                        multispace0,
                        expression,
                    ))
                ),
                multispace0,
                delimited(
                    char('{'),
                    many0(
                        statement,
                    ),
                    char('}'),
                ),
                multispace0,
            )),
            |(expr, _, stmts, _)| -> Statement {
                match expr {
                    Some(e) => Statement { expression: e.1, statements: stmts },
                    None => Statement { expression: Expression { factors: Vec::new() }, statements: stmts },
                }
            }
        )
    ))(s)
}

pub fn expression_all_consuming(s: &str) -> IResult<&str, Expression> {
    all_consuming(expression)(s)
}
pub fn expression(s: &str) -> IResult<&str, Expression> {
    map(
        permutation((
            factor,
            many0(
                permutation((
                    space1,
                    factor,
                ))
            ),
        )),
        |(factor, factors): (Factor, Vec<(&str, Factor)>)| -> Expression {
            let mut factorvec = Vec::new();
            factorvec.push(factor);
            for f in factors {
                factorvec.push(f.1);
            }
            Expression { factors: factorvec }
        }
    )(s)
}

pub fn factor(s: &str) -> IResult<&str, Factor> {
    alt((
        string,
        number,
        identifier,
        map(
            delimited(
                char('('),
                opt(
                    delimited(
                        multispace0,
                        expression,
                        multispace0,
                    )
                ),
                char(')'),
            ),
            |expr: Option<Expression>| -> Factor {
                match expr {
                    Some(e) => Factor { kind: FactorKind::Expression, name: None, string: None, int: None, float: None, expression: Some(e) },
                    None => Factor { kind: FactorKind::Expression, name: None, string: None, int: None, float: None, expression: Some(Expression { factors: Vec::new() }) },
                }
            }
        )
    ))(s)
}

pub fn identifier(s: &str) -> IResult<&str, Factor> {
    map(
        is_not(" \t\r\n(){}"),
        |identifier: &str| -> Factor {
            Factor { kind: FactorKind::Identifier, name: Some(identifier.to_owned()), string: None, int: None, float: None, expression: None }
        }
    )(s)
}
pub fn number(s: &str) -> IResult<&str, Factor> {
    map(
        double,
        |number: f64| -> Factor {
            Factor { kind: FactorKind::Float, name: None, string: None, int: None, float: Some(number), expression: None }
        }
    )(s)
}
pub fn string(s: &str) -> IResult<&str, Factor> {
    map(
        delimited(
            char('"'),
            escaped_transform(none_of("\"\\"), '\\', alt((
                value('\\', char('\\')),
                value('\"', char('\"')),
                value('\'', char('\'')),
                value('\r', char('r')),
                value('\n', char('n')),
                value('\t', char('t')),
                map(
                    permutation((char('u'), take_while_m_n(4, 4, |c: char| c.is_ascii_hexdigit()))),
                    |(_, code): (char, &str)| -> char {
                        decode_utf16(vec![u16::from_str_radix(code, 16).unwrap()]).nth(0).unwrap().unwrap_or(REPLACEMENT_CHARACTER)
                    },
                )
            ))),
            char('"'),
        ),
        |string: String| -> Factor {
            Factor { kind: FactorKind::String, name: None, string: Some(string), int: None, float: None, expression: None }
        }
    )(s)
}
