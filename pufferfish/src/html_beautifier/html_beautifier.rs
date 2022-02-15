// Call ruby

use std::io::{Read};
use std::process::{Command, Stdio};

static HTML_BEAUTIFIER: &'static [u8] = include_bytes!("beautifier.rb");

// TODO: option for tab size (implemented in ruby, but not in the execution of part)
/// Beautify html
pub fn beautify(html: &str) -> Result<String, String> {
    // echo {source_code} | ruby -- - html
    let source_code = match Command::new("echo")
        .arg(std::str::from_utf8(HTML_BEAUTIFIER).unwrap())
        .stdout(Stdio::piped())
        .spawn() {
        Ok(process) => process,
        Err(reason) => panic!("Couldn't spawn process cat: {reason}")
    };
    let process = match Command::new("ruby")
        .stdin(source_code.stdout.unwrap())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .arg(r#"--"#)
        .arg(r#"-"#)
        .arg(&format!("{}", html))
        .spawn()
    {
        Err(why) => panic!("Couldn't spawn process: {}", why),
        Ok(process) => process
    };
    
    // Write stdout to string
    let mut sout = String::new();
    match process.stdout.unwrap().read_to_string(&mut sout) {
        Err(why) => panic!("Couldn't read process' stdout: {}", why),
        Ok(_) => {
            if sout.is_empty() {
                // Write stderr to string
                let mut serr = String::new();
                match process.stderr.unwrap().read_to_string(&mut serr) {
                    Err(why) => panic!("Couldn't read process' stderr: {}", why),
                    Ok(_) => {
                        return Err(serr);
                    }
                }
            } else {
                return Ok(sout);
            }
        }
    }
}