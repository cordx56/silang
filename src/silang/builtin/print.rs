use crate::silang::{
    Interpreter,
    Value,
    EvalReturn,
    EvalResult,
};

impl Interpreter {
    pub fn print_value(&mut self, value: &Value) -> Result<Vec<Value>, String> {
        let mut retval = Vec::new();
        if value.expression.is_some() {
            match self.eval_value(value, true) {
                Ok(result) => {
                    if 1 < result.values.len() {
                        print!("(");
                    }
                    for i in 0..result.values.len() {
                        match self.print_value(&result.values[i]) {
                            Ok(values) => {
                                for v in values {
                                    retval.push(v);
                                }
                            },
                            Err(e) => return Err(e),
                        }
                        if i != result.values.len() - 1 {
                            print!(" ");
                        }
                    }
                    if 1 < result.values.len() {
                        print!(")");
                    }
                },
                Err(e) => return Err(e),
            }
        } else if value.identifier_id.is_some() {
            match self.dereference_value(value) {
                Ok(v) => {
                    match self.print_value(&v) {
                        Ok(values) => {
                            for v in values {
                                retval.push(v);
                            }
                        },
                        Err(e) => return Err(e),
                    }
                },
                Err(e) => return Err(e),
            }
        } else if let Some(string) = &value.string {
            print!("{}", string);
            retval.push(value.clone());
        } else if let Some(int) = value.int {
            print!("{}", int);
            retval.push(value.clone());
        } else if let Some(float) = value.float {
            print!("{}", float);
            retval.push(value.clone());
        } else if let Some(bool_val) = value.bool {
            if bool_val {
                print!("true");
            } else {
                print!("false");
            }
            retval.push(value.clone());
        } else {
            return Err("print: undefined value".to_owned())
        }
        Ok(retval)
    }
    pub fn print(&mut self, args: &[Value]) -> Result<EvalReturn, String> {
        let mut retval = Vec::new();
        for v in &args[1..] {
            match self.print_value(v) {
                Ok(values) => {
                    for v in values {
                        retval.push(v);
                    }
                },
                Err(e) => return Err(e),
            }
        }
        Ok(
            EvalReturn {
                result: EvalResult::Normal,
                values: retval,
            }
        )
    }
    pub fn println(&mut self, args: &[Value]) -> Result<EvalReturn, String> {
        let mut retval = Vec::new();
        for v in &args[1..] {
            match self.print_value(v) {
                Ok(values) => {
                    for v in values {
                        retval.push(v);
                    }
                },
                Err(e) => return Err(e),
            }
        }
        println!("");
        Ok(
            EvalReturn {
                result: EvalResult::Normal,
                values: retval,
            }
        )
    }
}
