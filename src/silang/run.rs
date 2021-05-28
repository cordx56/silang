use super::{
    UserDefinedFunction,
    FactorKind,

    ScopeType,
    IdentifierStorage,
    Context,
    ExecResult,
    ExecReturn,

    Interpreter,
    Value,
    EvalResult,
    EvalReturn,
};

use super::parser;
use super::builtin;
use super::define;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Expression {
    pub values: Vec<Value>,
}

impl Interpreter {
    pub fn dereference_value(&self, value: &Value) -> Result<Value, String> {
        if value.is_reference() {
            Ok(self.context.get_value_from_identifier_id(value.identifier_id.unwrap()).clone())
        } else {
            Err("Invalid dereference".to_owned())
        }
    }
    pub fn parser_expr_to_run_expr(&self, expr: &parser::Expression) -> Expression {
        let factors = &expr.factors;
        let mut expression = Expression { values: Vec::new() };
        for f in factors {
            expression.values.push(self.factor_to_value(&f));
        }
        expression
    }

    pub fn eval_value(&mut self, value: &Value) -> Result<EvalReturn, String> {
        if let Ok(v) = self.dereference_value(value) {
            self.eval_value(&v)
        } else if let Some(expr) = &value.expression {
            self.eval(&expr)
        } else if let Some(block) = &value.block {
            self.exec_block(&block)
        } else {
            Ok(EvalReturn {
                result: EvalResult::Normal,
                values: vec![value.clone()],
            })
        }
    }
    pub fn eval(&mut self, expr: &Expression) -> Result<EvalReturn, String> {
        let mut values = Vec::new();
        if expr.values.len() == 0 {
            return Ok(
                EvalReturn {
                    result: EvalResult::Normal,
                    values: values,
                }
            )
        }
        let first_value = &expr.values[0];
        match self.eval_value(first_value) {
            Ok(res) => values = res.values,
            Err(e) => return Err(e),
        }
        if values.len() == 0 {
            return Ok(
                EvalReturn {
                    result: EvalResult::Normal,
                    values: values,
                }
            )
        }

        if let Some(func) = values[0].function {
            for v in &expr.values[1..] {
                values.push(v.clone())
            }

            func(self, &values)
        } else if let Some(udf) = values[0].user_defined_function.clone() {
            let mut args = Vec::new();
            for v in &expr.values[1..] {
                args.push(v.clone())
            }

            let backup_scope = self.context.scope.clone();
            self.context.scope = udf.scope.clone();
            self.context.push_new(ScopeType::UserDefinedFunction);
            let mut args_lhs = Value::new();
            args_lhs.expression = Some(udf.args);
            let mut args_rhs = Value::new();
            args_rhs.expression = Some(Expression { values: args });
            match self.assign_variable(&args_lhs, &args_rhs, false) {
                Ok(_) => {},
                Err(e) => return Err(e),
            }
            let res = self.exec_block(&udf.block);
            self.context.pop();
            self.context.scope = backup_scope;
            return res
        } else {
            for v in &expr.values[1..] {
                match self.eval_value(v) {
                    Ok(r) => {
                        for i in r.values {
                            values.push(i);
                        }
                    },
                    Err(e) => return Err(e),
                }
            }

            return Ok(
                EvalReturn {
                    result: EvalResult::Normal,
                    values: values,
                }
            )
        }
    }


    pub fn exec(&mut self, statement: &parser::Statement) -> Result<EvalReturn, String> {
        let expression = self.parser_expr_to_run_expr(&statement.expression);
        self.eval(&expression)
    }
    pub fn exec_block(&mut self, block: &parser::Block) -> Result<EvalReturn, String> {
        self.run(&block.program)
    }

    pub fn run(&mut self, program: &parser::Program) -> Result<EvalReturn, String> {
        let mut result = EvalReturn {
            result: EvalResult::Normal,
            values: Vec::new(),
        };
        for s in &program.statements {
            match self.exec(&s) {
                Ok(r) => {
                    result = r;
                    if result.result == EvalResult::Return || result.result == EvalResult::Break {
                        break;
                    }
                },
                Err(e) => {
                    return Err(e)
                },
            }
        }
        Ok(result)
    }
}
