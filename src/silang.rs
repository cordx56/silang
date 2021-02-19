pub mod parser;
pub mod run;
pub mod builtin;
pub mod define;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum IdentifierType {
    None,
    String,
    Int,
    Float,
    Bool,
    Function,
    TypeName,
}

#[derive(Debug, Clone)]
pub struct UserDefinedFunction {
    scope: usize,
    statement: Statement,
}

#[derive(Clone)]
pub struct IdentifierValue {
    pub identifier_type: IdentifierType,
    pub string: Option<String>,
    pub int: Option<i64>,
    pub float: Option<f64>,
    pub bool: Option<bool>,
    pub user_defined_function: Option<UserDefinedFunction>,
    pub function: Option<fn (&mut Context, Vec<Factor>) -> Result<Vec<Factor>, String>>,
}

pub struct Identifier {
    pub identifier_type: IdentifierType,
    pub name: String,
    pub value: IdentifierValue,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FactorKind {
    Identifier,
    String,
    Int,
    Float,
    Bool,
    Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Factor {
    pub kind: FactorKind,
    pub name: Option<String>,
    pub string: Option<String>,
    pub int: Option<i64>,
    pub float: Option<f64>,
    pub bool: Option<bool>,
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


pub type IdentifierStorage = Vec<HashMap<String, IdentifierValue>>;
pub struct Context {
    pub scope: usize,
    pub identifier_storage: IdentifierStorage,
}

impl Context {
    pub fn push_new(&mut self) {
        self.scope += 1;
        self.identifier_storage.push(HashMap::new());
    }
    pub fn pop(&mut self) {
        self.scope -= 1;
        self.identifier_storage.pop();
    }
}
