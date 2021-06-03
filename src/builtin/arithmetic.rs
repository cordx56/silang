use crate::silang::{
    Interpreter,
    Value,
    EvalReturn,
    EvalResult,
    SILType,
};
use crate::define;

impl Interpreter {
    pub fn add(&mut self, args: &[Value]) -> Result<EvalReturn, String> {
        let mut accumulator;
        let mut values = Vec::new();
        for value in &args[1..] {
            match self.eval_value(value, true) {
                Ok(result) => {
                    for v in result.values {
                        values.push(v);
                    }
                },
                Err(e) => return Err(e),
            }
        }
        if values.len() < 2 {
            return Err("add: Argument length must be >=2".to_owned())
        }
        accumulator = values[0].clone();
        for value in &values[1..] {
            match self.add_value(&accumulator, value) {
                Ok(v) => accumulator = v,
                Err(e) => return Err(e),
            }
        }
        Ok(EvalReturn {
            result: EvalResult::Normal,
            values: vec![accumulator],
        })
    }
    pub fn add_value(&self, lhs: &Value, rhs: &Value) -> Result<Value, String> {
        let mut retval = Value::new();
        if lhs.string.is_some() && rhs.string.is_some() {
            let mut string = String::new();
            string += lhs.string.as_ref().unwrap();
            string += rhs.string.as_ref().unwrap();
            retval.string = Some(string);
            retval.sil_type = SILType::String;
            Ok(retval)
        } else if lhs.int.is_some() && rhs.int.is_some() {
            retval.int = Some(lhs.int.unwrap() + rhs.int.unwrap());
            retval.sil_type = SILType::Int;
            Ok(retval)
        } else if lhs.float.is_some() && rhs.float.is_some() {
            retval.float = Some(lhs.float.unwrap() + rhs.float.unwrap());
            retval.sil_type = SILType::Float;
            Ok(retval)
        } else if lhs.int.is_some() && rhs.float.is_some() {
            retval.float = Some(lhs.int.unwrap() as f64 + rhs.float.unwrap());
            retval.sil_type = SILType::Float;
            Ok(retval)
        } else if lhs.float.is_some() && rhs.int.is_some() {
            retval.float = Some(lhs.float.unwrap() + rhs.int.unwrap() as f64);
            retval.sil_type = SILType::Float;
            Ok(retval)
        } else {
            Err(format!("add: {}", define::UNSUPPORTED_OPERATION))
        }
    }
    pub fn sub(&mut self, args: &[Value]) -> Result<EvalReturn, String> {
        let mut accumulator;
        let mut values = Vec::new();
        for value in &args[1..] {
            match self.eval_value(value, true) {
                Ok(result) => {
                    for v in result.values {
                        values.push(v);
                    }
                },
                Err(e) => return Err(e),
            }
        }
        if values.len() < 2 {
            return Err("sub: Argument length must be >=2".to_owned())
        }
        accumulator = values[0].clone();
        for value in &values[1..] {
            match self.sub_value(&accumulator, value) {
                Ok(v) => accumulator = v,
                Err(e) => return Err(e),
            }
        }
        Ok(EvalReturn {
            result: EvalResult::Normal,
            values: vec![accumulator],
        })
    }
    pub fn sub_value(&self, lhs: &Value, rhs: &Value) -> Result<Value, String> {
        let mut retval = Value::new();
        if let Some(lhs_int) = lhs.int {
            if let Some(rhs_int) = rhs.int {
                retval.int = Some(lhs_int - rhs_int);
                Ok(retval)
            } else if let Some(rhs_float) = rhs.float {
                retval.float = Some(lhs_int as f64 - rhs_float);
                Ok(retval)
            } else {
                Err(format!("sub: {}", define::UNSUPPORTED_OPERATION))
            }
        } else if let Some(lhs_float) = lhs.float {
            if let Some(rhs_int) = rhs.int {
                retval.float = Some(lhs_float - rhs_int as f64);
                Ok(retval)
            } else if let Some(rhs_float) = rhs.float {
                retval.float = Some(lhs_float - rhs_float);
                Ok(retval)
            } else {
                Err(format!("sub: {}", define::UNSUPPORTED_OPERATION))
            }
        } else {
            Err(format!("sub: {}", define::UNSUPPORTED_OPERATION))
        }
    }
    pub fn mul(&mut self, args: &[Value]) -> Result<EvalReturn, String> {
        let mut accumulator;
        let mut values = Vec::new();
        for value in &args[1..] {
            match self.eval_value(value, true) {
                Ok(result) => {
                    for v in result.values {
                        values.push(v);
                    }
                },
                Err(e) => return Err(e),
            }
        }
        if values.len() < 2 {
            return Err("mul: Argument length must be >=2".to_owned())
        }
        accumulator = values[0].clone();
        for value in &values[1..] {
            match self.mul_value(&accumulator, value) {
                Ok(v) => accumulator = v,
                Err(e) => return Err(e),
            }
        }
        Ok(EvalReturn {
            result: EvalResult::Normal,
            values: vec![accumulator],
        })
    }
    pub fn mul_value(&self, lhs: &Value, rhs: &Value) -> Result<Value, String> {
        let mut retval = Value::new();
        if let Some(lhs_int) = lhs.int {
            if let Some(rhs_int) = rhs.int {
                retval.int = Some(lhs_int * rhs_int);
                Ok(retval)
            } else if let Some(rhs_float) = rhs.float {
                retval.float = Some(lhs_int as f64 * rhs_float);
                Ok(retval)
            } else {
                Err(format!("mul: {}", define::UNSUPPORTED_OPERATION))
            }
        } else if let Some(lhs_float) = lhs.float {
            if let Some(rhs_int) = rhs.int {
                retval.float = Some(lhs_float * rhs_int as f64);
                Ok(retval)
            } else if let Some(rhs_float) = rhs.float {
                retval.float = Some(lhs_float * rhs_float);
                Ok(retval)
            } else {
                Err(format!("mul: {}", define::UNSUPPORTED_OPERATION))
            }
        } else {
            Err(format!("mul: {}", define::UNSUPPORTED_OPERATION))
        }
    }
    pub fn div(&mut self, args: &[Value]) -> Result<EvalReturn, String> {
        let mut accumulator;
        let mut values = Vec::new();
        for value in &args[1..] {
            match self.eval_value(value, true) {
                Ok(result) => {
                    for v in result.values {
                        values.push(v);
                    }
                },
                Err(e) => return Err(e),
            }
        }
        if values.len() < 2 {
            return Err("div: Argument length must be >=2".to_owned())
        }
        accumulator = values[0].clone();
        for value in &values[1..] {
            match self.div_value(&accumulator, value) {
                Ok(v) => accumulator = v,
                Err(e) => return Err(e),
            }
        }
        Ok(EvalReturn {
            result: EvalResult::Normal,
            values: vec![accumulator],
        })
    }
    pub fn div_value(&self, lhs: &Value, rhs: &Value) -> Result<Value, String> {
        let mut retval = Value::new();
        if let Some(lhs_int) = lhs.int {
            if let Some(rhs_int) = rhs.int {
                retval.int = Some(lhs_int / rhs_int);
                Ok(retval)
            } else if let Some(rhs_float) = rhs.float {
                retval.float = Some(lhs_int as f64 / rhs_float);
                Ok(retval)
            } else {
                Err(format!("mul: {}", define::UNSUPPORTED_OPERATION))
            }
        } else if let Some(lhs_float) = lhs.float {
            if let Some(rhs_int) = rhs.int {
                retval.float = Some(lhs_float / rhs_int as f64);
                Ok(retval)
            } else if let Some(rhs_float) = rhs.float {
                retval.float = Some(lhs_float / rhs_float);
                Ok(retval)
            } else {
                Err(format!("mul: {}", define::UNSUPPORTED_OPERATION))
            }
        } else {
            Err(format!("mul: {}", define::UNSUPPORTED_OPERATION))
        }
    }
    pub fn rem(&mut self, args: &[Value]) -> Result<EvalReturn, String> {
        let mut values = Vec::new();
        for value in &args[1..] {
            match self.eval_value(value, true) {
                Ok(result) => {
                    for v in result.values {
                        values.push(v);
                    }
                },
                Err(e) => return Err(e),
            }
        }
        if values.len() != 2 {
            return Err("rem: Argument length must be 2".to_owned())
        }
        match self.rem_value(&values[0], &values[1]) {
            Ok(result) => Ok(
                EvalReturn {
                    result: EvalResult::Normal,
                    values: vec![result],
                }
            ),
            Err(e) => Err(e),
        }
    }
    pub fn rem_value(&self, lhs: &Value, rhs: &Value) -> Result<Value, String> {
        let mut retval = Value::new();
        if let Some(lhs_int) = lhs.int {
            if let Some(rhs_int) = rhs.int {
                retval.int = Some(lhs_int % rhs_int);
                Ok(retval)
            } else if let Some(rhs_float) = rhs.float {
                retval.float = Some(lhs_int as f64 % rhs_float);
                Ok(retval)
            } else {
                Err(format!("rem: {}", define::UNSUPPORTED_OPERATION))
            }
        } else if let Some(lhs_float) = lhs.float {
            if let Some(rhs_int) = rhs.int {
                retval.float = Some(lhs_float % rhs_int as f64);
                Ok(retval)
            } else if let Some(rhs_float) = rhs.float {
                retval.float = Some(lhs_float % rhs_float);
                Ok(retval)
            } else {
                Err(format!("rem: {}", define::UNSUPPORTED_OPERATION))
            }
        } else {
            Err(format!("rem: {}", define::UNSUPPORTED_OPERATION))
        }
    }
}
