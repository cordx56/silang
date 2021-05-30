use crate::silang::{
    Interpreter,
    Value,
    EvalReturn,
    EvalResult,
    SILType,
};

impl Interpreter {
    pub fn decas(&mut self, args: &[Value]) -> Result<EvalReturn, String> {
        if args.len() != 3 {
            return Err("decas: Argument length must be 2".to_owned())
        }
        let lhs = &args[1];
        let rhs = &args[2];
        let lhs_evaluated;
        let rhs_evaluated;
        match self.eval_value(lhs, false) {
            Ok(result) => lhs_evaluated = result,
            Err(e) => return Err(e),
        }
        match self.eval_value(rhs, true) {
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
            
            let mut v = Value::new();
            v.identifier = lhs_evaluated.values[i].identifier.clone();
            match self.get_type_from_identifier(&rhs_evaluated.values[i]) {
                Ok(type_value) => {
                    if type_value == SILType::String {
                        v.string = Some("".to_owned());
                    } else if type_value == SILType::Int {
                        v.int = Some(0);
                    } else if type_value == SILType::Float {
                        v.float = Some(0.0);
                    } else if type_value == SILType::Bool {
                        v.bool = Some(false);
                    }
                    v.sil_type = type_value;
                }
                Err(e) => return Err(e),
            }
            let mut val = Value::new();
            let identifier = v.identifier.as_ref().unwrap();
            if self.context.is_declared(current_scope.scope_number, identifier) {
                return Err(format!("decas: {} is already declared", identifier))
            }
            val.sil_type = v.sil_type.clone();
            val.identifier_id = Some(self.context.store_identifier(current_scope.scope_number, identifier, v.clone()));
            //eprintln!("Declare {} as {:?}", identifier, val);
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
        let lhs_values;
        match self.eval_value(lhs, false) {
            Ok(result) => lhs_values = result.values,
            Err(e) => return Err(e),
        }
        let mut lhs_instances = Vec::new();
        for lhs_v in &lhs_values {
            match self.eval_value(&lhs_v, true) {
                Ok(result) => {
                    for r in result.values {
                        lhs_instances.push(r);
                    }
                },
                Err(e) => return Err(e),
            }
        }
        let mut retval = Vec::new();
        let mut rhs_values;
        if evaluate_rhs {
            match self.eval_value(rhs, true) {
                Ok(result) => rhs_values = result.values,
                Err(e) => return Err(e),
            }
        } else {
            if let Some(expr) = &rhs.expression {
                rhs_values = Vec::new();
                for v in &expr.values {
                    rhs_values.push(v.clone());
                }
            } else {
                rhs_values = vec![rhs.clone()];
            }
        }
        if lhs_values.len() != rhs_values.len() {
            return Err("assign: LHS and RHS length must be equal".to_owned())
        }
        for i in 0..lhs_values.len() {
            if let Some(id) = lhs_values[i].identifier_id {
                if evaluate_rhs {
                    match self.cast_value(&rhs_values[i], lhs_instances[i].sil_type.clone()) {
                        Ok(set_value) => {
                            self.context.set_value_from_identifier_id(id, set_value);
                        },
                        Err(e) => return Err(e),
                    }
                } else {
                    self.context.set_value_from_identifier_id(id, rhs_values[i].clone());
                }
                let mut value = Value::new();
                value.identifier_id = Some(id);
                retval.push(value);
            } else {
                if !self.context.is_untyped() {
                    return Err("assign: LHS must be declared identifier\n        Hint: You can use untyped function".to_owned())
                }
                if lhs_values[i].identifier.is_none() {
                    return Err("assign: LHS must be identifier".to_owned())
                }
                let current_scope = self.context.current_scope();
                let id = self.context.store_identifier(current_scope.scope_number, lhs_values[i].identifier.as_ref().unwrap(), rhs.clone());
                let mut value = Value::new();
                value.identifier_id = Some(id);
                retval.push(value)
            }
        }
        Ok(retval)
    }
}
