use crate::silang::{
    Interpreter,
    Value,
    EvalReturn,
    EvalResult,
};

impl Interpreter {
    pub fn print_value(&mut self, value: &Value, evaluate: bool) -> Result<(), String> {
        if evaluate {
            match self.eval_value(value) {
                Ok(result) => {
                    if 1 < result.values.len() {
                        println!("(");
                    }
                    for i in 0..result.values.len() {
                        match self.print_value(&result.values[i], false) {
                            Ok(_) => {},
                            Err(e) => return Err(e),
                        }
                        if i != result.values.len() - 1 {
                            println!(" ");
                        }
                    }
                    if 1 < result.values.len() {
                        println!(")");
                    }
                },
                Err(e) => return Err(e),
            }
        } else if let Some(_) = value.identifier_id {
            match self.dereference_value(value) {
                Ok(v) => {
                    match self.print_value(&v, false) {
                        Ok(_) => {},
                        Err(e) => return Err(e),
                    }
                },
                Err(e) => return Err(e),
            }
        } else if let Some(string) = &value.string {
            print!("{}", string);
        } else if let Some(int) = value.int {
            print!("{}", int);
        } else if let Some(float) = value.float {
            print!("{}", float);
        } else if let Some(bool_val) = value.bool {
            if bool_val {
                print!("true");
            } else {
                print!("false");
            }
        } else {
            return Err("print: undefined value".to_owned())
        }
        Ok(())
    }
    pub fn print(&mut self, args: &[Value]) -> Result<EvalReturn, String> {
        for v in &args[1..] {
            match self.print_value(v, true) {
                Ok(_) => {},
                Err(e) => return Err(e),
            }
        }
        Ok(
            EvalReturn {
                result: EvalResult::Normal,
                values: args.to_vec(),
            }
        )
    }
    pub fn println(&mut self, args: &[Value]) -> Result<EvalReturn, String> {
        for v in &args[1..] {
            match self.print_value(v, true) {
                Ok(_) => {},
                Err(e) => return Err(e),
            }
        }
        println!("");
        Ok(
            EvalReturn {
                result: EvalResult::Normal,
                values: args.to_vec(),
            }
        )
    }
}
