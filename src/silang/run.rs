use super::{
    UserDefinedFunction,
    FactorKind,
    Factor,
    Expression,
    Statement,
    ScopeType,
    IdentifierStorage,
    Context,
    ExecResult,
    ExecReturn,
};

use super::builtin;
use super::define;

use std::collections::HashMap;

pub fn search_identifier<'a>(ctx: &'a mut Context, name: &str) -> Option<(usize, &'a Factor)> {
    if ctx.scope.is_empty() {
        return None
    } let mut n = ctx.scope.len() - 1;
    loop {
        let scope = ctx.scope[n];
        if ctx.identifier_storage[scope].contains_key(name) {
            return Some((scope, &ctx.identifier_storage[scope][name]))
        }
        if n == 0 {
            return None
        }
        n -= 1;
    }
}

pub fn assign_identifier(ctx: &mut Context, scope: usize, name: &str, iv: Factor) {
    ctx.identifier_storage[scope].get_mut(name).unwrap().kind = iv.kind;
    ctx.identifier_storage[scope].get_mut(name).unwrap().name = iv.name;
    ctx.identifier_storage[scope].get_mut(name).unwrap().string = iv.string;
    ctx.identifier_storage[scope].get_mut(name).unwrap().int = iv.int;
    ctx.identifier_storage[scope].get_mut(name).unwrap().float = iv.float;
    ctx.identifier_storage[scope].get_mut(name).unwrap().bool = iv.bool;
    ctx.identifier_storage[scope].get_mut(name).unwrap().vector = iv.vector;
    ctx.identifier_storage[scope].get_mut(name).unwrap().map = iv.map;
    ctx.identifier_storage[scope].get_mut(name).unwrap().expression = iv.expression;
    ctx.identifier_storage[scope].get_mut(name).unwrap().user_defined_function = iv.user_defined_function;
    ctx.identifier_storage[scope].get_mut(name).unwrap().function = iv.function;
}

pub fn eval_factor(ctx: &mut Context, factor: &Factor) -> Result<Vec<Factor>, String> {
    let mut factors = Vec::new();
    if factor.kind == FactorKind::Expression {
        match eval(ctx, factor.expression.as_ref().unwrap()) {
            Ok(er) => {
                factors = er;
            },
            Err(e) => {
                return Err(e)
            },
        }
    } else {
        factors.push(factor.clone());
    }
    Ok(factors)
}

pub fn eval(ctx: &mut Context, expr: &Expression) -> Result<Vec<Factor>, String> {
    if expr.factors.len() == 0 {
        return Ok(Vec::new())
    }
    let mut func = &expr.factors[0];
    let mut factors = Vec::new();
    if func.kind == FactorKind::Expression {
        match eval(ctx, func.expression.as_ref().unwrap()) {
            Ok(er) => {
                for f in er {
                    factors.push(f);
                }
            },
            Err(e) => {
                return Err(e)
            },
        }
        if factors.len() == 0 {
            for n in 1..expr.factors.len() {
                factors.push(expr.factors[n].clone());
            }
            return Ok(factors)
        }
    } else if func.kind != FactorKind::Identifier {
        return Ok(expr.factors.clone())
    } else {
        factors.push(func.clone());
    }
    for n in 1..expr.factors.len() {
        factors.push(expr.factors[n].clone());
    }
    if factors.len() == 0 {
        return Ok(factors)
    }
    func = &factors[0];
    if func.kind != FactorKind::Identifier {
        return Ok(factors)
    }

    match search_identifier(ctx, func.name.as_ref().unwrap()) {
        Some(iv) => {
            if iv.1.kind != FactorKind::Function {
                return Ok(factors.clone())
            }
            match iv.1.function {
                Some(f) => {
                    return f(ctx, factors.clone())
                },
                None => match &iv.1.user_defined_function {
                    Some(udf) => {
                        let user_defined_function = udf.clone();
                        let backup_scope = ctx.scope.clone();
                        let mut args = Vec::new();
                        for n in 1..factors.len() {
                            match eval_factor(ctx, &factors[n]) {
                                Ok(fs) => args.extend_from_slice(&fs),
                                Err(e) => return Err(e),
                            }
                        }
                        ctx.scope = user_defined_function.scope;
                        let res = exec(ctx, &user_defined_function.statement, &args, Some(ScopeType::UserDefinedFunction));
                        ctx.scope = backup_scope;
                        match res {
                            Ok(execreturn) => return Ok(execreturn.factors),
                            Err(e) => return Err(e),
                        }
                    },
                    None => {
                        return Err("Identifier is not function".to_owned())
                    },
                },
            }
        },
        None => {
            return Ok(factors.clone())
        },
    };
}

