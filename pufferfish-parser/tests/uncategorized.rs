use pufferfish_parser::PufferfishFile;

#[test]
fn file_name_no_extension() {
    let test = "%header%";
    assert_eq!(
        "header.html",
        PufferfishFile::parse(test).templates[0].get_file_name()
    );
}
#[test]
fn file_name_extension() {
    let test = "%header.php%";
    assert_eq!(
        "header.php",
        PufferfishFile::parse(test).templates[0].get_file_name()
    );
}
// TODO: too much gets trimmed from the end when using <> and multiple new lines (only last new line
//       and following spaces should be trimmed
#[ignore]
#[test]
fn funky_syntax() {
    let test = r#"
    %template?
        attr1   =    "rrr"
        
        ,
        
        
 attr2 = <
    p
    
            a
 >
    %"#;
    let puf_file = PufferfishFile::parse(test);
    let attrs = &puf_file.templates[0].attrs;
    println!("Attribute names: {:?}", attrs);
    assert_eq!(
        (&"rrr", &"    p
    
            a", &"template"),
        (attrs.get("attr1").unwrap(), attrs.get("attr2").unwrap(), &puf_file.templates[0].template_name)
    )
}
#[test]
fn template_count() {
    let test = "%temp1%%temp2%%temp3%";
    assert_eq!(3, PufferfishFile::parse(test).templates.iter().count());
}

#[test]
fn edge_case() {
    let test = r#"%this is%this is not%"#;
    let templates = PufferfishFile::parse(test).templates;
    assert_eq!("this is", templates[0].template_name);
    assert_eq!(1, templates.iter().count());
}
// text //
#[test]
fn text_puf_file() {
    let test = "some text %some template%\nsome more text";
    let puf_text = PufferfishFile::parse(test).text;
    assert_eq!(test, puf_text);
}
#[test]
fn text_template() {
    let test = r#"some text %sometemplate?attr1="attribute value",attr2=<
    some mulitline
    attribute
    value
    >%\nsome more text"#;
    let puf_text = PufferfishFile::parse(test).templates[0].text;
    assert_eq!(r#"%sometemplate?attr1="attribute value",attr2=<
    some mulitline
    attribute
    value
    >%"#, puf_text);
}