use anyhow::{self, Result};
use std::{env, fs, process};
use yaml_database_schema_parser::*;

fn print_help() {
    println!("YAML Schema Parser CLI");
    println!("Usage:");
    println!("  cargo run -- <command> <file_name>");
    println!("Commands:");
    println!("  parse <file_name>         Parses the specified schema file.");
    println!("  help                      Displays this help message.");
    println!("  credits                   Shows credits and author information.");
}

fn print_credits() {
    println!("YAML Schema Parser CLI");
    println!("Developed by Volodymyr Materynskyi");
    println!("Built using the Rust programming language.");
    println!("Powered by the materynskyis_yaml_schema_parser library.");
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: No command provided.");
        print_help();
        process::exit(1);
    }

    match args[1].as_str() {
        "parse" => {
            if args.len() < 3 {
                eprintln!("Error: No file name provided for 'parse' command.");
                process::exit(1);
            }
            let file_name = &args[2];
            let content = fs::read_to_string(file_name);
            match content {
                Ok(file_content) => match parse_schema(&file_content) {
                    Ok(schema) => println!("{:#?}", schema),
                    Err(SchemaErr::ParseError(e)) => {
                        eprintln!("Error while parsing the file: {}", e)
                    }
                    Err(SchemaErr::MissingValue) => {
                        eprintln!("Error: Missing value in schema.")
                    }
                },
                Err(e) => eprintln!("Failed to read file '{}': {}", file_name, e),
            }
        }
        "help" => print_help(),
        "credits" => print_credits(),
        _ => {
            eprintln!("Error: Unrecognized command '{}'", args[1]);
            print_help();
            process::exit(1);
        }
    }

    Ok(())
}
