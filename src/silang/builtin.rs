use super::{
    FactorKind,
    Factor,
    Expression,
    Context,
};

use super::run::{
    search_identifier,
    assign_identifier,
    eval,
};

use super::define;

use std::collections::HashMap;

pub fn define_variable(ctx: &mut Context, factors: Vec<Factor>) -> Result<Vec<Factor>, String> {
    if factors.len() != 3 {
        return Err("Argument length must be 2".to_owned())
    }
    let lval = &factors[1];
    let rval = &factors[2];

    let mut type_name_vec = Vec::new();
    if rval.kind == FactorKind::Expression {
        match eval(ctx, rval.expression.as_ref().unwrap()) {
            Ok(er) => {
                for f in er {
                    if f.kind != FactorKind::Identifier {
                        return Err("rval must be identifier".to_owned())
                    }
                    match search_identifier(ctx, f.name.as_ref().unwrap()) {
                        Some(iv) => {
                            if iv.1.kind != FactorKind::TypeName {
                                return Err("rval must be type name".to_owned())
                            } else {
                                type_name_vec.push(iv.1.string.as_ref().unwrap().to_owned());
                            }
                        },
                        None => {
                            return Err(define::IDENTIFIER_NOT_DEFINED.to_owned())
                        },
                    }
                }
            },
            Err(e) => {
                return Err(e)
            },
        }
    } else {
        if rval.kind != FactorKind::Identifier {
            return Err("rval must be identifier".to_owned())
        }
        match search_identifier(ctx, rval.name.as_ref().unwrap()) {
            Some(iv) => {
                if iv.1.kind != FactorKind::TypeName {
                    return Err("rval must be type name".to_owned())
                } else {
                    type_name_vec.push(iv.1.string.as_ref().unwrap().to_owned());
                }
            },
            None => {
                return Err(define::IDENTIFIER_NOT_DEFINED.to_owned())
            }
        }
    }

    let current_scope = ctx.current_scope();
    let mut return_vec = Vec::new();
    if lval.kind == FactorKind::Expression {
        match eval(ctx, lval.expression.as_ref().unwrap()) {
            Ok(er) => {
                for f in er {
                    if f.kind != FactorKind::Identifier {
                        return Err(define::LVAL_MUST_BE_IDENTIFIER.to_owned())
                    }
                    if ctx.identifier_storage[current_scope].contains_key(f.name.as_ref().unwrap()) {
                        return Err(define::REDEFINITION_NOT_SUPPORTED.to_owned())
                    }
                    return_vec.push(f);
                }
            },
            Err(e) => {
                return Err(e)
            },
        }
    } else {
        if lval.kind != FactorKind::Identifier {
            return Err(define::LVAL_MUST_BE_IDENTIFIER.to_owned())
        }
        if ctx.identifier_storage[current_scope].contains_key(lval.name.as_ref().unwrap()) {
            return Err(define::REDEFINITION_NOT_SUPPORTED.to_owned())
        }
        return_vec.push(lval.clone());
    }

    if return_vec.len() != type_name_vec.len() {
        return Err("lval and rval length must be equal".to_owned())
    }
    for n in 0..return_vec.len() {
        if type_name_vec[n] == define::STRING {
            let mut iv = Factor::new();
            iv.kind = FactorKind::String;
            iv.string = Some("".to_owned());
            ctx.identifier_storage[current_scope].insert(
                return_vec[n].name.as_ref().unwrap().to_owned(),
                iv,
            );
        } else if type_name_vec[n] == define::INT {
            let mut iv = Factor::new();
            iv.kind = FactorKind::Int;
            iv.int = Some(0);
            ctx.identifier_storage[current_scope].insert(
                return_vec[n].name.as_ref().unwrap().to_owned(),
                iv,
            );
        } else if type_name_vec[n] == define::FLOAT {
            let mut iv = Factor::new();
            iv.kind = FactorKind::Float;
            iv.float = Some(0.0);
            ctx.identifier_storage[current_scope].insert(
                return_vec[n].name.as_ref().unwrap().to_owned(),
                iv,
            );
        } else if type_name_vec[n] == define::BOOL {
            let mut iv = Factor::new();
            iv.kind = FactorKind::Bool;
            iv.bool = Some(false);
            ctx.identifier_storage[current_scope].insert(
                return_vec[n].name.as_ref().unwrap().to_owned(),
                iv,
            );
        } else if type_name_vec[n] == define::VECTOR {
            let mut iv = Factor::new();
            iv.kind = FactorKind::Vector;
            iv.vector = Some(Vec::new());
            ctx.identifier_storage[current_scope].insert(
                return_vec[n].name.as_ref().unwrap().to_owned(),
                iv,
            );
        } else if type_name_vec[n] == define::MAP {
            let mut iv = Factor::new();
            iv.kind = FactorKind::Map;
            iv.map = Some(HashMap::new());
            ctx.identifier_storage[current_scope].insert(
                return_vec[n].name.as_ref().unwrap().to_owned(),
                iv,
            );
        } else if type_name_vec[n] == define::FUNCTION {
            let mut iv = Factor::new();
            iv.kind = FactorKind::Function;
            ctx.identifier_storage[current_scope].insert(
                return_vec[n].name.as_ref().unwrap().to_owned(),
                iv,
            );
        } else {
            return Err("Unknown type".to_owned())
        }
    }
    Ok(return_vec)
}

