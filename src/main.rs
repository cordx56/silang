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

fn main() {
    let matches = App::new("SILang Interpreter")
        .version(silang::define::VERSION)
        .author("Kaoru Chisen <cordx56@cordx.net>")
        .about("Run SILang code")
        .arg(Arg::with_name("FILE")
             .help("Input file to run"))
        .arg(Arg::with_name("parseTree")
             .long("parseTree")
             .help("Print parse tree")
             .takes_value(false))
        .get_matches();

    let mut interpreter = silang::Interpreter::new();

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

            match silang::preprocessor::preprocess(&buffer) {
                Ok(source_code) => {
                    let parse_result = silang::parser::program_all_consuming(&source_code);
                    // eprintln!("{:?}", parse_result);
                    match parse_result {
                        Ok(program) => {
                            if matches.is_present("parseTree") {
                                println!("{}", silang::parser::parse_tree_program(&program.1, 0));
                            } else {
                                match interpreter.run(&program.1) {
                                    Ok(_) => {},
                                    Err(e) => eprintln!("{}", e),
                                }
                            }
                        },
                        Err(error) => {
                            eprintln!("Parse error");
                            match error {
                                nom::Err::Error(e) => {
                                    let input: &str = &buffer;
                                    eprintln!("{}", nom::error::convert_error(input, e))
                                },
                                _ => {},
                            }
                        },
                    }
                },
                Err(e) => {
                    eprintln!("Preprocess error\n{}", e);
                }
            }
        },
        None => {
            println!("SILang Interpreter Ver:{}", silang::define::VERSION);
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
                            println!("{}", silang::parser::parse_tree_statement(&s.1, 0));
                        } else {
                            match interpreter.exec(&s.1) {
                                Ok(result) => {
                                    for v in result.values {
                                        interpreter.print_value(&v).ok();
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
