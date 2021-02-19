use super::{
    FactorKind,
    Factor,
    Expression,
    Statement,
};
use super::define;

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
        tag,
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
                space0,
                delimited(
                    char('{'),
                    permutation((
                        multispace0,
                        many0(
                            statement,
                        ),
                        multispace0,
                    )),
                    char('}'),
                ),
                multispace0,
            )),
            |(expr, _, (_, stmts, _), _)| -> Statement {
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
                delimited(
                    multispace0,
                    opt(
                        expression,
                    ),
                    multispace0,
                ),
                char(')'),
            ),
            |expr: Option<Expression>| -> Factor {
                match expr {
                    Some(e) => Factor { kind: FactorKind::Expression, name: None, string: None, int: None, float: None, bool: None, expression: Some(e) },
                    None => Factor { kind: FactorKind::Expression, name: None, string: None, int: None, float: None, bool: None, expression: Some(Expression { factors: Vec::new() }) },
                }
            }
        )
    ))(s)
}

pub fn identifier(s: &str) -> IResult<&str, Factor> {
    map(
        is_not(" \t\r\n(){}"),
        |identifier: &str| -> Factor {
            Factor { kind: FactorKind::Identifier, name: Some(identifier.to_owned()), string: None, int: None, float: None, bool: None, expression: None }
        }
    )(s)
}
pub fn number(s: &str) -> IResult<&str, Factor> {
    map(
        double,
        |number: f64| -> Factor {
            Factor { kind: FactorKind::Float, name: None, string: None, int: None, float: Some(number), bool: None, expression: None }
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
            Factor { kind: FactorKind::String, name: None, string: Some(string), int: None, float: None, bool: None, expression: None }
        }
    )(s)
}
/*
pub fn bool(s: &str) -> IResult<&str, Factor> {
    alt((
        value(
            Factor { kind: FactorKind::Bool, name: None, string: None, int: None, float: None, bool: Some(true), expression: None },
            tag(define::TRUE),
        ),
        value(
            Factor { kind: FactorKind::Bool, name: None, string: None, int: None, float: None, bool: Some(false), expression: None },
            tag(define::FALSE),
        ),
    ))(s)
}*/


// Parse tree
fn push_indent(buffer: &mut String, depth: usize) {
    for _ in 0..depth {
        buffer.push_str("    ");
    }
}
pub fn parse_tree(stmts: Vec<Statement>) -> String {
    let mut buffer = String::new();
    for s in stmts {
        buffer.push_str(&parse_tree_statement(s, 0));
    }
    buffer
}
pub fn parse_tree_statement(stmt: Statement, depth: usize) -> String {
    let mut buffer = String::new();
    buffer.push_str(&parse_tree_expression(stmt.expression, depth));
    for s in stmt.statements {
        buffer.push_str(&parse_tree_statement(s, depth + 1));
    }
    buffer
}
pub fn parse_tree_expression(expr: Expression, depth: usize) -> String {
    let mut buffer = String::new();
    for f in expr.factors {
        buffer.push_str(&parse_tree_factor(f, depth));
    }
    buffer
}
pub fn parse_tree_factor(factor: Factor, depth: usize) -> String {
    let mut buffer = String::new();
    push_indent(&mut buffer, depth);
    buffer.push_str("Factor: ");
    if factor.kind == FactorKind::Identifier {
        buffer.push_str("Identifier\n");
        push_indent(&mut buffer, depth);
        buffer.push_str("        ");
        buffer.push_str(&factor.name.unwrap());
        buffer.push_str("\n");
    } else if factor.kind == FactorKind::String {
        buffer.push_str("String\n");
        push_indent(&mut buffer, depth);
        buffer.push_str("        ");
        buffer.push_str(&factor.string.unwrap());
        buffer.push_str("\n");
    } else if factor.kind == FactorKind::Int {
        buffer.push_str("Int\n");
        push_indent(&mut buffer, depth);
        buffer.push_str("        ");
        buffer.push_str(&format!("{}", factor.int.unwrap()));
        buffer.push_str("\n");
    } else if factor.kind == FactorKind::Float {
        buffer.push_str("Float\n");
        push_indent(&mut buffer, depth);
        buffer.push_str("        ");
        buffer.push_str(&format!("{}", factor.float.unwrap()));
        buffer.push_str("\n");
    } else if factor.kind == FactorKind::Expression {
        buffer.push_str("Expression\n");
        buffer.push_str(&parse_tree_expression(factor.expression.unwrap(), depth + 1));
        buffer.push_str("\n");
    }
    buffer
}
