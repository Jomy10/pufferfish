//! Contains everything for the CLI

pub mod cli;
pub mod build_html;
pub mod file_listener;
mod clap_app;

pub use clap_app::get_matches;