use lazy_static::lazy_static;
use crate::TemplateMatch;

/// Represents a Pufferfish file
#[derive(Debug, Clone)]
pub struct PufferfishFile<'a> {
    /// The original text of te file
    pub text: &'a str,
    /// The matched templates in this file
    pub templates: Vec<TemplateMatch<'a>>
}

impl<'a> PufferfishFile<'a> {
    /// Parse a Pufferfish file to a `PufferfishFile` struct containing all the matched templates.
    pub fn parse(text: &'a str) -> Self {
        // Escape char regex requires the "fancy-regex" crate. This might impact performance (but shouldn't be a big deal)
        // Credit for regex for escape characters: https://stackoverflow.com/a/11819111/14874405
        // `(?<!\\)(?:\\\\)*[character]`
        // (?<!\\)      Matches if the preceding character is not a backslash
        // (?:\\\\)*    Matches any number of occurrences of two backslashes
        // [character]  Matches a character (that has to be escaped) (replace)
        lazy_static! {
            static ref TEMPLATE_REGEX: fancy_regex::Regex = fancy_regex::Regex::new(r"(?s)(?<!\\)(?:\\\\)*%(.*?)(?<!\\)(?:\\\\)*%").unwrap();
        }
        
        let templates: Vec<TemplateMatch> = TEMPLATE_REGEX.find_iter(&text).map(|m| {
            let m = m.unwrap();
            // No escape characters
            //if &text.get(m.start()-1..m.start()).unwrap_or("l") != &"\\" && &text[m.end()-1..m.end()] != "\\" {
                Some(TemplateMatch::parse(&text[m.start()..m.end()]))
            // } else {
            //     None
            // }
        }).filter(|v| { println!("{:?}", v); v.is_some() }).map(|v| { println!("{:?}", v); v.unwrap() }).collect();
        println!("templates: {:?}", templates);
        
        Self {
            text,
            templates
        }
    }
}