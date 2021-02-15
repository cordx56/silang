use super::{
    IdentifierType,
    IdentifierValue,
    FactorKind,
    Factor,
    Expression,
    Context,
};

use super::run::{
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
                        return Err("rval must be a identifier")
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
    Err("Not implemented!")
}
