pub mod control;
pub mod declare;
pub mod function;
pub mod types;
pub mod print;
pub mod arithmetic;
pub mod compare;
use crate::silang::{
    Interpreter,
    Value,
    EvalReturn,
    EvalResult,
};
use crate::parser;

impl Interpreter {
    #[cfg(any(target_family = "unix", target_family = "windows"))]
    pub fn import(&mut self, args: &[Value]) -> Result<EvalReturn, String> {
        let mut module_names = Vec::new();
        for arg in &args[1..] {
            match self.eval_value(arg, true) {
                Ok(result) => {
                    for r in result.values {
                        module_names.push(r);
                    }
                },
                Err(e) => return Err(e),
            }
        }

        for module_name in module_names {
            if let Some(module_name_string) = module_name.string {
                unsafe {
                    match libloading::Library::new(format!("lib{}.so", module_name_string)) {
                        Ok(lib) => {
                            match lib.get::<libloading::Symbol<unsafe extern fn(&mut Interpreter)>>(b"sil_load_lib") {
                                Ok(func) => {
                                    func(self);
                                    self.libraries.push(lib);
                                    continue;
                                },
                                Err(_) => return Err(format!("import: Function get error\n        Is lib{}.so proper library file?", module_name_string))
                            }
                        },
                        Err(_) => {},
                    }
                }
                let file_name = format!("{}.sil", module_name_string);
                let mut buffer;
                match std::fs::read_to_string(&file_name) {
                    Ok(s) => {
                        buffer = s;
                    },
                    Err(_) => {
                        return Err(format!("import: File read error\n        Is {} exists?", file_name))
                    },
                }
                buffer.push_str("\n");
                let parse_result = parser::program_all_consuming(&buffer);
                match parse_result {
                    Ok(program) => {
                        match self.run(&program.1) {
                            Ok(_) => {},
                            Err(e) => {
                                return Err(e)
                            }
                        }
                    },
                    Err(_) => {
                        return Err("import: Program parse error".to_owned());
                    }
                }
            } else {
                return Err("import: Argument must be string".to_owned())
            }
        }
        Ok(
            EvalReturn {
                result: EvalResult::Normal,
                values: vec![],
            }
        )
    }
    #[cfg(any(target_family = "wasm"))]
    pub fn import(&mut self, args: &[Value]) -> Result<EvalReturn, String> {
        Err("import is not supported in wasm".to_owned())
    }
}

