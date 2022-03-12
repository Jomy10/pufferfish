use std::collections::HashMap;
use concat_strs::concat_strs;
use lazy_static::lazy_static;
use regex::Regex;

/// A template match inside a Pufferfish file
/// Holds the template `text`, the `template name` and its `attr`ibute`s`.
#[derive(Debug, Clone)]
pub struct TemplateMatch<'a> {
    /// The original text. e.g. `%template_name?attr="value"%`.
    /// Used for replacing later.
    pub text: &'a str,
    /// The name of the template file
    pub template_name: &'a str,
    /// The template attributes.
    /// (attribute_name, attribute_value)
    pub attrs: HashMap<&'a str, &'a str>
}

impl<'a> TemplateMatch<'a> {
    
    // TODO: better error handling everywhere where panic or except is used
    /// Create a new `TemplateMatch` struct from a string formatted in a Pufferfish template input way
    /// e.g.
    /// ```txt
    /// template_name?attr="my attr",attr2=<
    /// <p>my template attribute</p>
    /// >
    /// ```
    /// Holds data about the the template.
    pub fn parse(text: &'a str) -> Self {
        lazy_static! {
            static ref END_MULTILINE_REGEX: Regex = Regex::new(r"(.*)(\n *)").unwrap();
        }
        
        let mut name: &str = "";
        let mut attrs: HashMap<&str, &str> = HashMap::new();
        let _ = &text[1..text.len() - 1].split("?").enumerate().for_each(|(idx, v)|
            if idx == 0 {
                name = v;
            } else if idx == 1 {
                v.split(",").for_each(|v| {
                    let mut name = "";
                    let mut val = "";
                    v.split("=").enumerate().for_each(|(idx, v)| {
                        if idx == 0 {
                            // Attribute name
                            name = v.trim();
                        } else if idx == 1 {
                            // Attribute value
                            // Difference " " and < >:
                            //  - <> will remove a new line in the beginning and end, meaning that
                            // it contents can start beneath it
                            if let Some(first_quote) = v.find("\"") {
                                let last_quote = v.rfind("\"").expect("Unmatched quote");
                                val = &v[first_quote+1..last_quote];
                            } else if let Some(first_par) = v.find("<") {
                                let last_par = v.rfind(">").expect("Unmatched \"<\"");
                                let mut new = &v[first_par+1..last_par];
                                if new.starts_with("\n") {
                                    new = &new[1..new.len()];
                                }
                                // Remove trailing (\n *)
                                if let Some(m) = END_MULTILINE_REGEX.captures(new).unwrap().get(1) {
                                    new = &new[m.start()..m.end()]
                                }
                                val = new;
                            } else {
                                panic!("ERROR: Attribute values require `\"` or `<`")
                            }
                        } else {
                            // TODO: better error handling
                            panic!("ERROR: Too many parameters. Did you forget a \"=\"?");
                        }
                    });
                    attrs.insert(name, val);
                });
            } else {
                // TODO: better error message (line number)
                panic!("ERROR: Too many question marks in line")
            }
        );
        
        Self {
            text,
            template_name: name,
            attrs
        }
    }
    
    /// Returns the value of an attribute
    pub fn get_attr(&self, attr: &'a str) -> Option<&&'a str> {
        self.attrs.get(&attr)
    }
    
    // TODO: all parsed values -> lazy static; only computed once
    /// Returns the value of an attribute with all `\%` replaced with `%`
    pub fn get_attr_parsed(&self, attr: &'a str) -> Option<String> {
        if let Some(val) = self.attrs.get(attr) {
            Some(val.replace("\\%", "%").replace(r"\\", ""))
        } else {
            None
        }
    }
    
    /// Returns the value of an attribute with all `\%` replaced with `%`
    pub unsafe fn get_attr_parsed_unchecked(&self, attr: &'a str) -> String {
        self.attrs.get(attr).unwrap_unchecked().replace(r"\%", "%").replace(r"\\", "")
    }
    
    /// Returns the file name of the template
    pub fn get_file_name(&self) -> String {
        lazy_static! {
            static ref HTML_FILE_REGEX: Regex = Regex::new(r".*?\..*").unwrap();
        }
        
        return if HTML_FILE_REGEX.is_match(self.template_name) {
            self.template_name.to_string()
        } else {
            let file_name: &str = &self.template_name.replace(r"\%", "%").replace(r"\\", "");
            concat_strs! {file_name, ".html"}
        };
    }
    
    /// Returns the template name, with all `\%` replaced with `\%`
    pub fn get_template_name(&self) -> String {
        println!("{:?}", self.template_name);
        self.template_name.replace(r"\%","%").replace(r"\\", "")
    }
}
