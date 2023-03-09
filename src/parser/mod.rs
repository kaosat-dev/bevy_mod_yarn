use std::collections::HashMap;

pub mod common;
pub use common::*;

pub mod header;
pub use header::*;

pub mod body;
pub use body::*;

pub mod nodes;
pub use nodes::*;

pub mod nom_supreme_yeah;
pub use nom_supreme_yeah::*;

pub mod working_choice_blocks;
pub use working_choice_blocks::*;

pub mod choices_alt;
pub use choices_alt::*;

use crate::YarnNode;


///main entry point
pub fn parse_yarn_nodes_nom(yarn_text: &str) -> HashMap<String, YarnNode> {
    println!("PARSING WITH NOM");
    if let Ok(result) = yarn_nodes(yarn_text) {
        return result.1;
    }
    return  HashMap::new();
}
