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