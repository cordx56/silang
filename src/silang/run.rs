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

pub fn search_identifier<'a>(ctx: &'a mut Context, name: &str) -> Option<&'a IdentifierValue> {
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

pub fn eval(ctx: &mut Context, expr: Expression) -> Result<Vec<Factor>, &str> {
    if expr.factors.len() == 0 {
        return Ok(Vec::new())
    }
    let func = &expr.factors[0];
    if func.kind != FactorKind::Identifier {
        return Ok(expr.factors)
    }

    let mut udf_scope: usize = 0;
    let mut udf_statement = Statement {
        expression: Expression {
            factors: Vec::new(),
        },
        statements: Vec::new(),
    };
    match search_identifier(ctx, func.name.as_ref().unwrap()) {
        Some(iv) => {
            if iv.identifier_type != IdentifierType::Function {
                return Ok(expr.factors)
            }
            match iv.function {
                Some(f) => {
                    return f(ctx, expr.factors)
                },
                None => match &iv.user_defined_function {
                    Some(udf) => {
                        udf_scope = udf.scope;
                        udf_statement = udf.statement.clone();
                    },
                    None => {
                        return Err("Identifier is not function!")
                    },
                },
            }
        },
        None => {
            return Ok(expr.factors)
        },
    };
    let ctx_scope = ctx.scope;
    ctx.scope = udf_scope;
    let result = run(ctx, udf_statement);
    ctx.scope = ctx_scope;
    //result
    Ok(Vec::new())
}

pub fn run(ctx: &mut Context, statement: Statement) -> Result<Vec<Factor>, &str> {
    match eval(ctx, statement.expression) {
        Ok(er) => {
            if 0 < er.len() && er[0].kind == FactorKind::Identifier && er[0].name.as_ref().unwrap() == "return" {
            }
        },
        Err(e) => {
            return Err("TODO: Error message")
        },
    }
    let mut res = Vec::new();
    for s in statement.statements {
        match run(ctx, s) {
            Ok(er) => {
                res = er;
            },
            Err(e) => {
                return Err("TODO: Error message")
            }
        }
    }
    Ok(res)
}


pub fn init_identifier_storage() -> IdentifierStorage {
    let mut is = Vec::new();
    let mut scope0 = HashMap::new();

    scope0.insert(
        "::".to_owned(),
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
        "=".to_owned(),
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
        "int".to_owned(),
        IdentifierValue {
            identifier_type: IdentifierType::TypeName,
            string: None,
            int: None,
            float: None,
            bool: None,
            user_defined_function: None,
            function: None,
        }
    );

    is.push(scope0);
    is
}
