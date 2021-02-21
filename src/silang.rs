pub mod parser;
pub mod run;
pub mod builtin;
pub mod define;

use std::collections::HashMap;

#[derive(Clone)]
pub struct UserDefinedFunction {
    scope: usize,
    statement: Statement,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FactorKind {
    None,
    Identifier,
    String,
    Int,
    Float,
    Bool,
    Function,
    TypeName,
    Vector,
    Map,
    Expression,
}

#[derive(Clone)]
pub struct Factor {
    pub kind: FactorKind,
    pub name: Option<String>,
    pub string: Option<String>,
    pub int: Option<i64>,
    pub float: Option<f64>,
    pub bool: Option<bool>,
    pub vector: Option<Vec<Factor>>,
    pub map: Option<HashMap<String, Factor>>,
    pub expression: Option<Expression>,
    pub user_defined_function: Option<UserDefinedFunction>,
    pub function: Option<fn (&mut Context, Vec<Factor>) -> Result<Vec<Factor>, String>>,
}

impl Factor {
    pub fn new() -> Factor {
        Factor {
            kind: FactorKind::None,
            name: None,
            string: None,
            int: None,
            float: None,
            bool: None,
            vector: None,
            map: None,
            expression: None,
            user_defined_function: None,
            function: None,
        }
    }
}

#[derive(Clone)]
pub struct Expression {
    pub factors: Vec<Factor>,
}

#[derive(Clone)]
pub struct Statement {
    pub expression: Expression,
    pub statements: Vec<Statement>,
}


pub type IdentifierStorage = Vec<HashMap<String, Factor>>;
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
