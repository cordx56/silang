use super::{
    Interpreter,
    Value,
    EvalResult,
    EvalReturn,
    ScopeType,
};

use super::parser;

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

    pub fn eval_value(&mut self, value: &Value, dereference: bool) -> Result<EvalReturn, String> {
        if let Ok(v) = self.dereference_value(value) {
            if dereference {
                self.eval_value(&v, dereference)
            } else {
                Ok(
                    EvalReturn {
                        result: EvalResult::Normal,
                        values: vec![value.clone()]
                    }
                )
            }
        } else if let Some(expr) = &value.expression {
            self.eval(&expr, dereference)
        } else if let Some(block) = &value.block {
            self.context.push_new(ScopeType::Block, false);
            let result = self.exec_block(&block);
            self.context.pop();
            result
        } else {
            Ok(EvalReturn {
                result: EvalResult::Normal,
                values: vec![value.clone()],
            })
        }
    }
    pub fn eval(&mut self, expr: &Expression, dereference: bool) -> Result<EvalReturn, String> {
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
        match self.eval_value(first_value, true) {
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
            self.context.push_new(ScopeType::UserDefinedFunction, false);
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
                match self.eval_value(v, dereference) {
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
        self.eval(&expression, true)
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
                    if result.result == EvalResult::Return {
                        if self.context.current_scope().scope_type == ScopeType::UserDefinedFunction {
                            result.result = EvalResult::Normal;
                        }
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
