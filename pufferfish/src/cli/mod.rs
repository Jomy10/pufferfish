//! Contains everything for the CLI

mod cli_exec;
pub mod build_html;
pub mod file_listener;
pub mod dev_server;
mod clap_app;

pub use clap_app::get_matches;
pub use cli_exec::*;