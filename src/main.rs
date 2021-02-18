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

static VERSION: &str = "0.2.0-beta";

fn main() {
    let matches = App::new("SILang Interpreter")
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

    let mut buffer = String::new();
    match matches.value_of("FILE") {
        Some(i) => {
            if i == "-" {
                let stdin = io::stdin();
                let mut handle = stdin.lock();
                handle.read_to_string(&mut buffer).ok();
            } else {
                match fs::read_to_string(i) {
                    Ok(s) => {
                        buffer = s;
                    },
                    Err(e) => {
                        eprintln!("File read error");
                        eprintln!("{}", e);
                        return
                    },
                }
            }
            buffer.push_str("\n");

            let parse_result = silang::parser::program_all_consuming(&buffer);
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
        },
        None => {
            println!("SILang Interpreter Ver.{}", VERSION);
            loop {
                if 0 < buffer.len() {
                    print!(". ");
                } else {
                    print!("> ");
                }
                std::io::stdout().flush().ok();
                std::io::stdin().read_line(&mut buffer).ok();
                if buffer.len() == 0 {
                    break;
                }
                match silang::parser::statement_all_consuming(&buffer) {
                    Ok (s) => {
                        match silang::run::exec(&mut ctx, &s.1) {
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
        },
    }
}
