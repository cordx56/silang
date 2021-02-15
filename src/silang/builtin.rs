use super::{
    IdentifierType,
    IdentifierValue,
    FactorKind,
    Factor,
    Expression,
    Context,
};

use super::run::{
    search_identifier,
    eval,
};

pub fn define_variable(ctx: &mut Context, factors: Vec<Factor>) -> Result<Vec<Factor>, &str> {
    if factors.len() != 3 {
        return Err("Argument length must be 2")
    }
    let lval = &factors[1];
    let rval = &factors[2];

    let mut type_name_vec = Vec::new();
    if rval.kind == FactorKind::Expression {
        match eval(ctx, rval.expression.as_ref().unwrap().clone()) {
            Ok(er) => {
                for f in er {
                    if f.kind != FactorKind::Identifier {
                        return Err("rval must be identifier")
                    }
                    type_name_vec.push(f.name.as_ref().unwrap().to_owned());
                }
            },
            Err(e) => {
                return Err("TODO: Error message")
            },
        }
    } else {
        if rval.kind != FactorKind::Identifier {
            return Err("rval must be identifier")
        }
        type_name_vec.push(rval.name.as_ref().unwrap().to_owned());
    }

    let mut ident_name_vec = Vec::new();
    let mut return_vec = Vec::new();
    if lval.kind == FactorKind::Expression {
        match eval(ctx, lval.expression.as_ref().unwrap().clone()) {
            Ok(er) => {
                for f in er {
                    if f.kind != FactorKind::Identifier {
                        return Err("lval must be identifier")
                    }
                    if ctx.identifier_storage[ctx.scope].contains_key(f.name.as_ref().unwrap()) {
                        return Err("Redefinition not supported")
                    }
                    ident_name_vec.push(f.name.as_ref().unwrap().to_owned());
                    return_vec.push(f);
                }
            },
            Err(e) => {
                return Err("TODO: Error message")
            },
        }
    } else {
        if lval.kind != FactorKind::Identifier {
            return Err("lval must be identifier")
        }
        if ctx.identifier_storage[ctx.scope].contains_key(lval.name.as_ref().unwrap()) {
            return Err("Redefinition not supported")
        }
        ident_name_vec.push(lval.name.as_ref().unwrap().to_owned());
        return_vec.push(lval.clone());
    }

    if ident_name_vec.len() != type_name_vec.len() {
        return Err("lval and rval length must be equal")
    }
    for n in 0..ident_name_vec.len() {
        if type_name_vec[n] == "string" {
            ctx.identifier_storage[ctx.scope].insert(
                ident_name_vec[n].clone(),
                IdentifierValue {
                    identifier_type: IdentifierType::String,
                    string: Some("".to_owned()),
                    int: None,
                    float: None,
                    bool: None,
                    user_defined_function: None,
                    function: None,
                }
            );
        } else if type_name_vec[n] == "int" {
            ctx.identifier_storage[ctx.scope].insert(
                ident_name_vec[n].clone(),
                IdentifierValue {
                    identifier_type: IdentifierType::Int,
                    string: None,
                    int: Some(0),
                    float: None,
                    bool: None,
                    user_defined_function: None,
                    function: None,
                }
            );
        } else if type_name_vec[n] == "float" {
            ctx.identifier_storage[ctx.scope].insert(
                ident_name_vec[n].clone(),
                IdentifierValue {
                    identifier_type: IdentifierType::Float,
                    string: None,
                    int: None,
                    float: Some(0.0),
                    bool: None,
                    user_defined_function: None,
                    function: None,
                }
            );
        } else if type_name_vec[n] == "bool" {
            ctx.identifier_storage[ctx.scope].insert(
                ident_name_vec[n].clone(),
                IdentifierValue {
                    identifier_type: IdentifierType::Bool,
                    string: None,
                    int: None,
                    float: None,
                    bool: Some(false),
                    user_defined_function: None,
                    function: None,
                }
            );
        } else {
            return Err("Unknown type")
        }
    }
    Ok(return_vec)
}