pub fn exec(ctx: &mut Context, statement: &Statement, arguments: &[Factor], scope_type: Option<ScopeType>) -> Result<ExecReturn, String> {
    let mut is_loop = false;
    let mut scope_type_set = ScopeType::Block;
    if scope_type.is_some() {
        scope_type_set = scope_type.unwrap();
    }
    'root: loop {
        let mut res = Vec::new();
        match eval(ctx, &statement.expression) {
            Ok(er) => {
                res = er;
            },
            Err(e) => {
                return Err(e)
            },
        }
        // User Defined Function Definition
        if 2 <= res.len() &&
            res[0].kind == FactorKind::Identifier &&
                res[0].name.as_ref().unwrap() == define::FUNCTION_DEFINITION {
            let mut second_factor = res[1].clone();
            if second_factor.kind == FactorKind::Expression {
                match eval(ctx, second_factor.expression.as_ref().unwrap()) {
                    Ok(er) => {
                        if er.len() != 1 {
                            return Err("Function definition error".to_owned())
                        }
                        second_factor = er[0].clone();
                    },
                    Err(e) => {
                        return Err(e)
                    },
                }
            }
            if second_factor.kind != FactorKind::Identifier {
                return Err("lval must be identifier".to_owned())
            }
            let second_factor_name = second_factor.name.as_ref().unwrap();
            let current_scope = ctx.current_scope();
            if ctx.identifier_storage[current_scope].contains_key(second_factor_name) {
                return Err(define::REDEFINITION_NOT_SUPPORTED.to_owned())
            }
            let mut iv = Factor::new();
            iv.kind = FactorKind::Function;
            let mut params = Vec::new();
            if 2 < res.len() {
                if res[2].kind == FactorKind::Expression {
                    params = res[2].expression.as_ref().unwrap().factors.clone();
                }
            }
            iv.user_defined_function = Some(
                UserDefinedFunction {
                    scope: ctx.scope.clone(),
                    statement: Statement {
                        expression: Expression { factors: Vec::new() },
                        statements: statement.statements.clone(),
                        params: params,
                    }
                }
            );
            ctx.identifier_storage[current_scope].insert(
                second_factor_name.clone(),
                iv
            );
            return Ok(ExecReturn{
                result: ExecResult::UserDefinedFunctionDefinition,
                factors: vec![second_factor]
            })
        // if / loop statement
        } else if res.len() == 2 &&
            res[0].kind == FactorKind::Identifier &&
                (res[0].name.as_ref().unwrap() == define::IF ||
                res[0].name.as_ref().unwrap() == define::LOOP) {
            let if_loop_str = res[0].name.as_ref().unwrap();
            let mut second_factor = res[1].clone();
            if second_factor.kind == FactorKind::Expression {
                match eval(ctx, second_factor.expression.as_ref().unwrap()) {
                    Ok(er) => {
                        if er.len() != 1 {
                            return Err("Target value must be only one".to_owned())
                        }
                        second_factor = er[0].clone();
                    },
                    Err(e) => {
                        return Err(e)
                    },
                }
            }
            if second_factor.kind == FactorKind::Identifier {
                match search_identifier(ctx, second_factor.name.as_ref().unwrap()) {
                    Some(iv) => {
                        second_factor = iv.1.clone();
                    },
                    None => return Err(define::IDENTIFIER_NOT_DEFINED.to_owned()),
                }
            }
            if second_factor.kind == FactorKind::Bool {
                if second_factor.bool.unwrap() {
                    scope_type_set = ScopeType::If;
                    if if_loop_str == define::LOOP {
                        is_loop = true;
                        scope_type_set = ScopeType::Loop;
                    }
                } else {
                    if if_loop_str == define::LOOP {
                        return Ok(ExecReturn {
                            result: ExecResult::LoopFalse,
                            factors: Vec::new(),
                        })
                    } else {
                        return Ok(ExecReturn {
                            result: ExecResult::IfFalse,
                            factors: Vec::new(),
                        })
                    }
                }
            } else if second_factor.kind == FactorKind::Identifier {
                let second_factor_name = second_factor.name.as_ref().unwrap();
                match search_identifier(ctx, second_factor_name) {
                    Some(iv) => {
                        if iv.1.kind != FactorKind::Bool {
                            return Err("Target value is not bool".to_owned())
                        }
                        if iv.1.bool.unwrap() {
                            if if_loop_str == define::LOOP {
                                is_loop = true;
                            }
                        } else {
                            if if_loop_str == define::LOOP {
                                return Ok(ExecReturn {
                                    result: ExecResult::LoopFalse,
                                    factors: Vec::new(),
                                })
                            } else {
                                return Ok(ExecReturn {
                                    result: ExecResult::IfFalse,
                                    factors: Vec::new(),
                                })
                            }
                        }
                    },
                    None => return Err(define::IDENTIFIER_NOT_DEFINED.to_owned()),
                }
            } else {
                return Err("Target value is not bool".to_owned())
            }
        }

        // Normal Statement
        // Execute statements
        if 0 < statement.statements.len() {
            ctx.push_new(scope_type_set.clone());
            // Arguments
            let mut params = Vec::new();
            for param in &statement.params {
                match eval_factor(ctx, &param) {
                    Ok(p) => params.extend_from_slice(&p),
                    Err(e) => {
                        ctx.pop();
                        return Err(e)
                    },
                }
            }
            if arguments.len() != params.len() {
                ctx.pop();
                return Err(define::ARGUMENT_LENGTH_MISMATCH.to_owned())
            }
            for n in 0..arguments.len() {
                match builtin::assign_variable(ctx, &params[n], &arguments[n]) {
                    Ok(_) => {},
                    Err(e) => {
                        ctx.pop();
                        return Err(e)
                    }
                }
            }
            for s in &statement.statements {
                match exec(ctx, s, &Vec::new(), None) {
                    Ok(er) => {
                        if er.result == ExecResult::Return {
                            if ctx.scope_type[ctx.scope_type.len() - 1] == ScopeType::UserDefinedFunction {
                                ctx.pop();
                                return Ok(ExecReturn {
                                    result: ExecResult::Normal,
                                    factors: er.factors,
                                })
                            } else {
                                ctx.pop();
                                return Ok(ExecReturn {
                                    result: ExecResult::Return,
                                    factors: er.factors,
                                })
                            }
                        } else if er.result == ExecResult::LoopBreak {
                             if ctx.scope_type[ctx.scope_type.len() - 1] == ScopeType::Loop {
                                ctx.pop();
                                return Ok(ExecReturn {
                                    result: ExecResult::Normal,
                                    factors: er.factors,
                                })
                            } else {
                                ctx.pop();
                                return Ok(ExecReturn {
                                    result: ExecResult::LoopBreak,
                                    factors: er.factors,
                                })
                            }
                        } else if er.result == ExecResult::LoopContinue {
                             if ctx.scope_type[ctx.scope_type.len() - 1] == ScopeType::Loop {
                                ctx.pop();
                                continue 'root;
                            } else {
                                ctx.pop();
                                return Ok(ExecReturn {
                                    result: ExecResult::LoopContinue,
                                    factors: er.factors,
                                })
                            }
                        }
                        if 0 < er.factors.len() && er.factors[0].kind == FactorKind::Identifier {
                            let first_factor = &er.factors[0];
                            if first_factor.name.as_ref().unwrap() == define::RETURN {
                                if !ctx.scope_type.contains(&ScopeType::UserDefinedFunction) {
                                    ctx.pop();
                                    return Err("return outside function".to_owned())
                                }
                                let mut ret = Vec::new();
                                for n in 1..er.factors.len() {
                                    if er.factors[n].kind == FactorKind::Expression {
                                        match eval(ctx, er.factors[n].expression.as_ref().unwrap()) {
                                            Ok(er2) => {
                                                for f in er2 {
                                                    ret.push(f);
                                                }
                                            },
                                            Err(e) => {
                                                ctx.pop();
                                                return Err(e)
                                            }
                                        }
                                    } else {
                                        ret.push(er.factors[n].clone());
                                    }
                                }
                                ctx.pop();
                                return Ok(ExecReturn {
                                    result: ExecResult::Return,
                                    factors: ret,
                                })
                            } else if first_factor.name.as_ref().unwrap() == define::BREAK {
                                ctx.pop();
                                return Ok(ExecReturn {
                                    result: ExecResult::LoopBreak,
                                    factors: Vec::new(),
                                });
                            } else if first_factor.name.as_ref().unwrap() == define::CONTINUE {
                                if ctx.scope_type[ctx.scope_type.len() - 1] == ScopeType::Loop {
                                    ctx.pop();
                                    continue 'root;
                                } else {
                                    ctx.pop();
                                    return Ok(ExecReturn {
                                        result: ExecResult::LoopContinue,
                                        factors: Vec::new(),
                                    })
                                }
                            }
                        }
                        res = er.factors;
                    },
                    Err(e) => {
                        ctx.pop();
                        return Err(e)
                    }
                }
            }
            ctx.pop();
        }
        if !is_loop {
            return Ok(ExecReturn {
                result: ExecResult::Normal,
                factors: res,
            });
        }
    }
}

