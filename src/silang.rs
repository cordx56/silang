pub mod parser;
pub mod run;
pub mod builtin;
pub mod define;

use std::collections::HashMap;

#[derive(Clone)]
pub struct UserDefinedFunction {
    scope: Vec<usize>,
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

    pub fn format(&self) -> String {
        format!(
            "Factor {{ kind: {:?}, name: {:?}, string: {:?}, int: {:?}, float: {:?}, bool: {:?}, expression: {:?}, user_defined_function: {:?}, function: {:?} }}",
            self.kind,
            self.name,
            self.string,
            self.int,
            self.float,
            self.bool,
            self.expression.is_some(),
            self.user_defined_function.is_some(),
            self.function.is_some(),
        )
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
    pub params: Vec<Factor>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ScopeType {
    Root,
    Program,
    Block,
    UserDefinedFunction,
    If,
    Loop,
}

pub type IdentifierStorage = Vec<HashMap<String, Factor>>;
pub struct Context {
    pub scope: Vec<usize>,
    pub scope_type: Vec<ScopeType>,
    pub identifier_storage: IdentifierStorage,
}

impl Context {
    pub fn push_new(&mut self, scope_type: ScopeType) {
        self.scope_type.push(scope_type);
        self.scope.push(self.identifier_storage.len());
        self.identifier_storage.push(HashMap::new());
    }
    pub fn pop(&mut self) {
        self.scope_type.pop();
        self.scope.pop();
        self.identifier_storage.pop();
    }
    pub fn current_scope(&mut self) -> usize {
        self.scope[self.scope.len() - 1]
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExecResult {
    Normal,
    IfTrue,
    IfFalse,
    LoopFalse,
    LoopBreak,
    LoopContinue,
    UserDefinedFunctionDefinition,
    Return,
}
pub struct ExecReturn {
    pub result: ExecResult,
    pub factors: Vec<Factor>,
}
