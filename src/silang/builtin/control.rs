use crate::silang::{
    Interpreter,
    Value,
    EvalReturn,
    EvalResult,
    UserDefinedFunction,
    ScopeType,
};

impl Interpreter {
    pub fn untyped(&mut self, args: &[Value]) -> Result<EvalReturn, String> {
        if args.len() != 2 {
            return Err("untyped: Argument length must be 1".to_owned())
        }
        if args[1].block.is_none() {
            return Err("untyped: Argument 1 must be block".to_owned())
        }
        self.context.push_new(ScopeType::UnTyped, true);
        let result = self.exec_block(args[1].block.as_ref().unwrap());
        self.context.pop();
        result
    }

    pub fn if_expression(&mut self, args: &[Value]) -> Result<EvalReturn, String> {
        if args.len() < 3 || 4 < args.len() {
            return Err("if: Argument length must be 2-3".to_owned())
        }
        if args[1].expression.is_none() {
            return Err("if: Argument 1 must be expression".to_owned())
        }
        match self.eval_value(&args[1], true) {
            Ok(result) => {
                if result.values.len() != 1 || result.values[0].bool.is_none() {
                    return Err("if: Argument 1 must be single bool value".to_owned())
                }
                if result.values[0].bool.unwrap() {
                    match self.eval_value(&args[2], true) {
                        Ok(result) => Ok(result),
                        Err(e) => return Err(e),
                    }
                    /*
                    if let Some(expr) = &args[2].expression {
                        match self.eval(&expr, true) {
                            Ok(result) => Ok(
                                EvalReturn {
                                    result: EvalResult::Normal,
                                    values: result.values,
                                }
                            ),
                            Err(e) => return Err(e),
                        }
                    } else if let Some(block) = &args[2].block {
                        self.context.push_new(ScopeType::If, false);
                        match self.exec_block(&block) {
                            Ok(result) => {
                                self.context.pop();
                                Ok(
                                    EvalReturn {
                                        result: EvalResult::Normal,
                                        values: result.values,
                                    }
                                )
                            },
                            Err(e) => return Err(e),
                        }
                    } else {
                        Ok(
                            EvalReturn {
                                result: EvalResult::Normal,
                                values: vec![args[2].clone()],
                            }
                        )
                    }*/
                } else {
                    if args.len() == 4 {
                        match self.eval_value(&args[3], true) {
                            Ok(result) => Ok(result),
                            Err(e) => return Err(e),
                        }
                    } else {
                        Ok(
                            EvalReturn {
                                result: EvalResult::Normal,
                                values: vec![],
                            }
                        )
                    }
                }
            },
            Err(e) => return Err(e),
        }
    }
}