/*
use super::{
    FactorKind,
    Factor,
    Context,
};

use super::parser;

use super::run::{
    eval_factor,
    eval,
    run,
};

use super::define;

use std::collections::HashMap;
use std::fs;

pub fn import(ctx: &mut Context, factors: Vec<Factor>) -> Result<Vec<Factor>, String> {
    for f in factors[1..].iter() {
        let module_name;
        if f.kind == FactorKind::Identifier {
            match ctx.search_identifier(f.name.as_ref().unwrap()) {
                Some(iv) => {
                    if iv.1.kind == FactorKind::String {
                        module_name = iv.1.string.as_ref().unwrap();
                    } else {
                        return Err("Factor must be a string".to_owned())
                    }
                },
                None => {
                    return Err(define::IDENTIFIER_NOT_DEFINED.to_owned())
                },
            }
        } else if f.kind == FactorKind::String {
            module_name = f.string.as_ref().unwrap();
        } else {
            return Err("Factor must be a string".to_owned());
        }

        unsafe {
            match libloading::Library::new(format!("lib{}.so", module_name)) {
                Ok(lib) => {
                    match lib.get::<libloading::Symbol<unsafe extern fn(&mut Context)>>(b"sil_load_lib") {
                        Ok(func) => {
                            func(ctx);
                            return Ok(Vec::new());
                        },
                        Err(_) => {
                            return Err("Import Error: Function get error!".to_owned());
                        }
                    }
                },
                Err(_) => {
                }
            }
        }
        let file_name = format!("{}.sil", module_name);
        let mut buffer;
        match fs::read_to_string(file_name) {
            Ok(s) => {
                buffer = s;
            },
            Err(_) => {
                return Err("Import Error: File not found".to_owned())
            },
        }
        buffer.push_str("\n");
        let parse_result = parser::program_all_consuming(&buffer);
        match parse_result {
            Ok(program) => {
                match run(ctx, program.1) {
                    Ok(_) => {},
                    Err(e) => {
                        return Err(e)
                    }
                }
            },
            Err(_) => {
                return Err("Import Error: Program parse error".to_owned());
            }
        }
    }
    return Ok(Vec::new())
}

pub fn define(ctx: &mut Context, factors: Vec<Factor>) -> Result<Vec<Factor>, String> {
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
                    match ctx.search_identifier(f.name.as_ref().unwrap()) {
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
        match ctx.search_identifier(rval.name.as_ref().unwrap()) {
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

pub fn assign_variable(ctx: &mut Context, lval: &Factor, rval: &Factor) -> Result<(), String> {
    if lval.kind != FactorKind::Identifier {
        return Err(define::LVAL_MUST_BE_IDENTIFIER.to_owned())
    }
    let left_factor_name = lval.name.as_ref().unwrap();
    let scope;
    match ctx.search_identifier(left_factor_name) {
        Some(iv) => {
            scope = iv.0;
        },
        None => {
            return Err(define::IDENTIFIER_NOT_DEFINED.to_owned())
        },
    }
    if rval.kind == FactorKind::Identifier {
        let right_identifier_value;
        match ctx.search_identifier(rval.name.as_ref().unwrap()) {
            Some(iv) => {
                right_identifier_value = iv.1.clone();
            },
            None => {
                return Err(define::IDENTIFIER_NOT_DEFINED.to_owned())
            },
        }
        if ctx.identifier_storage[scope][left_factor_name].kind != right_identifier_value.kind {
            return Err(define::TYPE_NOT_MATCHED.to_owned())
        }
        ctx.assign_identifier(scope, left_factor_name, right_identifier_value);
        return Ok(());
    } else {
        if ctx.identifier_storage[scope][left_factor_name].kind == FactorKind::String {
            if rval.kind != FactorKind::String {
                return Err(define::TYPE_NOT_MATCHED.to_owned())
            }
            ctx.identifier_storage[scope].get_mut(left_factor_name).unwrap().string = Some(rval.string.as_ref().unwrap().to_owned());
            return Ok(());
        } else if ctx.identifier_storage[scope][left_factor_name].kind == FactorKind::Int {
            if rval.kind == FactorKind::Int {
                ctx.identifier_storage[scope].get_mut(left_factor_name).unwrap().int = Some(rval.int.unwrap());
                return Ok(());
            } else if rval.kind == FactorKind::Float {
                ctx.identifier_storage[scope].get_mut(left_factor_name).unwrap().int = Some(rval.float.unwrap() as i64);
                return Ok(());
            } else {
                return Err(define::TYPE_NOT_MATCHED.to_owned())
            }
        } else if ctx.identifier_storage[scope][left_factor_name].kind == FactorKind::Float {
            if rval.kind == FactorKind::Int {
                ctx.identifier_storage[scope].get_mut(left_factor_name).unwrap().float = Some(rval.int.unwrap() as f64);
                return Ok(());
            } else if rval.kind == FactorKind::Float {
                ctx.identifier_storage[scope].get_mut(left_factor_name).unwrap().float = Some(rval.float.unwrap());
                return Ok(());
            } else {
                return Err(define::TYPE_NOT_MATCHED.to_owned())
            }
        } else {
            return Err(define::TYPE_NOT_MATCHED.to_owned())
        }
    }
}

pub fn assign(ctx: &mut Context, factors: Vec<Factor>) -> Result<Vec<Factor>, String> {
    if factors.len() != 3 {
        return Err("Argument length must be 2".to_owned())
    }
    let lval = &factors[1];
    let rval = &factors[2];

    let right_factors;
    match eval_factor(ctx, rval) {
        Ok(factors) => right_factors = factors,
        Err(e) => return Err(e),
    }

    let left_factors;
    match eval_factor(ctx, lval) {
        Ok(factors) => left_factors = factors,
        Err(e) => return Err(e),
    }

    if left_factors.len() != right_factors.len() {
        return Err("lval and rval length must be equal".to_owned())
    }
    for n in 0..left_factors.len() {
        match assign_variable(ctx, &left_factors[n], &right_factors[n]) {
            Ok(_) => {},
            Err(e) => return Err(e),
        }
    }
    Ok(left_factors)
}

pub fn print_factor(ctx: &mut Context, f: Factor) -> Result<(), String> {
    if f.kind == FactorKind::Identifier {
        match ctx.search_identifier(f.name.as_ref().unwrap()) {
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
                    return Err("Can't print unknown identifier".to_owned()) }
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
        match ctx.search_identifier(f.name.as_ref().unwrap()) {
            Some(iv) => {
                res.push(iv.1.clone());
            },
            None => {
                return Err(define::IDENTIFIER_NOT_DEFINED.to_owned())
            }
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
    let to_type;
    match ctx.search_identifier(rval.name.as_ref().unwrap()) {
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
        match ctx.search_identifier(lval.name.as_ref().unwrap()) {
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
        match ctx.search_identifier(res.name.as_ref().unwrap()) {
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
            match ctx.search_identifier(factors_to_add[n].name.as_ref().unwrap()) {
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

pub fn equal_factor(lval: &Factor, rval: &Factor) -> Result<Factor, String> {
    let mut ret = Factor::new();
    ret.kind = FactorKind::Bool;
    if lval.kind == FactorKind::String && rval.kind == FactorKind::String {
        ret.bool = Some(lval.string.as_ref().unwrap() == rval.string.as_ref().unwrap());
        return Ok(ret)
    } else if lval.kind == FactorKind::Int {
        if rval.kind == FactorKind::Int {
            ret.bool = Some(lval.int.unwrap() == rval.int.unwrap());
            return Ok(ret)
        } else if rval.kind == FactorKind::Float {
            ret.bool = Some((lval.int.unwrap() as f64) == rval.float.unwrap());
            return Ok(ret)
        }
    } else if lval.kind == FactorKind::Float {
        if rval.kind == FactorKind::Int {
            ret.bool = Some(lval.float.unwrap() == (rval.int.unwrap() as f64));
            return Ok(ret)
        } else if rval.kind == FactorKind::Float {
            ret.bool = Some(lval.float.unwrap() == rval.float.unwrap());
            return Ok(ret)
        }
    }
    return Err(define::UNSUPPORTED_OPERATION.to_owned())
}

pub fn equal(ctx: &mut Context, factors: Vec<Factor>) -> Result<Vec<Factor>, String> {
    let mut factors_to_equal = Vec::new();
    for n in 1..factors.len() {
        match eval_factor(ctx, &factors[n]) {
            Ok(fs) => factors_to_equal.extend_from_slice(&fs),
            Err(e) => return Err(e),
        }
    }
    if factors_to_equal.len() < 2 {
        return Err(define::ARGUMENT_LENGTH_MISMATCH.to_owned())
    }
    let mut res = Factor::new();
    res.kind = FactorKind::Bool;
    res.bool = Some(false);

    let mut cmp = factors_to_equal[0].clone();
    if cmp.kind == FactorKind::Identifier {
        match ctx.search_identifier(factors_to_equal[0].name.as_ref().unwrap()) {
            Some(iv) => cmp = iv.1.clone(),
            None => return Err(define::IDENTIFIER_NOT_DEFINED.to_owned()),
        }
    }
    for n in 1..factors_to_equal.len() {
        if factors_to_equal[n].kind == FactorKind::Identifier {
            match ctx.search_identifier(factors_to_equal[n].name.as_ref().unwrap()) {
                Some(iv) => {
                    match equal_factor(&cmp, &iv.1) {
                        Ok(ef) => {
                            if ef.kind == FactorKind::Bool && ef.bool.unwrap() == false {
                                return Ok(vec![ef])
                            }
                        },
                        Err(e) => return Err(e),
                    }
                },
                None => return Err(define::IDENTIFIER_NOT_DEFINED.to_owned()),
            }
        } else {
            match equal_factor(&cmp, &factors_to_equal[n]) {
                Ok(ef) => {
                    if ef.kind == FactorKind::Bool && ef.bool.unwrap() == false {
                        return Ok(vec![ef])
                    }
                },
                Err(e) => return Err(e),
            }
        }
    }
    let mut factor_true = Factor::new();
    factor_true.kind = FactorKind::Bool;
    factor_true.bool = Some(true);
    Ok(vec![factor_true])
}
*/
