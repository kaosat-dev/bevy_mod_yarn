
pub mod parser;

/// Most commonly used types
pub mod prelude {

    #[doc(hidden)]
    pub use crate::{
        parser::{
            parse_yarn_nodes_nom,
            structs::*,
            statement_dialogue,
            statement_choice,
            statement_command
        }

    };
}