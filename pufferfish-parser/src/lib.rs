#![deny(missing_docs)]

//! # The Pufferfish Parser
//!
//! Parses pufferfish files

mod pufferfish_file;
mod template_match;

pub use pufferfish_file::*;
pub use template_match::*;
