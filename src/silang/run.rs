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

pub fn search_identifier<'a>(ctx: &'a Context<'a>, name: &str) -> Option<&'a IdentifierValue<'a>> {
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
    /*
    match search_identifier(ctx, func.name.as_ref().unwrap()) {
        Some(iv) => {
            if iv.identifier_type != IdentifierType::Function {
                return Err("Identifier is not function!")
            }
            match iv.function {
                Some(f) => Ok(Vec::new()),
                None => match &iv.statement {
                    Some(s) => run(&mut Context {
                        scope: s.scope,
                        identifier_storage: ctx.identifier_storage
                    }, s.statement.clone()),
                    None => Err("Identifier is not function!"),
                },
            }
        },
        None => Err("Identifier not found!"),
    }*/
    Err("Not implemented!")
}

pub fn run<'a>(ctx: &mut Context<'a>, statement: Statement) -> Result<Vec<Factor>, &'a str> {
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
            statement: None,
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
            statement: None,
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
            statement: None,
            function: Some(builtin::assign_variable),
        }
    );

    is.push(scope0);
    is
}