pub fn run(ctx: &mut Context, program: Vec<Statement>) -> Result<(), String> {
    for s in program {
        match exec(ctx, &s, &Vec::new(), None) {
            Ok(_) => {},
            Err(e) => {
                return Err(e)
            },
        }
    }
    Ok(())
}

pub fn init_identifier_storage() -> IdentifierStorage {
    let mut is = Vec::new();
    let mut scope0 = HashMap::new();

    let mut iv_decas = Factor::new();
    iv_decas.kind = FactorKind::Function;
    iv_decas.function = Some(builtin::define);
    scope0.insert(
        define::DECAS_ALIAS.to_owned(),
        iv_decas.clone(),
    );
    scope0.insert(
        define::DECAS.to_owned(),
        iv_decas.clone(),
    );
    let mut iv_assign = Factor::new();
    iv_assign.kind = FactorKind::Function;
    iv_assign.function = Some(builtin::assign);
    scope0.insert(
        define::ASSIGN.to_owned(),
        iv_assign,
    );

    let mut iv_print = Factor::new();
    iv_print.kind = FactorKind::Function;
    iv_print.function = Some(builtin::print);
    scope0.insert(
        define::PRINT.to_owned(),
        iv_print.clone(),
    );
    scope0.insert(
        define::PRINTLN.to_owned(),
        iv_print.clone(),
    );
    let mut iv_value = Factor::new();
    iv_value.kind = FactorKind::Function;
    iv_value.function = Some(builtin::value);
    scope0.insert(
        define::VALUE.to_owned(),
        iv_value,
    );

    let mut iv_make_vector = Factor::new();
    iv_make_vector.kind = FactorKind::Function;
    iv_make_vector.function = Some(builtin::make_vector);
    scope0.insert(
        define::MAKE_VECTOR.to_owned(),
        iv_make_vector,
    );

   let mut iv_as = Factor::new();
    iv_as.kind = FactorKind::Function;
    iv_as.function = Some(builtin::as_cast);
    scope0.insert(
        define::AS.to_owned(),
        iv_as,
    );

    let mut iv_arithmetic = Factor::new();
    iv_arithmetic.kind = FactorKind::Function;
    iv_arithmetic.function = Some(builtin::arithmetic);
    scope0.insert(
        define::ADD.to_owned(),
        iv_arithmetic.clone(),
    );
    scope0.insert(
        define::SUB.to_owned(),
        iv_arithmetic.clone(),
    );
    scope0.insert(
        define::MUL.to_owned(),
        iv_arithmetic.clone(),
    );
    scope0.insert(
        define::DIV.to_owned(),
        iv_arithmetic.clone(),
    );
    scope0.insert(
        define::REM.to_owned(),
        iv_arithmetic.clone(),
    );

    let mut iv_equal = Factor::new();
    iv_equal.kind = FactorKind::Function;
    iv_equal.function = Some(builtin::equal);
    scope0.insert(
        define::EQUAL.to_owned(),
        iv_equal,
    );

    let mut iv_string = Factor::new();
    iv_string.kind = FactorKind::TypeName;
    iv_string.string = Some(define::STRING.to_owned());
    scope0.insert(
        define::STRING.to_owned(),
        iv_string,
    );
    let mut iv_int = Factor::new();
    iv_int.kind = FactorKind::TypeName;
    iv_int.string = Some(define::INT.to_owned());
    scope0.insert(
        define::INT.to_owned(),
        iv_int,
    );
    let mut iv_float = Factor::new();
    iv_float.kind = FactorKind::TypeName;
    iv_float.string = Some(define::FLOAT.to_owned());
    scope0.insert(
        define::FLOAT.to_owned(),
        iv_float,
    );
    let mut iv_bool = Factor::new();
    iv_bool.kind = FactorKind::TypeName;
    iv_bool.string = Some(define::BOOL.to_owned());
    scope0.insert(
        define::BOOL.to_owned(),
        iv_bool,
    );
    let mut iv_function = Factor::new();
    iv_function.kind = FactorKind::TypeName;
    iv_function.string = Some(define::FUNCTION.to_owned());
    scope0.insert(
        define::FUNCTION.to_owned(),
        iv_function,
    );

    let mut iv_true = Factor::new();
    iv_true.kind = FactorKind::Bool;
    iv_true.bool = Some(true);
    scope0.insert(
        define::TRUE.to_owned(),
        iv_true,
    );
    let mut iv_false = Factor::new();
    iv_false.kind = FactorKind::Bool;
    iv_false.bool = Some(false);
    scope0.insert(
        define::FALSE.to_owned(),
        iv_false,
    );

    is.push(scope0);
    is
}
pub fn init_context() -> Context {
    let is = init_identifier_storage();
    let mut ctx = Context {
        scope_type: vec![ScopeType::Root],
        scope: vec![0],
        identifier_storage: is,
    };
    ctx.push_new(ScopeType::Program);
    ctx
}
