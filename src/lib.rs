#![doc = include_str!("../docs.md")]

use ::pest_derive::Parser;
use pest::iterators::Pair;
use pest::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct YAMLParser;

#[derive(Debug)]
pub struct Schema {
    name: String,
    tables: Vec<Table>,
}

#[derive(Debug)]
pub struct Table {
    name: String,
    columns: Vec<Column>,
}

#[derive(Debug)]
pub struct Column {
    name: String,
    datatype: String,
    mode: String,
}

#[derive(Debug, Error)]
pub enum SchemaErr {
    #[error("Error while parsing yaml schema")]
    ParseError(String),
    #[error("Missing value while parsing")]
    MissingValue,
}

pub fn parse_schema(input: &str) -> Result<Schema, SchemaErr> {
    let parse_result =
        YAMLParser::parse(Rule::file, input).map_err(|e| SchemaErr::ParseError(e.to_string()))?;

    let mut schema = Schema {
        name: String::new(),
        tables: Vec::new(),
    };

    for pair in parse_result {
        match pair.as_rule() {
            Rule::file => {
                for inner_pair in pair.into_inner() {
                    match inner_pair.as_rule() {
                        Rule::schema_section => {
                            for schema_entry in inner_pair.into_inner() {
                                match schema_entry.as_rule() {
                                    Rule::schema_entry => {
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
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    Ok(schema)
}

fn parse_tables(pair: Pair<Rule>) -> Result<Vec<Table>, SchemaErr> {
    let mut tables = Vec::new();

    for table_entry in pair.into_inner() {
        match table_entry.as_rule() {
            Rule::table_entry => {
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
            _ => {}
        }
    }

    Ok(tables)
}

fn parse_columns(pair: Pair<Rule>) -> Result<Vec<Column>, SchemaErr> {
    let mut columns = Vec::new();

    for column_entry in pair.into_inner() {
        match column_entry.as_rule() {
            Rule::column_entry => {
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
            _ => {}
        }
    }

    Ok(columns)
}
