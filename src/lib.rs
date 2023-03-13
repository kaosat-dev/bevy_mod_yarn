
pub mod parser;
pub mod evaluator;
// pub mod yarn_loader;
/// Most commonly used types
pub mod prelude {

    #[doc(hidden)]
    pub use crate::{
        parser::{
            structs::*,
            parse_yarn_nodes_nom,
            statement_dialogue,
            statement_choice,
            statement_command
        }

    };
}