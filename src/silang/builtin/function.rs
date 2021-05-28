use crate::silang::{
    Interpreter,
    Value,
    EvalReturn,
    EvalResult,
    UserDefinedFunction,
};

impl Interpreter {
    //pub fn define_function(&mut self, args: &[Value]) -> Result<EvalReturn, String> {
    //    if args.len() != 4 {
    //        return Err("Argument length must be 3".to_owned())
    //    }
    //    let identifier = args[1];
    //}

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
