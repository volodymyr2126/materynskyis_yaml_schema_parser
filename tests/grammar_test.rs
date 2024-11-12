use anyhow::{anyhow, Result};
use materynskyis_yaml_schema_parser::*;
use pest::Parser;

#[test]
fn identifier_test() -> anyhow::Result<()> {
    let pair = YAMLParser::parse(Rule::identifier, "valid_identifier")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "valid_identifier");
    Ok(())
}

#[test]
fn column_mode_value_test() -> anyhow::Result<()> {
    let pair = YAMLParser::parse(Rule::column_mode_value, "nullable")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "nullable");

    let pair = YAMLParser::parse(Rule::column_mode_value, "required")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "required");
    Ok(())
}

#[test]
fn column_type_value_test() -> anyhow::Result<()> {
    let pair = YAMLParser::parse(Rule::column_type_value, "int")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "int");

    let pair = YAMLParser::parse(Rule::column_type_value, "str")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "str");
    Ok(())
}
#[test]
fn column_mode_test() -> anyhow::Result<()> {
    let input = "mode: required";
    let pair = YAMLParser::parse(Rule::column_mode, input)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), input);
    Ok(())
}

#[test]
fn column_type_test() -> anyhow::Result<()> {
    let input = "type: int";
    let pair = YAMLParser::parse(Rule::column_type, input)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), input);
    Ok(())
}

#[test]
fn column_entry_test() -> anyhow::Result<()> {
    let input = "column1:\n  type: int\n  mode: required\n";
    let pair = YAMLParser::parse(Rule::column_entry, input)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), input);
    Ok(())
}

#[test]
fn column_section_test() -> anyhow::Result<()> {
    let input = "column1:\n  type: int\n  mode: required\n";
    let pair = YAMLParser::parse(Rule::column_section, input)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), input);
    Ok(())
}

#[test]
fn table_entry_test() -> anyhow::Result<()> {
    let input = "table1:\n  columns:\nid:\ntype: int\nmode: nullable";
    let pair = YAMLParser::parse(Rule::table_entry, input)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), input);
    Ok(())
}

#[test]
fn table_section_test() -> anyhow::Result<()> {
    let input = "table1:\n  columns:\nid:\ntype: int\nmode: nullable\ntable2:\n  columns:\nuser_id:\ntype: str\nmode: required";
    let pair = YAMLParser::parse(Rule::table_section, input)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), input);
    Ok(())
}

#[test]
fn schema_entry_test() -> anyhow::Result<()> {
    let input = "name: my_schema\ntables:\ntable1:\n  columns:\nid:\ntype: int\nmode: nullable";
    let pair = YAMLParser::parse(Rule::schema_entry, input)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), input);
    Ok(())
}

#[test]
fn schema_section_test() -> anyhow::Result<()> {
    let input = "schema:\n  name: my_schema\n  tables:\ntable1:\n  columns:\nid:\ntype: int\nmode: nullable\n";
    let pair = YAMLParser::parse(Rule::schema_section, input)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), input);
    Ok(())
}
