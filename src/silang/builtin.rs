use super::{
    IdentifierType,
    IdentifierValue,
    FactorKind,
    Factor,
    Expression,
    Context,
};

pub fn define_variable<'a>(ctx: &'a mut Context<'a>, factors: Vec<Factor>) -> Result<Vec<Factor>, &str> {
    if factors.len() != 3 {
        return Err("Argument length must be 2")
    }
    let lval = &factors[1];
    let rval = &factors[2];
    let mut type_name_vec = Vec::new();
    if rval.kind == FactorKind::Expression {
        for f in &rval.expression.as_ref().unwrap().factors {
            if f.kind != FactorKind::Identifier {
                return Err("rval must be identifier")
            }
            type_name_vec.push(f.name.as_ref().unwrap().to_owned());
        }
    } else {
        if rval.kind != FactorKind::Identifier {
            return Err("rval must be identifier")
        }
        type_name_vec.push(rval.name.as_ref().unwrap().to_owned());
    }

    let mut ident_name_vec = Vec::new();
    if lval.kind == FactorKind::Expression {
        for f in &lval.expression.as_ref().unwrap().factors {
            if f.kind != FactorKind::Identifier {
                return Err("lval must be identifier")
            }
            ident_name_vec.push(f.name.as_ref().unwrap().to_owned());
        }
    } else {
        if lval.kind != FactorKind::Identifier {
            return Err("lval must be identifier")
        }
        ident_name_vec.push(lval.name.as_ref().unwrap().to_owned());
    }

    if ident_name_vec.len() != type_name_vec.len() {
        return Err("lval and rval length must be equal")
    }
    for n in 0..ident_name_vec.len() {
        let mut identifier_type = IdentifierType::None;
        if type_name_vec[n] == "string" {
            identifier_type = IdentifierType::String;
        } else if type_name_vec[n] == "int" {
            identifier_type = IdentifierType::Int;
        } else if type_name_vec[n] == "float" {
            identifier_type = IdentifierType::Float;
        } else {
            return Err("Unknown type")
        }
        ctx.identifier_storage[ctx.scope].insert(
            ident_name_vec[n].clone(),
            IdentifierValue {
                identifier_type: identifier_type,
                string: None,
                int: None,
                float: None,
                statement: None,
                function: None,
            }
        );
    }
    Ok(vec![factors[1].clone()])
}

pub fn assign_variable<'a>(ctx: &'a mut Context<'a>, factors: Vec<Factor>) -> Result<Vec<Factor>, &str> {
    Err("Not implemented!")
}
