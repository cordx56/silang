use crate::parser;
use crate::run;
use crate::define;

use std::collections::HashMap;

#[cfg(any(target_family = "unix", target_family = "windows"))]
pub struct Interpreter {
    pub context: Context,
    pub version: &'static str,
    pub libraries: Vec<libloading::Library>,
    pub stdout_func: fn (&Interpreter, &str),
}
#[cfg(target_family = "wasm")]
pub struct Interpreter {
    pub context: Context,
    pub version: &'static str,
    pub stdout_buffer: String,
    pub stdout_func: fn (&Interpreter, &str),
}

impl Interpreter {
    #[cfg(any(target_family = "unix", target_family = "windows"))]
    pub fn new() -> Self {
        Interpreter {
            context: Context::new(),
            version: define::VERSION,
            libraries: Vec::new(),
            stdout_func: |_, data| print!("{}", data),
        }
    }
    #[cfg(target_family = "wasm")]
    pub fn new() -> Self {
        Interpreter {
            context: Context::new(),
            version: define::VERSION,
            stdout_func: |interpreter, data| interpreter.stdout_buffer.push_str(data),
        }
    }

    pub fn version(&self) -> &str {
        self.version
    }

    #[cfg(target_family = "wasm")]
    pub fn buffer_flush(&self) -> String {
        let tmp = self.stdout_buffer;
        self.stdout_buffer = String::new();
        tmp
    }

