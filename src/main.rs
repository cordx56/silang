mod silang;

use std::io::{
    self,
    Read,
};

fn main() {
    let mut is = silang::run::init_identifier_storage();
    let mut ctx = silang::Context {
        scope: 0,
        identifier_storage: is,
    };

    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer);
    let parse_result = silang::parser::program(&buffer);
    // println!("{:?}", parse_result);
    silang::run::run(&mut ctx, parse_result.unwrap().1);
}
