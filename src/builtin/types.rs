use crate::silang::{
    Interpreter,
    Value,
    SILType,
};
use crate::define;

impl Interpreter {
    pub fn get_type_from_identifier(&self, identifier: &Value) -> Result<SILType, String> {
        if let Some(type_name) = identifier.identifier.as_ref() {
            if type_name == define::STRING {
                Ok(SILType::String)
            } else if type_name == define::INT {
                Ok(SILType::Int)
            } else if type_name == define::FLOAT {
                Ok(SILType::Float)
            } else if type_name == define::BOOL {
                Ok(SILType::Bool)
            } else if type_name == define::VOID {
                Ok(SILType::Void)
            } else {
                Err("Identifier is not type name".to_owned())
            }
        } else {
            Err("target is not identifier".to_owned())
        }
    }
    pub fn cast_value(&self, value: &Value, to: SILType) -> Result<Value, String> {
        if to == SILType::Any || value.expression.is_some() || value.user_defined_function.is_some() {
            return Ok(value.clone())
        }
        if let Some(string) = &value.string {
            if to == SILType::Int {
                match string.parse() {
                    Ok(num) => {
                        let mut v = Value::new();
                        v.int = Some(num);
                        Ok(v)
                    }
                    Err(_) => return Err("Unable to cast".to_owned()),
                }
            } else if to == SILType::Float {
                match string.parse() {
                    Ok(num) => {
                        let mut v = Value::new();
                        v.float = Some(num);
                        Ok(v)
                    }
                    Err(_) => return Err("Unable to cast".to_owned()),
                }
            } else if to == SILType::String {
                Ok(value.clone())
            } else {
                Err("Unable to cast".to_owned())
            }
        } else if let Some(int) = value.int {
            if to == SILType::String {
                let mut v = Value::new();
                v.string = Some(int.to_string());
                Ok(v)
            } else if to == SILType::Float {
                let mut v = Value::new();
                v.float = Some(int as f64);
                Ok(v)
            } else if to == SILType::Int {
                Ok(value.clone())
            } else {
                Err("Unable to cast".to_owned())
            }
        } else if let Some(float) = value.float {
            if to == SILType::String {
                let mut v = Value::new();
                v.string = Some(float.to_string());
                Ok(v)
            } else if to == SILType::Int {
                let mut v = Value::new();
                v.int = Some(float as i64);
                Ok(v)
            } else if to == SILType::Float {
                Ok(value.clone())
            } else {
                Err("Unable to cast".to_owned())
            }
        } else if let Some(_) = value.bool {
            if to == SILType::Bool {
                Ok(value.clone())
            } else {
                Err("Unable to cast".to_owned())
            }
        } else {
            if value.sil_type == to {
                return Ok(value.clone())
            }
            Err("Unable to cast".to_owned())
        }
    }
}
