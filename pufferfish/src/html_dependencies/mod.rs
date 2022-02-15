//! Generate dependant graph so each file can be updated if one of its dependencies is updated.
//!
//! Used in the development server.

pub mod dependency_graph;
pub use dependency_graph as graph;
mod dependency;
pub mod dependant;
