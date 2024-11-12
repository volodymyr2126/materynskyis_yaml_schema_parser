#![doc = include_str!("../docs.md")]

use ::pest_derive::Parser;
use pest::iterators::Pair;
use pest::Parser;
use thiserror::Error;

/// The main YAML parser for database schema files.
/// This parser reads and validates YAML files that define a database schema
/// structure, including schema name, tables, and columns.
#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct YAMLParser;

/// Represents a database schema containing multiple tables.
#[derive(Debug)]
pub struct Schema {
    /// Name of the schema
    name: String,
    /// List of tables included in the schema
    tables: Vec<Table>,
}

/// Represents a table within the schema, containing a list of columns.
#[derive(Debug)]
pub struct Table {
    /// The name of the table
    name: String,
    /// List of columns in the table
    columns: Vec<Column>,
}

/// Represents a column within a table, including its type and mode.
#[derive(Debug)]
pub struct Column {
    /// The name of the column
    name: String,
    /// The data type of the column (e.g., `int`, `str`, `float`, `bool`)
    datatype: String,
    /// The mode of the column, specifying its constraints (e.g., `nullable`, `required`, `repeated`)
    mode: String,
}

/// Enum for handling errors during schema parsing.
#[derive(Debug, Error)]
pub enum SchemaErr {
    /// Represents a parsing error, containing a message describing the issue.
    #[error("Error while parsing yaml schema")]
    ParseError(String),
    /// Indicates a missing required value during parsing.
    #[error("Missing value while parsing")]
    MissingValue,
}

/// Parses a YAML string into a `Schema` structure.
///
/// This function takes the entire input as a string and applies the `YAMLParser`
/// on it. It navigates through the grammar rules to extract schema, tables, and
/// columns data, converting them into Rust structs.
///
/// # Arguments
///
/// * `input` - A YAML string defining the schema structure.
///
/// # Errors
///
/// Returns a `SchemaErr` if the input does not match the expected structure
/// or if required values are missing.
pub fn parse_schema(input: &str) -> Result<Schema, SchemaErr> {
    let parse_result =
        YAMLParser::parse(Rule::file, input).map_err(|e| SchemaErr::ParseError(e.to_string()))?;

    let mut schema = Schema {
        name: String::new(),
        tables: Vec::new(),
    };
    for pair in parse_result {
        if pair.as_rule() == Rule::file {
            for inner_pair in pair.into_inner() {
                if inner_pair.as_rule() == Rule::schema_section {
                    for schema_entry in inner_pair.into_inner() {
                        if schema_entry.as_rule() == Rule::schema_entry {
                            for schema_attr in schema_entry.into_inner() {
                                match schema_attr.as_rule() {
                                    Rule::identifier => {
                                        schema.name = schema_attr.as_str().to_string();
                                    }
                                    Rule::table_section => {
                                        schema.tables = parse_tables(schema_attr)?;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(schema)
}

/// Parses a `table_section` from the input, returning a list of `Table` structs.
///
/// # Arguments
///
/// * `pair` - A `Pair` representing the `table_section` in the grammar.
///
/// # Returns
///
/// A `Result` containing a vector of `Table` structs or an error if parsing fails.
fn parse_tables(pair: Pair<Rule>) -> Result<Vec<Table>, SchemaErr> {
    let mut tables = Vec::new();

    for table_entry in pair.into_inner() {
        if table_entry.as_rule() == Rule::table_entry {
            let mut table = Table {
                name: String::new(),
                columns: Vec::new(),
            };

            for sub_entry in table_entry.into_inner() {
                match sub_entry.as_rule() {
                    Rule::identifier => {
                        table.name = sub_entry.as_str().to_string();
                    }
                    Rule::column_section => {
                        table.columns = parse_columns(sub_entry)?;
                    }
                    _ => {}
                }
            }
            tables.push(table);
        }
    }

    Ok(tables)
}

/// Parses a `column_section` from the input, returning a list of `Column` structs.
///
/// # Arguments
///
/// * `pair` - A `Pair` representing the `column_section` in the grammar.
///
/// # Returns
///
/// A `Result` containing a vector of `Column` structs or an error if parsing fails.
fn parse_columns(pair: Pair<Rule>) -> Result<Vec<Column>, SchemaErr> {
    let mut columns = Vec::new();

    for column_entry in pair.into_inner() {
        if column_entry.as_rule() == Rule::column_entry {
            let mut column = Column {
                name: String::new(),
                datatype: String::new(),
                mode: String::new(),
            };

            for sub_entry in column_entry.into_inner() {
                match sub_entry.as_rule() {
                    Rule::identifier => {
                        column.name = sub_entry.as_str().to_string();
                    }
                    Rule::column_type => {
                        if let Some(datatype_pair) = sub_entry.into_inner().next() {
                            column.datatype = datatype_pair.as_str().to_string();
                        } else {
                            return Err(SchemaErr::MissingValue);
                        }
                    }
                    Rule::column_mode => {
                        if let Some(mode_pair) = sub_entry.into_inner().next() {
                            column.mode = mode_pair.as_str().to_string();
                        } else {
                            return Err(SchemaErr::MissingValue);
                        }
                    }
                    _ => {}
                }
            }
            columns.push(column);
        }
    }
    Ok(columns)
}
