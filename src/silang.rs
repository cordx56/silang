pub mod parser;
pub mod run;
pub mod builtin;

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum IdentifierType {
    None,
    String,
    Int,
    Float,
    Function,
    TypeName,
}

#[derive(Debug)]
pub struct UserDefinedFunction {
    scope: usize,
    statement: Statement,
}

#[derive(Debug)]
pub struct IdentifierValue<'a> {
    pub identifier_type: IdentifierType,
    pub string: Option<String>,
    pub int: Option<i64>,
    pub float: Option<f64>,
    pub user_defined_function: Option<UserDefinedFunction>,
    pub function: Option<fn (&'a mut Context<'a>, Vec<Factor>) -> Result<Vec<Factor>, &str>>,
}

pub struct Identifier<'a> {
    pub identifier_type: IdentifierType,
    pub name: String,
    pub value: IdentifierValue<'a>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FactorKind {
    Identifier,
    String,
    Int,
    Float,
    Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Factor {
    pub kind: FactorKind,
    pub name: Option<String>,
    pub string: Option<String>,
    pub int: Option<i64>,
    pub float: Option<f64>,
    pub expression: Option<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Expression {
    pub factors: Vec<Factor>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Statement {
    pub expression: Expression,
    pub statements: Vec<Statement>,
}


pub type IdentifierStorage<'a> = Vec<HashMap<String, IdentifierValue<'a>>>;
pub struct Context<'a> {
    pub scope: usize,
    pub identifier_storage: &'a mut IdentifierStorage<'a>,
}
