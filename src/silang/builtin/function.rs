use crate::silang::{
    Interpreter,
    Value,
    EvalReturn,
    EvalResult,
    UserDefinedFunction,
};

impl Interpreter {
    pub fn define_function(&mut self, args: &[Value]) -> Result<EvalReturn, String> {
        let mut retval;
        let mut storeval = Value::new();
        let udf_args;
        if let Some(expr) = &args[2].expression {
            udf_args = expr.clone();
        } else {
            return Err("f: Argument 2 must be expression".to_owned())
        }
        let udf_block;
        if self.context.is_untyped() {
            if args.len() == 5 {
                if let Some(block) = &args[4].block {
                    udf_block = block.clone();
                } else {
                    return Err("f: Argument 4 must be block".to_owned())
                }
            } else if args.len() == 4 {
                if let Some(block) = &args[3].block {
                    udf_block = block.clone();
                } else {
                    return Err("f: Argument 3 must be block".to_owned())
                }
            } else {
                return Err("f: Argument length must be 3-4".to_owned())
            }
        } else {
            if args.len() != 5 {
                return Err("f: Argument length must be 4".to_owned())
            }
            if let Some(block) = &args[4].block {
                udf_block = block.clone();
            } else {
                return Err("f: Argument 4 must be block".to_owned())
            }
        }
        if args.len() == 5 {
            match self.get_type_from_identifier(&args[3]) {
                Ok(type_value) => {
                    storeval.sil_type = type_value;
                },
                Err(e) => return Err(e),
            }
        }
        storeval.user_defined_function = Some(
            UserDefinedFunction {
                scope: self.context.scope.clone(),
                args: udf_args,
                block: udf_block,
            }
        );
        retval = args[1].clone();
        if retval.identifier.is_none() {
            return Err("f: Argument 1 must be identifier".to_owned())
        }

        storeval.identifier = Some(retval.identifier.as_ref().unwrap().clone());
        storeval.sil_type = storeval.sil_type.clone();
        let current_scope = self.context.current_scope();
        retval.identifier_id = Some(self.context.store_identifier(current_scope, retval.identifier.as_ref().unwrap(), storeval.clone()));
        match self.assign_variable(&retval, &storeval, false) {
            Ok(_) => {},
            Err(e) => return Err(e),
        }
        Ok(
            EvalReturn {
                result: EvalResult::Normal,
                values: vec![retval],
            }
        )
    }

    pub fn return_expression(&mut self, args: &[Value]) -> Result<EvalReturn, String> {
        let return_values = &args[1..];
        let mut evaluated_return_values = Vec::new();
        for r in return_values {
            match self.eval_value(r, true) {
                Ok(v) => {
                    for i in v.values {
                        evaluated_return_values.push(i);
                    }
                },
                Err(e) => return Err(e),
            }
        }
        Ok(
            EvalReturn {
                result: EvalResult::Return,
                values: evaluated_return_values,
            }
        )
    }

    pub fn lambda(&mut self, args: &[Value]) -> Result<EvalReturn, String> {
        if args.len() != 3 {
            return Err("lambda: Argument length must be 2".to_owned())
        }
        if args[1].expression.is_none() {
            return Err("lambda: Argument 1 must be expression".to_owned())
        }
        if args[2].block.is_none() {
            return Err("lambda: Argument 2 must be block".to_owned())
        }
        let mut value = Value::new();
        value.user_defined_function = Some(
            UserDefinedFunction {
                scope: self.context.scope.clone(),
                args: args[1].expression.as_ref().unwrap().clone(),
                block: args[2].block.as_ref().unwrap().clone(),
            }
        );
        Ok(
            EvalReturn {
                result: EvalResult::Normal,
                values: vec![value],
            }
        )
    }
}
