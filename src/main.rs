extern crate getopts;
use getopts::Options;
use std::env;
use std::io;
use std::io::prelude::*;
extern crate regex;
use regex::Regex;

fn filter_dump(include_tables: Option<String>, exclude_tables: Option<String>, copy_only: bool,
               truncate: bool) {
    let stdin = io::stdin();
    let copy_re = Regex::new(r"^COPY (\w+) \(([\w, ]+)\) FROM stdin;").unwrap();
    
    let mut in_copy_statement = false;
    let mut output_copy = false;

    let include_tables_re = match include_tables {
        Some(re) => Regex::new(&re).unwrap(),
        None => Regex::new("").unwrap()
    };
    let exclude_tables_re = match exclude_tables {
        Some(re) => Regex::new(&re).unwrap(),
        None => Regex::new("0").unwrap()
    };

    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                if in_copy_statement {
                    if line == "\\." {
                        in_copy_statement = false;
                    }
                    if output_copy {
                        println!("{}", line)
                    }
                }
                else {
                    match copy_re.captures(&line) {
                        None => {
                            if !copy_only {
                                println!("{}", line);
                            }
                        },
                        Some(caps) => {
                            let current_table = String::from(&caps[1]);
                            if include_tables_re.is_match(&current_table) &&
                                !exclude_tables_re.is_match(&current_table) {
                                    output_copy = true;
                                    if truncate {
                                        println!("TRUNCATE table {};", current_table);
                                    }
                                    println!("{}", line);
                                }
                            else {
                                output_copy = false;
                            };
                            in_copy_statement = true;
                        },
                    };
                }
            },
            Err(_) => continue,
        }
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("t", "table", "output only tables matching NAME", "NAME");
    opts.optopt("T", "exclude-table", "no not output tables matching NAME", "NAME");
    opts.optflag("c", "copy-only", "output only COPY FROM statements");
    opts.optflag("r", "truncate", "empty tables before COPY FROM statements");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => {
            print_usage(&program, opts);
            panic!(f.to_string())
        }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let include_tables = matches.opt_str("t");
    let exclude_tables = matches.opt_str("T");
    let copy_only = matches.opt_present("c");
    let truncate = matches.opt_present("r");
    filter_dump(include_tables, exclude_tables, copy_only, truncate);
}
