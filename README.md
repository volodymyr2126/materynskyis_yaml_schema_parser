# My YAML Parser

A Rust program for parsing `.yaml` files that define a database schema. This parser identifies tables, and columns to allow structured interpretation of database schema files.

### Technical Description

The parser uses the Pest library to define grammar rules that interpret specific `.yaml` structures. It parses:
- Schema name
- Tables within the schema
- Columns their types and modes within each table

The parsed data can be used to generate structured representation of database schemas for your application.

### Example of Schema
```text
Schema should be structured like this  
File
 └── Schema Section
      ├── Schema Entry
      │    ├── Name: "name"
      │    └── Tables
      │         └── Table Section
      │              ├── Table Entry
      │              │    ├── Name: table_name
      │              │    └── Columns
      │              │         └── Column Section
      │              │              ├── Column Entry
      │              │              │    ├── Column Name
      │              │              │    ├── Type
      │              │              │    └── Mode
```  
And generally look like this  
```yaml
schema:
  name: myschema
  tables:
    user:
      columns:
        id:
          type: int
          mode: required
        name:
          type: str
          mode: required
        avg_grade:
          type: float
          mode: nullable
```
This schema could be processed like this. 
```rust
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

```
And in result we will have this struct:
```rust
Schema {
    name: "myschema",
    tables: [
        Table {
            name: "user",
            columns: [
                Column {
                    name: "id",
                    datatype: "int",
                    mode: "required",
                },
                Column {
                    name: "name",
                    datatype: "str",
                    mode: "required",
                },
                Column {
                    name: "avg_grade",
                    datatype: "float",
                    mode: "nullable",
                },
            ],
        },
    ],
}
```

