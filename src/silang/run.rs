use super::{
    IdentifierType,
    IdentifierValue,
    FactorKind,
    Factor,
    Expression,
    Statement,
    IdentifierStorage,
    Context,
};

use super::builtin;

use std::collections::HashMap;

pub fn search_identifier<'a>(ctx: &'a mut Context<'a>, name: &str) -> Option<&'a IdentifierValue<'a>> {
    let mut scope = ctx.scope;
    loop {
        if ctx.identifier_storage[scope].contains_key(name) {
            return Some(&ctx.identifier_storage[scope][name])
        }
        if scope == 0 {
            return None
        }
        scope -= 1;
    }
}

pub fn eval<'a>(ctx: &'a mut Context<'a>, expr: Expression) -> Result<Vec<Factor>, &str> {
    if expr.factors.len() == 0 {
        return Ok(Vec::new())
    }
    let func = &expr.factors[0];
    if func.kind != FactorKind::Identifier {
        return Err("First factor is not identifier!")
    }

    let mut scope = ctx.scope;
    let mut iv = &IdentifierValue {
        identifier_type: IdentifierType::None,
        string: None,
        int: None,
        float: None,
        user_defined_function: None,
        function: None,
    };
    loop {
        if ctx.identifier_storage[scope].contains_key(func.name.as_ref().unwrap()) {
            iv = &ctx.identifier_storage[scope][func.name.as_ref().unwrap()]
        }
        if scope == 0 {
            break;
        }
    }
    if iv.identifier_type != IdentifierType::Function {
        return Ok(expr.factors)
    }
    match iv.function {
        Some(f) => f(ctx, expr.factors),
        None => match &iv.user_defined_function {
            Some(udf) => Err("Not implemented!"),//run(ctx, &udf.statement),
            None => Err("Identifier is not function!"),
        }
    }
    /*
    match search_identifier(ctx, func.name.as_ref().unwrap()) {
        Some(iv) => {
            if iv.identifier_type != IdentifierType::Function {
                return Err("Identifier is not function!")
            }
            match iv.function {
                Some(f) => Err("Not implemented!"),//f(ctx, expr.factors),
                None => match &iv.user_defined_function {
                    Some(udf) => Err("Not implemented!"),//run(ctx, &udf.statement),
                    None => Err("Identifier is not function!"),
                },
            }
        },
        None => Err("Identifier not found!"),
    }*/
}

pub fn run<'a>(ctx: &mut Context<'a>, statement: &Statement) -> Result<Vec<Factor>, &'a str> {
    Err("Not implemented!")
}


pub fn init_identifier_storage<'a>() -> IdentifierStorage<'a> {
    let mut is = Vec::new();
    let mut scope0 = HashMap::new();

    scope0.insert(
        "::".to_owned(),
        IdentifierValue {
            identifier_type: IdentifierType::Function,
            string: None,
            int: None,
            float: None,
            user_defined_function: None,
            function: Some(builtin::define_variable),
        }
    );
    scope0.insert(
        "=".to_owned(),
        IdentifierValue {
            identifier_type: IdentifierType::Function,
            string: None,
            int: None,
            float: None,
            user_defined_function: None,
            function: Some(builtin::assign_variable),
        }
    );

    scope0.insert(
        "int".to_owned(),
        IdentifierValue {
            identifier_type: IdentifierType::TypeName,
            string: None,
            int: None,
            float: None,
            user_defined_function: None,
            function: None,
        }
    );

    is.push(scope0);
    is
}
