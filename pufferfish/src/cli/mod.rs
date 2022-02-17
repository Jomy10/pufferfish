//! Contains everything for the CLI

mod cli_exec;
pub mod file_listener;
pub mod dev_server;
mod clap_app;
pub mod build;

pub use clap_app::get_matches;
pub use cli_exec::*;