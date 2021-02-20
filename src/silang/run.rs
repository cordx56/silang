use super::{
    IdentifierType,
    UserDefinedFunction,
    IdentifierValue,
    FactorKind,
    Factor,
    Expression,
    Statement,
    IdentifierStorage,
    Context,
};

use super::builtin;
use super::define;

use std::collections::HashMap;

pub fn search_identifier<'a>(ctx: &'a mut Context, name: &str) -> Option<(usize, &'a IdentifierValue)> {
    let mut scope = ctx.scope;
    loop {
        if ctx.identifier_storage[scope].contains_key(name) {
            return Some((scope, &ctx.identifier_storage[scope][name]))
        }
        if scope == 0 {
            return None
        }
        scope -= 1;
    }
}

pub fn assign_identifier(ctx: &mut Context, scope: usize, name: &str, iv: IdentifierValue) {
    ctx.identifier_storage[scope].get_mut(name).unwrap().identifier_type = iv.identifier_type;
    ctx.identifier_storage[scope].get_mut(name).unwrap().string = iv.string;
    ctx.identifier_storage[scope].get_mut(name).unwrap().int = iv.int;
    ctx.identifier_storage[scope].get_mut(name).unwrap().float = iv.float;
    ctx.identifier_storage[scope].get_mut(name).unwrap().bool = iv.bool;
    ctx.identifier_storage[scope].get_mut(name).unwrap().user_defined_function = iv.user_defined_function;
    ctx.identifier_storage[scope].get_mut(name).unwrap().function = iv.function;
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
            if iv.1.identifier_type != IdentifierType::Function {
                return Ok(expr.factors.clone())
            }
            match iv.1.function {
                Some(f) => {
                    return f(ctx, expr.factors.clone())
                },
                None => match &iv.1.user_defined_function {
                    Some(udf) => {
                        let udf_scope = udf.scope;
                        let udf_statement = udf.statement.clone();
                        match exec(ctx, &udf_statement) {
                            Ok(er) => {
                                return Ok(er);
                            },
                            Err(e) => {
                                return Err(e)
                            },
                        }
                    },
                    None => {
                        return Err("Identifier is not function".to_owned())
                    },
                },
            }
        },
        None => {
            return Ok(expr.factors.clone())
        },
    };
    //let ctx_scope = ctx.scope;
    //ctx.scope = udf_scope;
    //ctx.scope = ctx_scope;
}

