use std::fs;
use std::path::{Path, PathBuf};
use minify_html::minify;
use crate::pufferfish_core::config::{MinifyMethod, PufferfishConfig};
use crate::{html_beautifier};
use crate::pufferfish_core::parser;

/// Build & write 1 file
/// - `clean`: don't use cached files
pub fn build_html_file(file: &Path, config: &PufferfishConfig, clean: bool, dev: bool) {
    let mut parsed = parser::parse_file(&file, &config.html_dir(), &config.template_dir(), clean, Some(&config.cache_dir()));
    let mut output_dir: Option<String> = None;
    if !dev { output_dir = Some(config.clone().output_dir()) } else { output_dir = Some(config.clone().dev_dir()) };
    if config.minify() {
        if config.method() == MinifyMethod::Default {
            let parsed = parsed.as_bytes();
            build_minify_default(parsed, unsafe { &output_dir.as_ref().unwrap_unchecked() }, file, config);
        } else {
            let mut parsed = unsafe{ parsed.as_bytes_mut() };
            build_minify_onepass(&mut parsed, unsafe { &output_dir.as_ref().unwrap_unchecked() }, file, config);
        }
    } else if config.pretty() {
        let parsed = html_beautifier::beautify(&parsed).expect("Could not beautify html");
        fs::write(PathBuf::from(unsafe { &output_dir.as_ref().unwrap_unchecked() }).join(&file.file_name().unwrap()), parsed)
            .expect(&format!("Couldn't write file {:?}", PathBuf::from(unsafe { &output_dir.as_ref().unwrap_unchecked() }).join(&file.file_name().unwrap())));
    } else {
        fs::write(PathBuf::from(unsafe { &output_dir.as_ref().unwrap_unchecked() }).join(&file.file_name().unwrap()), parsed)
            .expect(&format!("Couldn't write file {:?}", PathBuf::from(unsafe { &output_dir.as_ref().unwrap_unchecked() }).join(&file.file_name().unwrap())));
    }
    if config.verbose() {
        eprintln!("Written file {:?}", file);
    }
}

fn build_minify_default(html: &[u8], output_dir: &str, file: &Path, config: &PufferfishConfig) {
    let mut cfg = minify_html::Cfg::new();
    cfg.do_not_minify_doctype = !config.minify_doctype();
    cfg.ensure_spec_compliant_unquoted_attribute_values = config.ensure_spec_compliant_unquoted_attribute_values();
    cfg.keep_closing_tags = config.keep_closing_tags();
    cfg.keep_html_and_head_opening_tags = config.keep_html_and_head_opening_tags();
    cfg.keep_spaces_between_attributes = config.keep_spaces_between_attributes();
    cfg.keep_comments = config.keep_comments();
    cfg.minify_css = config.minify_css();
    cfg.minify_js = config.minify_js();
    cfg.remove_bangs = config.remove_bangs();
    cfg.remove_processing_instructions = config.remove_processing_instructions();
    let minified = minify(&html, &cfg);
    fs::write(PathBuf::from(output_dir).join(&file.file_name().unwrap()), minified)
        .expect(&format!("Couldn't write file {:?}", PathBuf::from(output_dir).join(&file.file_name().unwrap())));
}

fn build_minify_onepass(html: &mut [u8], output_dir: &str, file: &Path, config: &PufferfishConfig) {
    let cfg = &minify_html_onepass::Cfg {
        minify_js: config.minify_js(),
        minify_css: config.minify_css()
    };
    minify_html_onepass::in_place(html, cfg)
        .expect("Couldn't minify html");
    fs::write(PathBuf::from(output_dir).join(&file.file_name().unwrap()), html)
        .expect(&format!("Couldn't write file {:?}", PathBuf::from(output_dir).join(&file.file_name().unwrap())));
}