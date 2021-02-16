mod silang;

use std::fs;
use std::io::{
    self,
    Read,
    Write,
};

extern crate clap;
use clap::{
    Arg,
    App,
};

static VERSION: &str = "0.2.0";

fn main() {
    let matches = App::new("SILang interpreter")
        .version(VERSION)
        .author("Kaoru Saso <cordx56@cordx.net>")
        .about("Run SILang code")
        .arg(Arg::with_name("FILE")
             .help("Input file to run"))
        .get_matches();

    let mut ctx = silang::Context {
        scope: 0,
        identifier_storage: silang::run::init_identifier_storage(),
    };

    let mut input_file = String::new();
    let mut buffer = String::new();
    match matches.value_of("FILE") {
        Some(i) => {
            input_file = i.to_owned();
        },
        None => {
            println!("SILang Interpreter Ver.{}", VERSION);
            loop {
                print!("> ");
                std::io::stdout().flush().ok();
                std::io::stdin().read_line(&mut buffer).ok();
                match silang::parser::statement(&buffer) {
                    Ok (s) => {
                        match silang::run::exec(&mut ctx, s.1) {
                            Ok(fs) => {
                                for f in fs {
                                    silang::builtin::print_factor(&mut ctx, f).ok();
                                    print!(" ");
                                }
                                println!("");
                            },
                            Err(e) => {
                                eprintln!("{}", e);
                            },
                        }
                        buffer = String::new();
                    },
                    Err(_) => {
                        continue;
                    },
                }
            }
        }
    }

    if input_file == "-" {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_to_string(&mut buffer).ok();
    } else {
        match fs::read_to_string(input_file) {
            Ok(s) => {
                buffer = s;
            },
            Err(_) => {
                eprintln!("File read error");
                return
            },
        }
    }

    let parse_result = silang::parser::program(&buffer);
    // println!("{:?}", parse_result);
    match parse_result {
        Ok(program) => {
            silang::run::run(&mut ctx, program.1).ok();
        },
        Err(e) => {
            eprintln!("Parse error");
            eprintln!("{}", e);
        },
    }
}
