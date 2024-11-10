use anyhow::{self, Result};
use materynskyis_yaml_schema_parser::*;
use std::{env, fs, process};

fn print_help() {
    println!("YAML schema parser CLI");
    println!("  cargo run <file_name>         Parses schema.");
    println!("  cargo run -- --help        To get instructions.");
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: No file name provided.");
        process::exit(1);
    }

    match args[1].as_str() {
        "--help" => print_help(),
        file_name => {
            let content = fs::read_to_string(file_name);
            match content {
                Ok(file_content) => match parse_schema(&file_content) {
                    Ok(schema) => println!("{:#?}", schema),
                    Err(SchemaErr::ParseError(e)) => {
                        eprintln!("Error while parsing the file: {}", e)
                    }
                    Err(materynskyis_yaml_schema_parser::SchemaErr::MissingValue) => {
                        eprintln!("Missing value");
                    }
                },
                Err(e) => eprintln!("Failed to read file '{}': {}", file_name, e),
            }
        }
    }

    Ok(())
}
