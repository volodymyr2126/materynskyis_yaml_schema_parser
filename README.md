# My YAML Parser

A Rust program for parsing `.yaml` files that define a database schema. This parser identifies tables, and columns to allow structured interpretation of database schema files.

### Technical Description

The parser uses the Pest library to define grammar rules that interpret specific `.yaml` structures. It parses:
- Schema name
- Tables within the schema
- Columns their types and modes within each table

The parsed data can be used to generate structured representation of database schemas for your application.

### Example of Usage

```rust

```