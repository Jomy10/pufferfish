use pufferfish_parser::PufferfishFile;

#[test]
fn escape() {
    let test = r#"\%nottemplate%"#;
    let templates = PufferfishFile::parse(test).templates;
    println!("{:?}", templates);
    assert_eq!(0, templates.iter().count());
}
#[test]
fn escape_last() {
    let test = r#"%nottemplate\%"#;
    assert_eq!(0, PufferfishFile::parse(test).templates.iter().count());
}
#[test]
fn escape_both() {
    let test = r#"\%nottemplate\%"#;
    assert_eq!(0, PufferfishFile::parse(test).templates.iter().count());
}
#[test]
fn escape_multiple() {
    let test = r#"\%nottemplate%thisis%"#;
    assert_eq!(1, PufferfishFile::parse(test).templates.iter().count());
}
#[test]
fn escape_multiple_middle() {
    let test = r#"%nottemplate\%thisis%"#;
    let temps = PufferfishFile::parse(test).templates;
    assert_eq!(1, temps.iter().count());
    assert_eq!(r#"nottemplate\%thisis"#, temps[0].template_name);
}
#[test]
fn escape_both_multiple() {
    let test = r#"\%nottemplate\%thisis%"#;
    assert_eq!(0, PufferfishFile::parse(test).templates.iter().count());
}
#[test]
fn escape_file_name() {
    let test = r#"%test\%name?test="e"%"#;
    assert_eq!("test%name.html", PufferfishFile::parse(test).templates[0].get_file_name());
}
#[test]
fn get_attr_parsed() {
    let test = r#"%test\%name?test="e\%bb"%"#;
    assert_eq!("e%bb".to_string(), PufferfishFile::parse(test).templates[0].get_attr_parsed("test").unwrap());
}
// TODO
#[test]
fn get_attr_parsed_double() {
    let test = r#"%test\%name?test="e\\%bb"%"#;
    assert_eq!("e%bb".to_string(), PufferfishFile::parse(test).templates[0].get_attr_parsed("test").unwrap());
}
#[test]
fn get_template_name() {
    let test = r#"%test\%name?test="e\%bb"%"#;
    assert_eq!("test%name".to_string(), PufferfishFile::parse(test).templates[0].get_template_name());
}
// TODO: find out why this test doesn't pass
#[test]
fn double_escape_name() {
    let test = r#"%test\\%name?test="e\%bb"%"#;
    assert_eq!(r"test\%name".to_string(), PufferfishFile::parse(test).templates[0].get_template_name());
}
// TODO
#[test]
fn escape_quote() {
    let test = r#"%test?attr="val\"quote"%"#;
    assert_eq!(r#"val"quote"#.to_string(), PufferfishFile::parse(test).templates[0].get_attr_parsed("attr").unwrap())
}
