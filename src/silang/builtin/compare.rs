use crate::silang::{
    Interpreter,
    Value,
    EvalReturn,
    EvalResult,
    UserDefinedFunction,
    ScopeType,
    SILType,
};

impl Interpreter {
    pub fn equal_value(&self, lhs: &Value, rhs: &Value) -> bool {
        if lhs.string.is_some() && rhs.string.is_some() {
            lhs.string.as_ref().unwrap() == rhs.string.as_ref().unwrap()
        } else if let Some(lhs_int) = lhs.int {
            if let Some(rhs_int) = rhs.int {
                lhs_int == rhs_int
            } else if let Some(rhs_float) = rhs.float {
                lhs_int as f64 == rhs_float
            } else {
                false
            }
        } else if let Some(lhs_float) = lhs.float {
            if let Some(rhs_int) = rhs.int {
                lhs_float == rhs_int as f64
            } else if let Some(rhs_float) = rhs.float {
                lhs_float == rhs_float
            } else {
                false
            }
        } else {
            false
        }
    }
    pub fn equal(&mut self, args: &[Value]) -> Result<EvalReturn, String> {
        if args.len() < 3 {
            return Err("equal: Argument length must be >2".to_owned())
        }
        let mut retval = Value::new();
        let mut evaluated_args = Vec::new();
        for arg in &args[1..] {
            match self.eval_value(arg, true) {
                Ok(result) => {
                    for v in result.values {
                        evaluated_args.push(v);
                    }
                },
                Err(e) => return Err(e),
            }
        }
        let first_arg = &evaluated_args[0];
        for arg in &evaluated_args[1..] {
            if !self.equal_value(first_arg, arg) {
                retval.bool = Some(false);
                return Ok(
                    EvalReturn {
                        result: EvalResult::Normal,
                        values: vec![retval],
                    }
                )
            }
        }
        retval.bool = Some(true);
        return Ok(
            EvalReturn {
                result: EvalResult::Normal,
                values: vec![retval],
            }
        )
    }
}
