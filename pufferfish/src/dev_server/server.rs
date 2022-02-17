use std::fs;
use std::path::PathBuf;
use regex::Regex;
use simple_server::{Method, Server, StatusCode};
use crate::pufferfish_core::config::PufferfishConfig;

// TODO: live reload for server when a file is changed (maybe append a websocket listener to
//       html that will reload when it receives something.
//       concrete: have a "standard lib" script for pufferfish and add a script tag to html loaded
//       by the server

#[cfg(feature = "server")]
/// Development server for Pufferfish. Serving files in the `dev_dir`.
pub struct PufferfishDevServer<'a> {
    config: &'a PufferfishConfig,
    server: Option<Server>
}

impl<'a> PufferfishDevServer<'a> {
    pub fn new(config: &'a PufferfishConfig) -> Self {
        Self { config, server: None }
    }
    pub fn start(&mut self) {
        let host = "127.0.0.1";
        let port = &self.config.server_port();
        let conf = &self.config;
        let verbose = conf.verbose();
        let dev_dir = conf.dev_dir();
        
        self.server = Some(Server::new(move |req, mut res| {
            match req.method() {
                &Method::GET => {
                    let path = req.uri().path();
                    let mut puf_dir = std::env::current_dir()?;
                    puf_dir = puf_dir.join(&dev_dir);
                    let mut path_buf = PathBuf::from(&path);
                    let reg = Regex::new(r"\A[/\\]").unwrap();
                    path_buf = puf_dir.join(&reg.replace(&path_buf.to_str().unwrap(), "").to_string());
                    // If dir, read index.html
                    if let Ok(metadata) = fs::metadata(&path_buf) {
                        if metadata.is_dir() {
                            path_buf = path_buf.join("index.html");
                        }
                    }
                    if verbose {
                        eprintln!("Serving: {:?}", &path_buf);
                    }
                    let file_contents = fs::read_to_string(&path_buf).unwrap_or("404 Not found!".to_string());
                    let html_reg = Regex::new(r".*\.html").unwrap();
                    if html_reg.is_match(&path_buf.to_str().unwrap()) {
                        res.header("Content-Type", "text/html".as_bytes());
                    } else {
                        // ... Other content types (text, css, images, ...)
                        // https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types
                        res.header("Content-Type", "text/plain".as_bytes());
                    }
                    Ok(res.body(file_contents.into_bytes())?)
                }
                _ => {
                    res.status(StatusCode::NOT_FOUND);
                    Ok(res.body(b"404 Not found!".to_vec())?)
                }
            }
        }));
        eprintln!("Pufferfish development server starting at http://localhost:{} 🐡", &port);
        unsafe {
            // Set serve dir to dev_dir
            // TODO (css, js, images, ...) -> create assets folder, maybe have a template for assets in html
            self.server.as_mut().unwrap_unchecked().set_static_directory(conf.output_dir());
            // Start server
            self.server.as_ref().unwrap_unchecked().listen(host, port)
        };
    }
}