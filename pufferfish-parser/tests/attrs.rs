use std::collections::HashMap;
use pufferfish_parser::PufferfishFile;

#[test]
fn one_attr() {
    let test = r#"%temp?attr="my attr"%"#;
    assert_eq!(
        (&"attr", &"my attr"),
        PufferfishFile::parse(test).templates.first().unwrap().attrs.iter().next().unwrap()
    )
}
#[test]
/// Test if only one attribute is present
fn only_one_attr() {
    let test = r#"%template?attr="test"%"#;
    assert_eq!(
        1,
        PufferfishFile::parse(test).templates.first().unwrap().attrs.iter().count()
    )
}
#[test]
fn two_attrs() {
    let test = r#"%temp?attr1="my attr",attr2="my other attr"%"#;
    assert_eq!(
        HashMap::from([("attr1", "my attr"), ("attr2", "my other attr")]),
        PufferfishFile::parse(test).templates.first().unwrap().attrs
    )
}
#[test]
fn only_two_attrs() {
    let test = r#"%temp?attr1="my attr",attr2="my other attr"%"#;
    assert_eq!(
        2,
        PufferfishFile::parse(test).templates.first().unwrap().attrs.iter().count()
    )
}
#[test]
fn multiline_attr_bracket() {
    let test = r#"
    %teemp?attr1=<
    <p>my template text</p>
    >%
    "#;
    assert_eq!(
        &"    <p>my template text</p>",
        PufferfishFile::parse(test).templates[0].attrs.get("attr1").unwrap()
    )
}
#[test]
fn multiline_attr_quote() {
    let test = r#"
    %teemp?attr1="
    <p>my template text</p>
    "%
    "#;
    assert_eq!(
        &"\n    <p>my template text</p>\n    ",
        PufferfishFile::parse(test).templates[0].attrs.get("attr1").unwrap()
    )
}
#[test]
fn multiline_single_line_attr() {
    let test = r#"
    %teemp?attr1=<
    <p>my template text</p>
    >,
    attr2 =
    "hello world"%
    "#;
    let attrs = &PufferfishFile::parse(test).templates[0].attrs;
    assert_eq!(
        (&"    <p>my template text</p>", &"hello world"),
        (attrs.get("attr1").unwrap(), attrs.get("attr2").unwrap())
    )
}