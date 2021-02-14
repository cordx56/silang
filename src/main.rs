mod silang;

fn main() {
    //assert_eq!(parser::factor("abc"), Ok(("", parser::Factor { kind: parser::FactorKind::Identifier, identifier: "abc".to_owned(), string: "".to_owned(), number: 0.0 })));
    println!("Expression:");
    println!("{:?}", silang::parser::expression("(a b (c d))"));
    println!("Statement:");
    println!("{:?}", silang::parser::statement("(a b (c d))\n"));
    println!("Program:");
    println!("{:?}", silang::parser::program(r#"
{
    b = 1
    return b
}

println (a)

"#));

    let mut is = silang::run::init_identifier_storage();
    let mut ctx = silang::Context {
        scope: 0,
        identifier_storage: &mut is,
    };
    println!("{:?}", silang::run::eval(&mut ctx, silang::parser::expression(":: (a b c) (int int int)").unwrap().1));
    //println!("{:?}", silang::run::eval(&mut ctx, silang::parser::expression("= a e").unwrap().1));
}