pub fn exec(ctx: &mut Context, statement: &Statement) -> Result<Vec<Factor>, String> {
    let mut is_loop = false;
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
        if res.len() == 2 &&
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
            if ctx.identifier_storage[ctx.scope].contains_key(second_factor_name) {
                return Err(define::REDEFINITION_NOT_SUPPORTED.to_owned())
            }
            ctx.identifier_storage[ctx.scope].insert(
                second_factor_name.clone(),
                IdentifierValue{
                    identifier_type: IdentifierType::Function,
                    string: None,
                    int: None,
                    float: None,
                    bool: None,
                    user_defined_function: Some(
                        UserDefinedFunction {
                            scope: ctx.scope + 1,
                            statement: Statement {
                                expression: Expression { factors: Vec::new() },
                                statements: statement.statements.clone(),
                            }
                        }
                    ),
                    function: None,
                }
            );
            return Ok(vec![second_factor])
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
            if second_factor.kind != FactorKind::Identifier {
                return Err("Target value must be a identifier".to_owned())
            }
            let second_factor_name = second_factor.name.as_ref().unwrap();
            match search_identifier(ctx, second_factor_name) {
                Some(iv) => {
                    if iv.1.identifier_type != IdentifierType::Bool {
                        return Err("Target value is not bool".to_owned())
                    }
                    if iv.1.bool.unwrap() {
                        if if_loop_str == define::LOOP {
                            is_loop = true;
                        }
                    } else {
                        return Ok(Vec::new())
                    }
                },
                None => {
                    return Err(define::IDENTIFIER_NOT_DEFINED.to_owned())
                },
            }
        }

        // Normal Statement
        // Execute statements
        if 0 < statement.statements.len() {
            ctx.push_new();
            for s in &statement.statements {
                match exec(ctx, s) {
                    Ok(er) => {
                        if 0 < er.len() && er[0].kind == FactorKind::Identifier {
                            let first_factor = &er[0];
                            if first_factor.name.as_ref().unwrap() == define::RETURN {
                                let mut ret = Vec::new();
                                for n in 1..er.len() {
                                    if er[n].kind == FactorKind::Expression {
                                        match eval(ctx, er[n].expression.as_ref().unwrap()) {
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
                                        ret.push(er[n].clone());
                                    }
                                }
                                ctx.pop();
                                return Ok(ret)
                            } else if first_factor.name.as_ref().unwrap() == define::BREAK {
                                ctx.pop();
                                return Ok(Vec::new());
                            } else if first_factor.name.as_ref().unwrap() == define::CONTINUE {
                                ctx.pop();
                                continue 'root;
                            }
                        }
                        res = er;
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
            return Ok(res);
        }
    }
}

pub fn run(ctx: &mut Context, program: Vec<Statement>) -> Result<(), String> {
    for s in program {
        match exec(ctx, &s) {
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

    scope0.insert(
        define::DECAS_ALIAS.to_owned(),
        IdentifierValue {
            identifier_type: IdentifierType::Function,
            string: None,
            int: None,
            float: None,
            bool: None,
            user_defined_function: None,
            function: Some(builtin::define_variable),
        }
    );
    scope0.insert(
        define::DECAS.to_owned(),
        IdentifierValue {
            identifier_type: IdentifierType::Function,
            string: None,
            int: None,
            float: None,
            bool: None,
            user_defined_function: None,
            function: Some(builtin::define_variable),
        }
    );
    scope0.insert(
        define::ASSIGN.to_owned(),
        IdentifierValue {
            identifier_type: IdentifierType::Function,
            string: None,
            int: None,
            float: None,
            bool: None,
            user_defined_function: None,
            function: Some(builtin::assign_variable),
        }
    );

    scope0.insert(
        define::PRINT.to_owned(),
        IdentifierValue {
            identifier_type: IdentifierType::Function,
            string: None,
            int: None,
            float: None,
            bool: None,
            user_defined_function: None,
            function: Some(builtin::print),
        }
    );
    scope0.insert(
        define::PRINTLN.to_owned(),
        IdentifierValue {
            identifier_type: IdentifierType::Function,
            string: None,
            int: None,
            float: None,
            bool: None,
            user_defined_function: None,
            function: Some(builtin::print),
        }
    );
    scope0.insert(
        define::VALUE.to_owned(),
        IdentifierValue {
            identifier_type: IdentifierType::Function,
            string: None,
            int: None,
            float: None,
            bool: None,
            user_defined_function: None,
            function: Some(builtin::value),
        }
    );

    scope0.insert(
        define::AS.to_owned(),
        IdentifierValue {
            identifier_type: IdentifierType::Function,
            string: None,
            int: None,
            float: None,
            bool: None,
            user_defined_function: None,
            function: Some(builtin::as_cast),
        }
    );

    scope0.insert(
        define::ADD.to_owned(),
        IdentifierValue {
            identifier_type: IdentifierType::Function,
            string: None,
            int: None,
            float: None,
            bool: None,
            user_defined_function: None,
            function: Some(builtin::arithmetic),
        }
    );
    scope0.insert(
        define::SUB.to_owned(),
        IdentifierValue {
            identifier_type: IdentifierType::Function,
            string: None,
            int: None,
            float: None,
            bool: None,
            user_defined_function: None,
            function: Some(builtin::arithmetic),
        }
    );
    scope0.insert(
        define::MUL.to_owned(),
        IdentifierValue {
            identifier_type: IdentifierType::Function,
            string: None,
            int: None,
            float: None,
            bool: None,
            user_defined_function: None,
            function: Some(builtin::arithmetic),
        }
    );
    scope0.insert(
        define::DIV.to_owned(),
        IdentifierValue {
            identifier_type: IdentifierType::Function,
            string: None,
            int: None,
            float: None,
            bool: None,
            user_defined_function: None,
            function: Some(builtin::arithmetic),
        }
    );
    scope0.insert(
        define::REM.to_owned(),
        IdentifierValue {
            identifier_type: IdentifierType::Function,
            string: None,
            int: None,
            float: None,
            bool: None,
            user_defined_function: None,
            function: Some(builtin::arithmetic),
        }
    );

    scope0.insert(
        define::STRING.to_owned(),
        IdentifierValue {
            identifier_type: IdentifierType::TypeName,
            string: Some(define::STRING.to_owned()),
            int: None,
            float: None,
            bool: None,
            user_defined_function: None,
            function: None,
        }
    );
    scope0.insert(
        define::INT.to_owned(),
        IdentifierValue {
            identifier_type: IdentifierType::TypeName,
            string: Some(define::INT.to_owned()),
            int: None,
            float: None,
            bool: None,
            user_defined_function: None,
            function: None,
        }
    );
    scope0.insert(
        define::FLOAT.to_owned(),
        IdentifierValue {
            identifier_type: IdentifierType::TypeName,
            string: Some(define::FLOAT.to_owned()),
            int: None,
            float: None,
            bool: None,
            user_defined_function: None,
            function: None,
        }
    );
    scope0.insert(
        define::BOOL.to_owned(),
        IdentifierValue {
            identifier_type: IdentifierType::TypeName,
            string: Some(define::BOOL.to_owned()),
            int: None,
            float: None,
            bool: None,
            user_defined_function: None,
            function: None,
        }
    );
    scope0.insert(
        define::FUNCTION.to_owned(),
        IdentifierValue {
            identifier_type: IdentifierType::TypeName,
            string: Some(define::FUNCTION.to_owned()),
            int: None,
            float: None,
            bool: None,
            user_defined_function: None,
            function: None,
        }
    );

    scope0.insert(
        define::TRUE.to_owned(),
        IdentifierValue {
            identifier_type: IdentifierType::Bool,
            string: None,
            int: None,
            float: None,
            bool: Some(true),
            user_defined_function: None,
            function: None,
        }
    );
    scope0.insert(
        define::FALSE.to_owned(),
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

    is.push(scope0);
    is
}
pub fn init_context() -> Context {
    let mut is = init_identifier_storage();
    is.push(HashMap::new());
    Context {
        scope: 1,
        identifier_storage: is,
    }
}