    pub fn factor_to_value(&self, factor: &parser::Factor) -> Value {
        let mut value = Value::new();
        if factor.identifier.is_some() {
            match self.context.search_identifier_id(factor.identifier.as_ref().unwrap()) {
                Some(id) => {
                    value.identifier = Some(factor.identifier.as_ref().unwrap().clone());
                    value.identifier_id = Some(id.1);
                    value
                },
                None => {
                    value.identifier = Some(factor.identifier.as_ref().unwrap().clone());
                    value
                },
            }
        } else if factor.string.is_some() {
            value.sil_type = SILType::String;
            value.string = Some(factor.string.as_ref().unwrap().clone());
            value
        } else if factor.int.is_some() {
            value.sil_type = SILType::Int;
            value.int = Some(factor.int.unwrap());
            value
        } else if factor.float.is_some() {
            value.sil_type = SILType::Float;
            value.float = Some(factor.float.unwrap());
            value
        } else if factor.expression.is_some() {
            value.expression = Some(self.parser_expr_to_run_expr(factor.expression.as_ref().unwrap()));
            value
        } else if factor.block.is_some() {
            value.block = Some(factor.block.as_ref().unwrap().clone());
            value
        } else {
            value
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EvalResult {
    Normal,
    Return,
    Break,
}

#[derive(Debug, Clone)]
pub struct EvalReturn {
    pub result: EvalResult,
    pub values: Vec<Value>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SILType {
    String,
    Int,
    Float,
    Bool,
    Vector,
    Map,
    TypeName,
    Void,
    Any,
}


#[derive(Debug, Clone)]
pub struct UserDefinedFunction {
    pub scope: Vec<ScopeInfo>,
    pub args: run::Expression,
    pub block: parser::Block,
}

pub type IdentifierRefID = usize;
#[derive(Clone)]
pub struct Value {
    pub sil_type: SILType,
    pub identifier: Option<String>,
    pub identifier_id: Option<IdentifierRefID>,
    pub string: Option<String>,
    pub int: Option<i64>,
    pub float: Option<f64>,
    pub bool: Option<bool>,
    pub vector: Option<Vec<Value>>,
    pub map: Option<HashMap<String, Value>>,
    pub expression: Option<run::Expression>,
    pub block: Option<parser::Block>,
    pub user_defined_function: Option<UserDefinedFunction>,
    pub function: Option<fn (&mut Interpreter, &[Value]) -> Result<EvalReturn, String>>,
}
impl Value {
    pub fn new() -> Self {
        Value {
            sil_type: SILType::Any,
            identifier: None,
            identifier_id: None,
            string: None,
            int: None,
            float: None,
            bool: None,
            vector: None,
            map: None,
            expression: None,
            block: None,
            user_defined_function: None,
            function: None,
        }
    }
    pub fn is_reference(&self) -> bool {
        self.identifier_id.is_some()
    }
}
impl std::fmt::Debug for Value {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            fmt,
            "Value {{ {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?} }}",
            self.sil_type,
            self.identifier,
            self.identifier_id,
            self.string,
            self.int,
            self.float,
            self.bool,
            self.vector,
            self.map,
            self.expression,
            self.block,
            self.user_defined_function.is_some(),
            self.function.is_some(),
        )?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ScopeType {
    Root,
    Program,
    Block,
    UserDefinedFunction,
    If,
    Loop,
    UnTyped,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ScopeInfo {
    pub scope_number: usize,
    pub scope_type: ScopeType,
}

pub struct IdentifierStorage {
    pub freed: Vec<IdentifierRefID>,
    pub storage: Vec<Value>,
}
pub type IdentifierIndex = Vec<HashMap<String, IdentifierRefID>>;

pub struct Context {
    pub scope: Vec<ScopeInfo>,
    pub untyped_scopes: Vec<usize>,
    pub identifier_storage: IdentifierStorage,
    pub identifier_index: IdentifierIndex,
}

impl Context {
    pub fn push_new(&mut self, scope_type: ScopeType, is_untyped: bool) {
        self.scope.push(
            ScopeInfo {
                scope_number: self.identifier_index.len(),
                scope_type: scope_type,
            }
        );
        if is_untyped {
            self.untyped_scopes.push(self.identifier_index.len());
        }
        self.identifier_index.push(HashMap::new());
    }
    pub fn pop(&mut self) {
        match self.scope.pop() {
            Some(popped) => {
                if popped.scope_type == ScopeType::UnTyped {
                    self.untyped_scopes.pop();
                }
            },
            None => {},
        }
        self.identifier_index.pop();
    }
    pub fn current_scope(&self) -> ScopeInfo {
        self.scope[self.scope.len() - 1].clone()
    }
    /// Returns scope and reference to Value
    /// # Arguments
    /// - `name` - A string slice index
    pub fn search_identifier(&self, name: &str) -> Option<(usize, &Value)> {
        match self.search_identifier_id(name) {
            Some(id) => Some((id.0, &self.identifier_storage.storage[id.1])),
            None => None,
        }
    }
    pub fn search_identifier_id(&self, name: &str) -> Option<(usize, IdentifierRefID)> {
        if self.scope.is_empty() {
            return None
        }
        let mut n = self.scope.len() -1;
        loop {
            let scope = self.scope[n].scope_number;
            if self.identifier_index[scope].contains_key(name) {
                return Some((scope, self.identifier_index[scope][name]))
            }
            if n == 0 {
                return None
            }
            n -= 1;
        }
    }
    pub fn get_value_from_identifier_id(&self, id: IdentifierRefID) -> &Value {
        &self.identifier_storage.storage[id]
    }
    pub fn set_value_from_identifier_id(&mut self, id: IdentifierRefID, value: Value) {
        self.identifier_storage.storage[id] = value;
    }
    pub fn store_value(&mut self, value: Value) -> IdentifierRefID {
        match self.identifier_storage.freed.pop() {
            Some(index) => {
                self.identifier_storage.storage[index] = value;
                index
            },
            None => {
                self.identifier_storage.storage.push(value);
                self.identifier_storage.storage.len() - 1
            },
        }
    }
    pub fn store_identifier(&mut self, scope: usize, name: &str, value: Value) -> IdentifierRefID {
        let id = self.store_value(value);
        self.identifier_index[scope].insert(name.to_string(), id);
        id
    }
    pub fn is_declared(&self, scope: usize, name: &str) -> bool {
        self.identifier_index[scope].contains_key(name)
    }

    pub fn is_untyped(&self) -> bool {
        let current_scope = self.current_scope().scope_number;
        self.untyped_scopes.iter().find(|&&x| x == current_scope).is_some()
    }


    pub fn new() -> Self {
        let is = IdentifierStorage {
            freed: Vec::new(),
            storage: Vec::new(),
        };
        let mut ctx = Context {
            scope: vec![ScopeInfo { scope_number: 0, scope_type: ScopeType::Root }],
            untyped_scopes: vec![],
            identifier_storage: is,
            identifier_index: Vec::new(),
        };
        ctx.init_identifier_storage();
        ctx.push_new(ScopeType::Program, false);
        ctx
    }
    fn init_identifier_storage(&mut self) {
        self.identifier_index.push(HashMap::new());

        // Functions
        let mut import = Value::new();
        import.identifier = Some(define::IMPORT.to_owned());
        import.function = Some(Interpreter::import);
        self.store_identifier(0, define::IMPORT, import);
        let mut lambda = Value::new();
        lambda.identifier = Some(define::LAMBDA.to_owned());
        lambda.function = Some(Interpreter::lambda);
        self.store_identifier(0, define::LAMBDA, lambda);
        let mut return_expr = Value::new();
        return_expr.identifier = Some(define::RETURN.to_owned());
        return_expr.function = Some(Interpreter::return_expression);
        self.store_identifier(0, define::RETURN, return_expr);
        // control
        let mut untyped = Value::new();
        untyped.identifier = Some(define::UNTYPED.to_owned());
        untyped.function = Some(Interpreter::untyped);
        self.store_identifier(0, define::UNTYPED, untyped);
        let mut if_expr = Value::new();
        if_expr.identifier = Some(define::IF.to_owned());
        if_expr.function = Some(Interpreter::if_expression);
        self.store_identifier(0, define::IF, if_expr);
        let mut loop_expr = Value::new();
        loop_expr.identifier = Some(define::LOOP.to_owned());
        loop_expr.function = Some(Interpreter::loop_expression);
        self.store_identifier(0, define::LOOP, loop_expr);
        // Declare
        let mut decas = Value::new();
        decas.identifier = Some(define::DECAS.to_owned());
        decas.function = Some(Interpreter::decas);
        self.store_identifier(0, define::DECAS, decas);
        let mut decas_alias = Value::new();
        decas_alias.identifier = Some(define::DECAS_ALIAS.to_owned());
        decas_alias.function = Some(Interpreter::decas);
        self.store_identifier(0, define::DECAS_ALIAS, decas_alias);
        let mut func_def = Value::new();
        func_def.identifier = Some(define::FUNCTION_DEFINITION.to_owned());
        func_def.function = Some(Interpreter::define_function);
        self.store_identifier(0, define::FUNCTION_DEFINITION, func_def);
        // Assign
        let mut assign = Value::new();
        assign.identifier = Some(define::ASSIGN.to_owned());
        assign.function = Some(Interpreter::assign);
        self.store_identifier(0, define::ASSIGN, assign);
        let mut assign_defer = Value::new();
        assign_defer.identifier = Some(define::ASSIGN_DEFER.to_owned());
        assign_defer.function = Some(Interpreter::assign_defer);
        self.store_identifier(0, define::ASSIGN_DEFER, assign_defer);
        // Print
        let mut print = Value::new();
        print.identifier = Some(define::PRINT.to_owned());
        print.function = Some(Interpreter::print);
        self.store_identifier(0, define::PRINT, print);
        let mut println = Value::new();
        println.identifier = Some(define::PRINTLN.to_owned());
        println.function = Some(Interpreter::println);
        self.store_identifier(0, define::PRINTLN, println);
        // Arithmetic
        let mut add = Value::new();
        add.identifier = Some(define::ADD.to_owned());
        add.function = Some(Interpreter::add);
        self.store_identifier(0, define::ADD, add);
        let mut sub = Value::new();
        sub.identifier = Some(define::SUB.to_owned());
        sub.function = Some(Interpreter::sub);
        self.store_identifier(0, define::SUB, sub);
        let mut mul = Value::new();
        mul.identifier = Some(define::MUL.to_owned());
        mul.function = Some(Interpreter::mul);
        self.store_identifier(0, define::MUL, mul);
        let mut div = Value::new();
        div.identifier = Some(define::DIV.to_owned());
        div.function = Some(Interpreter::div);
        self.store_identifier(0, define::DIV, div);
        let mut rem = Value::new();
        rem.identifier = Some(define::REM.to_owned());
        rem.function = Some(Interpreter::rem);
        self.store_identifier(0, define::REM, rem);
        // Compare
        let mut equal = Value::new();
        equal.identifier = Some(define::EQUAL.to_owned());
        equal.function = Some(Interpreter::equal);
        self.store_identifier(0, define::EQUAL, equal);


        // Type name
        let mut string = Value::new();
        string.identifier = Some(define::STRING.to_owned());
        string.sil_type = SILType::TypeName;
        self.store_identifier(0, define::STRING, string);
        let mut int = Value::new();
        int.identifier = Some(define::INT.to_owned());
        int.sil_type = SILType::TypeName;
        self.store_identifier(0, define::INT, int);
        let mut float = Value::new();
        float.identifier = Some(define::FLOAT.to_owned());
        float.sil_type = SILType::TypeName;
        self.store_identifier(0, define::FLOAT, float);
        let mut bool_type = Value::new();
        bool_type.identifier = Some(define::BOOL.to_owned());
        bool_type.sil_type = SILType::TypeName;
        self.store_identifier(0, define::BOOL, bool_type);
        let mut void = Value::new();
        void.identifier = Some(define::VOID.to_owned());
        void.sil_type = SILType::TypeName;
        self.store_identifier(0, define::VOID, void);

        let mut true_value = Value::new();
        true_value.identifier = Some(define::TRUE.to_owned());
        true_value.bool = Some(true);
        true_value.sil_type = SILType::Bool;
        self.store_identifier(0, define::TRUE, true_value);
        let mut false_value = Value::new();
        false_value.identifier = Some(define::FALSE.to_owned());
        false_value.bool = Some(false);
        false_value.sil_type = SILType::Bool;
        self.store_identifier(0, define::FALSE, false_value);
    }
}