pub fn assign_variable(ctx: &mut Context, factors: Vec<Factor>) -> Result<Vec<Factor>, &str> {
    if factors.len() != 3 {
        return Err("Argument length must be 2")
    }
    let lval = &factors[1];
    let rval = &factors[2];

    let mut right_factors = Vec::new();
    if rval.kind == FactorKind::Expression {
        match eval(ctx, rval.expression.as_ref().unwrap().clone()) {
            Ok(er) => {
                right_factors = er;
            },
            Err(e) => {
                return Err("TODO: Error message")
            },
        }
    } else {
        right_factors.push(rval.clone());
    }

    let mut left_factors = Vec::new();
    if lval.kind == FactorKind::Expression {
        match eval(ctx, lval.expression.as_ref().unwrap().clone()) {
            Ok(er) => {
                left_factors = er;
            },
            Err(e) => {
                return Err("TODO: Error message")
            },
        }
    } else {
        left_factors.push(lval.clone());
    }

    if left_factors.len() != right_factors.len() {
        return Err("lval and rval length must be equal")
    }
    for n in 0..left_factors.len() {
        let mut right_identifier_value = IdentifierValue {
            identifier_type: IdentifierType::None,
            string: None,
            int: None,
            float: None,
            bool: None,
            user_defined_function: None,
            function: None,
        };
        if right_factors[n].kind == FactorKind::Identifier {
            match search_identifier(ctx, right_factors[n].name.as_ref().unwrap()) {
                Some(iv) => {
                    right_identifier_value = iv.1.clone();
                },
                None => {
                    return Err("Identifier not defined")
                },
            }
        }
        let left_factor_name = left_factors[n].name.as_ref().unwrap();
        let mut scope: usize = 0;
        match search_identifier(ctx, left_factor_name) {
            Some(iv) => {
                scope = iv.0;
            },
            None => {
                return Err("Identifier not defined")
            },
        }
        if right_factors[n].kind == FactorKind::Identifier {
            if ctx.identifier_storage[scope][left_factor_name].identifier_type != right_identifier_value.identifier_type {
                return Err("Type not matched")
            }
            ctx.identifier_storage[scope].get_mut(left_factor_name).unwrap().identifier_type = right_identifier_value.identifier_type;
            ctx.identifier_storage[scope].get_mut(left_factor_name).unwrap().string = right_identifier_value.string;
            ctx.identifier_storage[scope].get_mut(left_factor_name).unwrap().int = right_identifier_value.int;
            ctx.identifier_storage[scope].get_mut(left_factor_name).unwrap().float = right_identifier_value.float;
            ctx.identifier_storage[scope].get_mut(left_factor_name).unwrap().bool = right_identifier_value.bool;
            ctx.identifier_storage[scope].get_mut(left_factor_name).unwrap().user_defined_function = right_identifier_value.user_defined_function;
            ctx.identifier_storage[scope].get_mut(left_factor_name).unwrap().function = right_identifier_value.function;
        } else {
            if ctx.identifier_storage[scope][left_factor_name].identifier_type == IdentifierType::String {
                if right_factors[n].kind != FactorKind::String {
                    return Err("Type not matched")
                }
                ctx.identifier_storage[scope].get_mut(left_factor_name).unwrap().string = Some(right_factors[n].string.as_ref().unwrap().to_owned());
            } else if ctx.identifier_storage[scope][left_factor_name].identifier_type == IdentifierType::Int {
                if right_factors[n].kind == FactorKind::Int {
                    ctx.identifier_storage[scope].get_mut(left_factor_name).unwrap().int = Some(right_factors[n].int.unwrap());
                } else if right_factors[n].kind == FactorKind::Float {
                    ctx.identifier_storage[scope].get_mut(left_factor_name).unwrap().int = Some(right_factors[n].float.unwrap() as i64);
                } else {
                    return Err("Type not matched")
                }
            } else if ctx.identifier_storage[scope][left_factor_name].identifier_type == IdentifierType::Float {
                if right_factors[n].kind == FactorKind::Int {
                    ctx.identifier_storage[scope].get_mut(left_factor_name).unwrap().float = Some(right_factors[n].int.unwrap() as f64);
                } else if right_factors[n].kind == FactorKind::Float {
                    ctx.identifier_storage[scope].get_mut(left_factor_name).unwrap().float = Some(right_factors[n].float.unwrap());
                } else {
                    return Err("Type not matched")
                }
            }
        }
    }
    Ok(left_factors)
}

pub fn print(ctx: &mut Context, factors: Vec<Factor>) -> Result<Vec<Factor>, &str> {
    let mut print_factors = Vec::new();
    for n in 1..factors.len() {
        if factors[n].kind == FactorKind::Expression {
            match eval(ctx, factors[n].expression.as_ref().unwrap().clone()) {
                Ok(er) => {
                    for f in er {
                        print_factors.push(f);
                    }
                },
                Err(e) => {
                    return Err("TODO: Error message")
                },
            }
        } else {
            print_factors.push(factors[n].clone());
        }
    }
    for f in print_factors {
        if f.kind == FactorKind::Identifier {
            match search_identifier(ctx, f.name.as_ref().unwrap()) {
                Some(iv) => {
                    if iv.1.identifier_type == IdentifierType::String {
                        print!("{}", iv.1.string.as_ref().unwrap());
                    } else if iv.1.identifier_type == IdentifierType::Int {
                        print!("{}", iv.1.int.unwrap());
                    } else if iv.1.identifier_type == IdentifierType::Float {
                        print!("{}", iv.1.float.unwrap());
                    } else if iv.1.identifier_type == IdentifierType::Bool {
                        print!("{}", iv.1.bool.unwrap());
                    }
                },
                None => {
                    return Err("Identifier not defined")
                },
            }
        } else if f.kind == FactorKind::String {
            print!("{}", f.string.unwrap());
        } else if f.kind == FactorKind::Int {
            print!("{}", f.int.unwrap());
        } else if f.kind == FactorKind::Float {
            print!("{}", f.float.unwrap());
        }
    }
    if factors[0].name.as_ref().unwrap() == "println" {
        println!("");
    }
    Ok(Vec::new())
}
