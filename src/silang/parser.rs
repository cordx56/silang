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
    error::VerboseError,
};
use std::char::{
    decode_utf16,
    REPLACEMENT_CHARACTER,
};
use std::u16;

#[derive(Debug, PartialEq, Clone)]
pub struct Factor {
    pub identifier: Option<String>,
    pub string: Option<String>,
    pub int: Option<i64>,
    pub float: Option<f64>,
    pub expression: Option<Expression>,
    pub block: Option<Block>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Expression {
    pub factors: Vec<Factor>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Statement {
    pub expression: Expression,
    //pub params: Vec<Factor>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub program: Program,
}


pub fn program_all_consuming(s: &str) -> IResult<&str, Program, VerboseError<&str>> {
    all_consuming(program)(s)
}
pub fn program(s: &str) -> IResult<&str, Program, VerboseError<&str>> {
    map(
        many1(
            delimited(
                multispace0,
                statement,
                multispace0,
            )
        ),
        |statements| {
            Program { statements: statements }
        }
    )(s)
}

pub fn block(s: &str) -> IResult<&str, Block, VerboseError<&str>> {
    map(
        delimited(
            tag(define::BLOCK_OPEN),
            delimited(
                multispace0,
                program,
                multispace0,
            ),
            tag(define::BLOCK_CLOSE),
        ),
        |program| {
            Block { program: program }
        }
    )(s)
}

pub fn statement_all_consuming(s: &str) -> IResult<&str, Statement, VerboseError<&str>> {
    all_consuming(statement)(s)
}
pub fn statement(s: &str) -> IResult<&str, Statement, VerboseError<&str>> {
    map(
        permutation((
            multispace0,
            expression,
            space0,
            line_ending,
        )),
        |expr| {
            Statement{ expression: expr.1 }
        }
    )(s)
}

pub fn expression_all_consuming(s: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    all_consuming(expression)(s)
}
pub fn expression(s: &str) -> IResult<&str, Expression, VerboseError<&str>> {
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

pub fn factor(s: &str) -> IResult<&str, Factor, VerboseError<&str>> {
    alt((
        string,
        number,
        map(
            permutation((
                identifier,
                opt(
                    delimited(
                        tag(define::INDEX_OPEN),
                        permutation((
                            space0,
                            expression,
                            space0,
                        )),
                        tag(define::INDEX_CLOSE),
                    )
                ),
            )),
            |(mut identifier, expr)| -> Factor {
                if expr.is_some() {
                    identifier.expression = Some(expr.unwrap().1);
                }
                identifier
            }
        ),
        map(
            delimited(
                tag(define::EXPRESSION_OPEN),
                delimited(
                    multispace0,
                    opt(
                        expression,
                    ),
                    multispace0,
                ),
                tag(define::EXPRESSION_CLOSE),
            ),
            |expr: Option<Expression>| -> Factor {
                match expr {
                    Some(e) => Factor {
                        identifier: None,
                        string: None,
                        int: None,
                        float: None,
                        expression: Some(e),
                        block: None
                    },
                    None => Factor {
                        identifier: None,
                        string: None,
                        int: None,
                        float: None,
                        expression: Some(Expression { factors: Vec::new() }),
                        block: None,
                    },
                }
            }
        ),
        map(
            block,
            |block| {
                Factor {
                    identifier: None,
                    string: None,
                    int: None,
                    float: None,
                    expression: None,
                    block: Some(block),
                }
            }
        ),
    ))(s)
}

pub fn identifier(s: &str) -> IResult<&str, Factor, VerboseError<&str>> {
    map(
        is_not(define::PARSER_NOT_IDENTIFIER),
        |identifier: &str| -> Factor {
            Factor { identifier: Some(identifier.to_owned()), string: None, int: None, float: None, expression: None, block: None }
        }
    )(s)
}
pub fn number(s: &str) -> IResult<&str, Factor, VerboseError<&str>> {
    map(
        double,
        |number: f64| -> Factor {
            Factor { identifier: None, string: None, int: None, float: Some(number), expression: None, block: None }
        }
    )(s)
}
pub fn string(s: &str) -> IResult<&str, Factor, VerboseError<&str>> {
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
            Factor { identifier: None, string: Some(string), int: None, float: None, expression: None, block: None }
        }
    )(s)
}


// Parse tree
fn push_indent(buffer: &mut String, depth: usize) {
    for _ in 0..depth {
        buffer.push_str("    ");
    }
}
pub fn parse_tree_program(program: &Program, depth: usize) -> String {
    let mut buffer = String::new();
    for s in &program.statements {
        buffer.push_str(&parse_tree_statement(s, depth));
    }
    buffer
}
pub fn parse_tree_statement(stmt: &Statement, depth: usize) -> String {
    let mut buffer = String::new();
    push_indent(&mut buffer, depth);
    buffer.push_str("Statement: \n");
    buffer.push_str(&parse_tree_expression(&stmt.expression, depth));
    /*for s in stmt.statements {
        buffer.push_str(&parse_tree_statement(s, depth + 1));
    }*/
    buffer
}
pub fn parse_tree_expression(expr: &Expression, depth: usize) -> String {
    let mut buffer = String::new();
    push_indent(&mut buffer, depth);
    buffer.push_str("Expression: \n");
    for f in &expr.factors {
        buffer.push_str(&parse_tree_factor(f, depth + 1));
    }
    buffer
}
pub fn parse_tree_block(block: &Block, depth: usize) -> String {
    let mut buffer = String::new();
    push_indent(&mut buffer, depth);
    buffer.push_str("Block: \n");
    buffer.push_str(&parse_tree_program(&block.program, depth + 1));
    buffer
}
pub fn parse_tree_factor(factor: &Factor, depth: usize) -> String {
    let mut buffer = String::new();
    push_indent(&mut buffer, depth);
    buffer.push_str("Factor: ");
    if let Some(identifier) = &factor.identifier {
        buffer.push_str("Identifier\n");
        push_indent(&mut buffer, depth);
        buffer.push_str("        ");
        buffer.push_str(&identifier);
        buffer.push_str("\n");
    } else if let Some(string) = &factor.string {
        buffer.push_str("String\n");
        push_indent(&mut buffer, depth);
        buffer.push_str("        \"");
        buffer.push_str(&string);
        buffer.push_str("\"\n");
    } else if let Some(int) = factor.int {
        buffer.push_str("Int\n");
        push_indent(&mut buffer, depth);
        buffer.push_str("        ");
        buffer.push_str(&format!("{}", int));
        buffer.push_str("\n");
    } else if let Some(float) = factor.float {
        buffer.push_str("Float\n");
        push_indent(&mut buffer, depth);
        buffer.push_str("        ");
        buffer.push_str(&format!("{}", float));
        buffer.push_str("\n");
    } else if let Some(expression) = &factor.expression {
        buffer.push_str("Expression\n");
        buffer.push_str(&parse_tree_expression(expression, depth + 1));
    } else if let Some(block) = &factor.block {
        buffer.push_str("Block\n");
        buffer.push_str(&parse_tree_block(block, depth + 1))
    }
    buffer
}
