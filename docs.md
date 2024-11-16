***USEFUL LINKS***  
[This crate on crates.io](https://crates.io/crates/yaml_database_schema_parser)
[Documentation on docs.rs](https://docs.rs/yaml_database_schema_parser/0.1.1/yaml_database_schema_parser/)
***PURPOSE***  
The parser uses the Pest library to define grammar rules that interpret specific `.yaml` structures. It parses:
- Schema name
- Tables within the schema
- Columns their types and modes within each table

The parsed data can be used to generate structured representation of database schemas for your application.
***GRAMMAR***
1. WHITESPACE and NEWLINE
    Handle whitespace and line breaks for flexibility in formatting.
2. file
    Represents the entire schema file, containing a schema definition.
3. schema_section
    Defines the overall structure of the schema, including tables.
4. schema_entry
    Defines the name of the schema and the list of tables.
5. table_section
    Contains one or more table definitions (table_entry).
6. table_entry
    Defines a table with a name and its columns.
7. column_section
    Lists one or more column definitions (column_entry).
8. column_entry
    Describes the properties of a single column, including type and mode.
9. column_type
    Specifies the data type for a column.
10. column_mode
    Specifies whether a column is nullable, required, or repeated.
11. column_type_value
    Defines valid data types for columns.
12. column_mode_value
    Defines valid modes for columns.
13. identifier
    Defines valid identifiers for schema, table, and column names.