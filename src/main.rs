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
        .author("Kaoru Chisen <cordx56@cordx.net>")
        .about("Run SILang code")
        .arg(Arg::with_name("FILE")
             .help("Input file to run"))
        .arg(Arg::with_name("parseTree")
             .long("parseTree")
             .help("Print parse tree")
             .takes_value(false))
        .get_matches();

    let mut ctx = silang::run::init_context();

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
                    if matches.is_present("parseTree") {
                        println!("{}", silang::parser::parse_tree(program.1));
                    } else {
                        silang::run::run(&mut ctx, program.1).ok();
                    }
                },
                Err(e) => {
                    eprintln!("Parse error");
                    eprintln!("{}", e);
                },
            }
        },
        None => {
            println!("SILang Interpreter Ver:{}", VERSION);
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
                        if matches.is_present("parseTree") {
                            println!("{}", silang::parser::parse_tree_statement(s.1, 0));
                        } else {
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