pub fn assign_variable(ctx: &mut Context, factors: Vec<Factor>) -> Result<Vec<Factor>, String> {
    if factors.len() != 3 {
        return Err("Argument length must be 2".to_owned())
    }
    let lval = &factors[1];
    let rval = &factors[2];

    let mut right_factors = Vec::new();
    if rval.kind == FactorKind::Expression {
        match eval(ctx, rval.expression.as_ref().unwrap()) {
            Ok(er) => {
                right_factors = er;
            },
            Err(e) => {
                return Err(e)
            },
        }
    } else {
        right_factors.push(rval.clone());
    }

    let mut left_factors = Vec::new();
    if lval.kind == FactorKind::Expression {
        match eval(ctx, lval.expression.as_ref().unwrap()) {
            Ok(er) => {
                left_factors = er;
            },
            Err(e) => {
                return Err(e)
            },
        }
    } else {
        left_factors.push(lval.clone());
    }

    if left_factors.len() != right_factors.len() {
        return Err("lval and rval length must be equal".to_owned())
    }
    for n in 0..left_factors.len() {
        let mut right_identifier_value = Factor::new();
        if right_factors[n].kind == FactorKind::Identifier {
            match search_identifier(ctx, right_factors[n].name.as_ref().unwrap()) {
                Some(iv) => {
                    right_identifier_value = iv.1.clone();
                },
                None => {
                    return Err(define::IDENTIFIER_NOT_DEFINED.to_owned())
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
                return Err(define::IDENTIFIER_NOT_DEFINED.to_owned())
            },
        }
        if right_factors[n].kind == FactorKind::Identifier {
            if ctx.identifier_storage[scope][left_factor_name].kind != right_identifier_value.kind {
                return Err("Type not matched".to_owned())
            }
            assign_identifier(ctx, scope, left_factor_name, right_identifier_value);
        } else {
            if ctx.identifier_storage[scope][left_factor_name].kind == FactorKind::String {
                if right_factors[n].kind != FactorKind::String {
                    return Err("Type not matched".to_owned())
                }
                ctx.identifier_storage[scope].get_mut(left_factor_name).unwrap().string = Some(right_factors[n].string.as_ref().unwrap().to_owned());
            } else if ctx.identifier_storage[scope][left_factor_name].kind == FactorKind::Int {
                if right_factors[n].kind == FactorKind::Int {
                    ctx.identifier_storage[scope].get_mut(left_factor_name).unwrap().int = Some(right_factors[n].int.unwrap());
                } else if right_factors[n].kind == FactorKind::Float {
                    ctx.identifier_storage[scope].get_mut(left_factor_name).unwrap().int = Some(right_factors[n].float.unwrap() as i64);
                } else {
                    return Err("Type not matched".to_owned())
                }
            } else if ctx.identifier_storage[scope][left_factor_name].kind == FactorKind::Float {
                if right_factors[n].kind == FactorKind::Int {
                    ctx.identifier_storage[scope].get_mut(left_factor_name).unwrap().float = Some(right_factors[n].int.unwrap() as f64);
                } else if right_factors[n].kind == FactorKind::Float {
                    ctx.identifier_storage[scope].get_mut(left_factor_name).unwrap().float = Some(right_factors[n].float.unwrap());
                } else {
                    return Err("Type not matched".to_owned())
                }
            }
        }
    }
    Ok(left_factors)
}

pub fn print_factor(ctx: &mut Context, f: Factor) -> Result<(), String> {
    if f.kind == FactorKind::Identifier {
        match search_identifier(ctx, f.name.as_ref().unwrap()) {
            Some(iv) => {
                if iv.1.kind == FactorKind::String {
                    print!("{}", iv.1.string.as_ref().unwrap());
                } else if iv.1.kind == FactorKind::Int {
                    print!("{}", iv.1.int.unwrap());
                } else if iv.1.kind == FactorKind::Float {
                    print!("{}", iv.1.float.unwrap());
                } else if iv.1.kind == FactorKind::Bool {
                    print!("{}", iv.1.bool.unwrap());
                } else {
                    return Err("Can't print unknown identifier".to_owned())
                }
            },
            None => {
                return Err(define::IDENTIFIER_NOT_DEFINED.to_owned())
            },
        }
    } else if f.kind == FactorKind::String {
        print!("{}", f.string.unwrap());
    } else if f.kind == FactorKind::Int {
        print!("{}", f.int.unwrap());
    } else if f.kind == FactorKind::Float {
        print!("{}", f.float.unwrap());
    } else if f.kind == FactorKind::Bool {
        if f.bool.unwrap() {
            print!("true");
        } else {
            print!("false");
        }
    } else {
        return Err("Can't print unknown factor".to_owned())
    }
    Ok(())
}

pub fn print(ctx: &mut Context, factors: Vec<Factor>) -> Result<Vec<Factor>, String> {
    let mut factors_to_print = Vec::new();
    for n in 1..factors.len() {
        if factors[n].kind == FactorKind::Expression {
            match eval(ctx, factors[n].expression.as_ref().unwrap()) {
                Ok(er) => {
                    for f in er {
                        factors_to_print.push(f);
                    }
                },
                Err(e) => {
                    return Err(e)
                },
            }
        } else {
            factors_to_print.push(factors[n].clone());
        }
    }
    for f in factors_to_print {
        match print_factor(ctx, f) {
            Ok(_) => {},
            Err(e) => {
                return Err(e)
            },
        }
    }
    if factors[0].name.as_ref().unwrap() == define::PRINTLN {
        println!("");
    }
    Ok(Vec::new())
}

/*
pub fn identifier_value_to_factor(iv: &IdentifierValue) -> Result<Factor, String> {
    if iv.identifier_type == FactorKind::String {
        Ok(Factor {
            kind: FactorKind::String,
            name: None,
            string: Some(iv.string.as_ref().unwrap().clone()),
            int: None,
            float: None,
            bool: None,
            vector: None,
            map: None,
            expression: None,
        })
    } else if iv.identifier_type == FactorKind::Int {
        Ok(Factor {
            kind: FactorKind::Int,
            name: None,
            string: None,
            int: Some(iv.int.unwrap()),
            float: None,
            bool: None,
            vector: None,
            map: None,
            expression: None,
        })
    } else if iv.identifier_type == FactorKind::Float {
        Ok(Factor {
            kind: FactorKind::Float,
            name: None,
            string: None,
            int: None,
            float: Some(iv.float.unwrap()),
            bool: None,
            vector: None,
            map: None,
            expression: None,
        })
    } else if iv.identifier_type == FactorKind::Bool {
        Ok(Factor {
            kind: FactorKind::Bool,
            name: None,
            string: None,
            int: None,
            float: None,
            bool: Some(iv.bool.unwrap()),
            vector: None,
            map: None,
            expression: None,
        })
    } else {
        Err("Unable to cast from Identifier to Factor".to_owned())
    }
}*/

pub fn value(ctx: &mut Context, factors: Vec<Factor>) -> Result<Vec<Factor>, String> {
    let mut factors_to_value = Vec::new();
    for n in 1..factors.len() {
        if factors[n].kind == FactorKind::Expression {
            match eval(ctx, factors[n].expression.as_ref().unwrap()) {
                Ok(er) => {
                    for f in er {
                        factors_to_value.push(f);
                    }
                },
                Err(e) => {
                    return Err(e)
                },
            }
        } else {
            factors_to_value.push(factors[n].clone());
        }
    }
    let mut res = Vec::new();
    for f in factors_to_value {
        if f.kind != FactorKind::Identifier {
            return Err(define::LVAL_MUST_BE_IDENTIFIER.to_owned());
        }
        match search_identifier(ctx, f.name.as_ref().unwrap()) {
            Some(iv) => {
                res.push(iv.1.clone());
            },
            None => return Err(define::IDENTIFIER_NOT_DEFINED.to_owned()),
        }
    }
    Ok(res)
}
pub fn make_vector(ctx: &mut Context, factors: Vec<Factor>) -> Result<Vec<Factor>, String> {
    let mut factors_to_vec = Vec::new();
    for n in 1..factors.len() {
        if factors[n].kind == FactorKind::Expression {
            match eval(ctx, factors[n].expression.as_ref().unwrap()) {
                Ok(er) => {
                    for f in er {
                        factors_to_vec.push(f);
                    }
                },
                Err(e) => {
                    return Err(e)
                },
            }
        } else {
            factors_to_vec.push(factors[n].clone());
        }
    }
    let mut nf = Factor::new();
    nf.kind = FactorKind::Vector;
    nf.vector = Some(factors_to_vec);
    Ok(vec![nf])
}

pub fn cast_factor(factor: &Factor, to: FactorKind) -> Result<Factor, String> {
    let mut res = factor.clone();
    if factor.kind == FactorKind::String {
        if to == FactorKind::Int {
            match factor.string.as_ref().unwrap().parse() {
                Ok(num) => {
                    res = Factor {
                        kind: to,
                        name: None,
                        string: None,
                        int: Some(num),
                        float: None,
                        bool: None,
                        vector: None,
                        map: None,
                        expression: None,
                        user_defined_function: None,
                        function: None,
                    };
                },
                Err(_) => {
                    return Err(define::UNABLE_TO_CAST.to_owned())
                },
            }
        } else if to == FactorKind::Float {
            match factor.string.as_ref().unwrap().parse() {
                Ok(num) => {
                    res = Factor {
                        kind: to,
                        name: None,
                        string: None,
                        int: None,
                        float: Some(num),
                        bool: None,
                        vector: None,
                        map: None,
                        expression: None,
                        user_defined_function: None,
                        function: None,
                    };
                },
                Err(_) => {
                    return Err(define::UNABLE_TO_CAST.to_owned())
                },
            }
        }
    } else if factor.kind == FactorKind::Int {
        if to == FactorKind::String {
            res = Factor {
                kind: to,
                name: None,
                string: Some(factor.int.unwrap().to_string()),
                int: None,
                float: None,
                bool: None,
                vector: None,
                map: None,
                expression: None,
                user_defined_function: None,
                function: None,
            }
        } else if to == FactorKind::Float {
            res = Factor {
                kind: to,
                name: None,
                string: None,
                int: None,
                float: Some(factor.int.unwrap() as f64),
                bool: None,
                vector: None,
                map: None,
                expression: None,
                user_defined_function: None,
                function: None,
            }
        }
     } else if factor.kind == FactorKind::Float {
        if to == FactorKind::String {
            res = Factor {
                kind: to,
                name: None,
                string: Some(factor.float.unwrap().to_string()),
                int: None,
                float: None,
                bool: None,
                vector: None,
                map: None,
                expression: None,
                user_defined_function: None,
                function: None,
            }
        } else if to == FactorKind::Int {
            res = Factor {
                kind: to,
                name: None,
                string: None,
                int: Some(factor.float.unwrap() as i64),
                float: None,
                bool: None,
                vector: None,
                map: None,
                expression: None,
                user_defined_function: None,
                function: None,
            }
        }
    } else {
        return Err("Unable to cast".to_owned())
    }
    Ok(res)
}

pub fn as_cast(ctx: &mut Context, factors: Vec<Factor>) -> Result<Vec<Factor>, String> {
    if factors.len() != 3 {
        return Err("Argument length must be 2".to_owned())
    }
    let mut lval = factors[1].clone();
    let mut rval = factors[2].clone();
    if lval.kind == FactorKind::Expression {
        match eval(ctx, lval.expression.as_ref().unwrap()) {
            Ok(er) => {
                if er.len() != 1 {
                    return Err("lval length must be 1".to_owned())
                }
                lval = er[0].clone();
            },
            Err(e) => {
                return Err(e)
            },
        }
    }
    if rval.kind == FactorKind::Expression {
        match eval(ctx, rval.expression.as_ref().unwrap()) {
            Ok(er) => {
                if er.len() != 1 {
                    return Err("rval length must be 1".to_owned())
                }
                rval = er[0].clone();
            },
            Err(e) => {
                return Err(e)
            },
        }
    }
    let mut to_type = FactorKind::String;
    match search_identifier(ctx, rval.name.as_ref().unwrap()) {
        Some(iv) => {
            if iv.1.kind != FactorKind::TypeName {
                return Err("Identifier is not type".to_owned())
            }
            if iv.1.string.as_ref().unwrap() == define::STRING {
                to_type = FactorKind::String;
            } else if iv.1.string.as_ref().unwrap() == define::INT {
                to_type = FactorKind::Int;
            } else if iv.1.string.as_ref().unwrap() == define::FLOAT {
                to_type = FactorKind::Float;
            } else {
                return Err(format!("Can't cast to {}", iv.1.string.as_ref().unwrap()))
            }
        },
        None => {
            return Err(define::IDENTIFIER_NOT_DEFINED.to_owned())
        },
    }
    if lval.kind == FactorKind::Identifier {
        match search_identifier(ctx, lval.name.as_ref().unwrap()) {
            Some(iv) => {
                match cast_factor(&iv.1, to_type) {
                    Ok(f) => {
                        return Ok(vec![f])
                    },
                    Err(e) => {
                        return Err(e)
                    },
                }
            },
            None => {
                return Err(define::IDENTIFIER_NOT_DEFINED.to_owned())
            },
        }
    } else {
        match cast_factor(&lval, to_type) {
            Ok(f) => Ok(vec![f]),
            Err(e) => Err(e),
        }
    }
}

pub fn factor_arithmetic(opr: &str, lval: &Factor, rval: &Factor) -> Result<Factor, String> {
    if opr == define::ADD {
        if lval.kind == FactorKind::String && rval.kind == FactorKind::String {
            let mut string = String::new();
            string += lval.string.as_ref().unwrap();
            string += rval.string.as_ref().unwrap();
            let mut ret = Factor::new();
            ret.kind = FactorKind::String;
            ret.string = Some(string);
            return Ok(ret)
        } else if lval.kind == FactorKind::Int {
            if rval.kind == FactorKind::Int {
                let mut ret = Factor::new();
                ret.kind = FactorKind::Int;
                ret.int = Some(lval.int.unwrap() + rval.int.unwrap());
                return Ok(ret)
            } else if rval.kind == FactorKind::Float {
                let mut ret = Factor::new();
                ret.kind = FactorKind::Float;
                ret.float = Some((lval.int.unwrap() as f64) + rval.float.unwrap());
                return Ok(ret)
            }
        } else if lval.kind == FactorKind::Float {
            if rval.kind == FactorKind::Int {
                let mut ret = Factor::new();
                ret.kind = FactorKind::Float;
                ret.float = Some(lval.float.unwrap() + (rval.int.unwrap() as f64));
                return Ok(ret)
            } else if rval.kind == FactorKind::Float {
                let mut ret = Factor::new();
                ret.kind = FactorKind::Float;
                ret.float = Some(lval.float.unwrap() + rval.float.unwrap());
                return Ok(ret)
            }
        }
        return Err(define::UNSUPPORTED_OPERATION.to_owned())
    } else if opr == define::SUB {
        if lval.kind == FactorKind::Int {
            if rval.kind == FactorKind::Int {
                let mut ret = Factor::new();
                ret.kind = FactorKind::Int;
                ret.int = Some(lval.int.unwrap() - rval.int.unwrap());
                return Ok(ret)
            } else if rval.kind == FactorKind::Float {
                let mut ret = Factor::new();
                ret.kind = FactorKind::Float;
                ret.float = Some((lval.int.unwrap() as f64) - rval.float.unwrap());
                return Ok(ret)
            }
        } else if lval.kind == FactorKind::Float {
            if rval.kind == FactorKind::Int {
                let mut ret = Factor::new();
                ret.kind = FactorKind::Float;
                ret.float = Some(lval.float.unwrap() - (rval.int.unwrap() as f64));
                return Ok(ret)
            } else if rval.kind == FactorKind::Float {
                let mut ret = Factor::new();
                ret.kind = FactorKind::Float;
                ret.float = Some(lval.float.unwrap() - rval.float.unwrap());
                return Ok(ret)
            }
        }
        return Err(define::UNSUPPORTED_OPERATION.to_owned())
    } else if opr == define::MUL {
        if lval.kind == FactorKind::Int {
            if rval.kind == FactorKind::Int {
                let mut ret = Factor::new();
                ret.kind = FactorKind::Int;
                ret.int = Some(lval.int.unwrap() * rval.int.unwrap());
                return Ok(ret)
            } else if rval.kind == FactorKind::Float {
                let mut ret = Factor::new();
                ret.kind = FactorKind::Float;
                ret.float = Some((lval.int.unwrap() as f64) * rval.float.unwrap());
                return Ok(ret)
            }
        } else if lval.kind == FactorKind::Float {
            if rval.kind == FactorKind::Int {
                let mut ret = Factor::new();
                ret.kind = FactorKind::Float;
                ret.float = Some(lval.float.unwrap() * (rval.int.unwrap() as f64));
                return Ok(ret)
            } else if rval.kind == FactorKind::Float {
                let mut ret = Factor::new();
                ret.kind = FactorKind::Float;
                ret.float = Some(lval.float.unwrap() * rval.float.unwrap());
                return Ok(ret)
            }
        }
        return Err(define::UNSUPPORTED_OPERATION.to_owned())
    } else if opr == define::DIV {
        if lval.kind == FactorKind::Int {
            if rval.kind == FactorKind::Int {
                let mut ret = Factor::new();
                ret.kind = FactorKind::Int;
                ret.int = Some(lval.int.unwrap() / rval.int.unwrap());
                return Ok(ret)
            } else if rval.kind == FactorKind::Float {
                let mut ret = Factor::new();
                ret.kind = FactorKind::Float;
                ret.float = Some((lval.int.unwrap() as f64) / rval.float.unwrap());
                return Ok(ret)
            }
        } else if lval.kind == FactorKind::Float {
            if rval.kind == FactorKind::Int {
                let mut ret = Factor::new();
                ret.kind = FactorKind::Float;
                ret.float = Some(lval.float.unwrap() / (rval.int.unwrap() as f64));
                return Ok(ret)
            } else if rval.kind == FactorKind::Float {
                let mut ret = Factor::new();
                ret.kind = FactorKind::Float;
                ret.float = Some(lval.float.unwrap() / rval.float.unwrap());
                return Ok(ret)
            }
        }
        return Err(define::UNSUPPORTED_OPERATION.to_owned())
    } else if opr == define::REM {
        if lval.kind == FactorKind::Int {
            if rval.kind == FactorKind::Int {
                let mut ret = Factor::new();
                ret.kind = FactorKind::Int;
                ret.int = Some(lval.int.unwrap() % rval.int.unwrap());
                return Ok(ret)
            } else if rval.kind == FactorKind::Float {
                let mut ret = Factor::new();
                ret.kind = FactorKind::Float;
                ret.float = Some((lval.int.unwrap() as f64) % rval.float.unwrap());
                return Ok(ret)
            }
        } else if lval.kind == FactorKind::Float {
            if rval.kind == FactorKind::Int {
                let mut ret = Factor::new();
                ret.kind = FactorKind::Float;
                ret.float = Some(lval.float.unwrap() % (rval.int.unwrap() as f64));
                return Ok(ret)
            } else if rval.kind == FactorKind::Float {
                let mut ret = Factor::new();
                ret.kind = FactorKind::Float;
                ret.float = Some(lval.float.unwrap() % rval.float.unwrap());
                return Ok(ret)
            }
        }
        return Err(define::UNSUPPORTED_OPERATION.to_owned())
    } else {
        Err("Unknown operator".to_owned())
    }
}

pub fn arithmetic(ctx: &mut Context, factors: Vec<Factor>) -> Result<Vec<Factor>, String> {
    let opr = factors[0].name.as_ref().unwrap();
    let mut factors_to_add = Vec::new();
    for n in 1..factors.len() {
        if factors[n].kind == FactorKind::Expression {
            match eval(ctx, factors[n].expression.as_ref().unwrap()) {
                Ok(er) => {
                    for f in er {
                        factors_to_add.push(f)
                    }
                },
                Err(e) => {
                    return Err(e)
                },
            }
        } else {
            factors_to_add.push(factors[n].clone());
        }
    }
    if factors_to_add.len() == 0 {
        return Err("No arguments".to_owned())
    }
    let mut res = factors_to_add[0].clone();
    if res.kind == FactorKind::Identifier {
        match search_identifier(ctx, res.name.as_ref().unwrap()) {
            Some(iv) => {
                res = iv.1.clone();
            },
            None => {
                return Err(define::IDENTIFIER_NOT_DEFINED.to_owned())
            },
        }
    }
    for n in 1..factors_to_add.len() {
        if factors_to_add[n].kind == FactorKind::Identifier {
            match search_identifier(ctx, factors_to_add[n].name.as_ref().unwrap()) {
                Some(iv) => {
                    match factor_arithmetic(opr, &res, iv.1) {
                        Ok(f) => {
                            res = f;
                        },
                        Err(e) => {
                            return Err(e)
                        }
                    }
                },
                None => {
                    return Err(define::IDENTIFIER_NOT_DEFINED.to_owned())
                },
            }
        } else {
            match factor_arithmetic(opr, &res, &factors_to_add[n]) {
                Ok(fa) => {
                    res = fa;
                },
                Err(e) => {
                    return Err(e)
                },
            }
        }
    }
    Ok(vec![res])
}
