use std::collections::HashMap;

pub mod common;
pub use common::*;

pub mod header;
pub use header::*;

pub mod body;
pub use body::*;

pub mod nodes;
pub use nodes::*;

pub mod structs;
pub use structs::*;

/// main entry point
pub fn parse_yarn_nodes(yarn_text: &str) -> HashMap<String, YarnNode> {
    if let Ok(result) = yarn_nodes(yarn_text) {
        return result.1;
    }
    return HashMap::new();
}
