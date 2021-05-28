use crate::silang::{
    Interpreter,
    Value,
    EvalReturn,
    EvalResult,
    SILType,
};
use crate::silang::define;

impl Interpreter {
    pub fn decas(&mut self, args: &[Value]) -> Result<EvalReturn, String> {
        if args.len() != 3 {
            return Err("decas: Argument length must be 2".to_owned())
        }
        let lhs = &args[1];
        let rhs = &args[2];
        let lhs_evaluated;
        let rhs_evaluated;
        match self.eval_value(lhs) {
            Ok(result) => lhs_evaluated = result,
            Err(e) => return Err(e),
        }
        match self.eval_value(rhs) {
            Ok(result) => rhs_evaluated = result,
            Err(e) => return Err(e),
        }
        if lhs_evaluated.values.len() != rhs_evaluated.values.len() {
            return Err("decas: LHS and RHS length must be equal".to_owned())
        }
        let current_scope = self.context.current_scope();
        let mut retval = Vec::new();
        for i in 0..lhs_evaluated.values.len() {
            if lhs_evaluated.values[i].identifier.is_none() {
                return Err("decas: LHS must be identifier".to_owned())
            }
            if rhs_evaluated.values[i].sil_type != SILType::TypeName || rhs_evaluated.values[i].identifier.is_none() {
                return Err("decas: RHS must be type name".to_owned())
            }
            let type_name = rhs_evaluated.values[i].identifier.as_ref().unwrap();
            let mut v = lhs_evaluated.values[i].clone();
            if type_name == define::STRING {
                v.sil_type = SILType::String;
                v.string = Some("".to_owned());
            } else if type_name == define::INT {
                v.sil_type = SILType::Int;
                v.int = Some(0);
            } else if type_name == define::FLOAT {
                v.sil_type = SILType::Float;
                v.float = Some(0.0);
            } else if type_name == define::BOOL {
                v.sil_type = SILType::Bool;
                v.bool = Some(false);
            } else if type_name == define::VOID {
                v.sil_type = SILType::Void;
            }
            let mut val = Value::new();
            val.identifier_id = Some(self.context.store_identifier(current_scope, v.identifier.as_ref().unwrap(), v.clone()));
            retval.push(val);
        }
        Ok(
            EvalReturn {
                result: EvalResult::Normal,
                values: retval,
            }
        )
    }

    pub fn assign(&mut self, args: &[Value]) -> Result<EvalReturn, String> {
        if args.len() != 3 {
            return Err("assign: Argument length must be 2".to_owned())
        }
        let lhs = &args[1];
        let rhs = &args[2];

        match self.assign_variable(lhs, rhs, true) {
            Ok(values) => {
                Ok(
                    EvalReturn {
                        result: EvalResult::Normal,
                        values: values,
                    }
                )
            },
            Err(e) => return Err(e),
        }
    }
    pub fn assign_defer(&mut self, args: &[Value]) -> Result<EvalReturn, String> {
        if args.len() != 3 {
            return Err("assign: Argument length must be 2".to_owned())
        }
        let lhs = &args[1];
        let rhs = &args[2];

        match self.assign_variable(lhs, rhs, false) {
            Ok(values) => {
                Ok(
                    EvalReturn {
                        result: EvalResult::Normal,
                        values: values,
                    }
                )
            },
            Err(e) => return Err(e),
        }
    }
    pub fn assign_variable(&mut self, lhs: &Value, rhs: &Value, evaluate_rhs: bool) -> Result<Vec<Value>, String> {
        eprintln!("LHS: {:?}", lhs);
        if let Some(id) = lhs.identifier_id {
            if evaluate_rhs {
                match self.eval_value(rhs) {
                    Ok(result) => {
                        let values = result.values;
                        if values.len() != 1 {
                            return Err("assign: LHS and RHS length must be equal".to_owned())
                        }
                        self.context.set_value_from_identifier_id(id, values[0].clone());
                    },
                    Err(e) => return Err(e),
                }
            } else {
                self.context.set_value_from_identifier_id(id, rhs.clone());
            }
            let mut value = Value::new();
            value.identifier_id = Some(id);
            Ok(vec![value])
        } else if let Some(lhs_expr) = &lhs.expression {
            let mut retval = Vec::new();
            if let Some(rhs_expr) = &rhs.expression {
                if lhs_expr.values.len() != rhs_expr.values.len() {
                    return Err("assign: LHS and RHS length must be equal".to_owned())
                }
                for i in 0..lhs_expr.values.len() {
                    //eprintln!("LHS: {:?}", lhs_expr.values[i]);
                    //eprintln!("RHS: {:?}", rhs_expr.values[i]);
                    match self.assign_variable(&lhs_expr.values[i], &rhs_expr.values[i], evaluate_rhs) {
                        Ok(ids) => {
                            for id in ids {
                                retval.push(id);
                            }
                        },
                        Err(e) => return Err(e),
                    }
                }
            }
            Ok(retval)
        } else {
            if !self.context.is_untyped() {
                return Err("assign: LHS must be declared identifier".to_owned())
            }
            if lhs.identifier.is_none() {
                return Err("assign: LHS must be identifier".to_owned())
            }
            let current_scope = self.context.current_scope();
            let id = self.context.store_identifier(current_scope, lhs.identifier.as_ref().unwrap(), rhs.clone());
            let mut value = Value::new();
            value.identifier_id = Some(id);
            Ok(vec![value])
        }
    }
}
