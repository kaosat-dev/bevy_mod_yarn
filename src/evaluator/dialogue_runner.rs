use bevy::ecs::component::Component;
// use bevy::ecs::system::Resource;
// use bevy::asset::Handle;
use crate::prelude::{Branch, Commands, Dialogue, Statements, YarnAsset};

#[derive(Debug, Component)]
pub struct DialogueRunner {
    /// what yarn script are we using for this dialogue tracker
    pub yarn_asset: Option<YarnAsset>,
    pub current_node_name: String,
    pub current_statement_index: usize,
    pub current_choice_index: usize,

    pub current_branch: Branch,

    branches_stack: Vec<Branch>,
    indices_stack: Vec<(usize, usize)>, // we want to resume where we where
}

impl Default for DialogueRunner {
    fn default() -> Self {
        DialogueRunner {
            yarn_asset: None,
            current_node_name: "".into(),
            current_statement_index: 0,
            current_choice_index: 0,
            current_branch: Branch { statements: vec![] },

            branches_stack: vec![],
            indices_stack: vec![],
        }
    }
}

impl DialogueRunner {
    pub fn new(yarn_asset: YarnAsset, start_node_name: &str) -> DialogueRunner {
        let start_node_name = &start_node_name.clone().to_string();
        // let current_branch: Branch = Branch { statements: vec![] }; // to handle case where there is no matching start node ?
        if !yarn_asset.nodes.contains_key(start_node_name) {
            panic!("yarn file does not contain node {:?}", start_node_name)
        }

        DialogueRunner {
            yarn_asset: Some(yarn_asset.clone()),
            current_node_name: start_node_name.clone(),
            current_statement_index: 0,
            current_choice_index: 0,
            current_branch: yarn_asset.nodes[&start_node_name.clone()].branch.clone(),

            branches_stack: vec![],
            indices_stack: vec![],
        }
    }

    pub fn set_current_branch(&mut self, yarn_asset: &YarnAsset) {
        //let default = &yarn_asset.nodes[&self.current_node_name];
        self.current_branch = yarn_asset.nodes[&self.current_node_name].branch.clone();
        // FIXME: self.current_node_name might not be set correcly, add safeguard
    }

    pub fn current_statement(&self) -> Statements {
        let current_statement =
            self.current_branch.statements[self.current_statement_index].clone();
        current_statement
    }

    /// go to next entry if available, currently also validates the selected choice
    /// TODO: perhaps this should be called next_statement() ? or even just next() ?
    /// TODO: this should either return an Option<Statement> or another error signifier (ie for example if there is no node for jumping etc)
    pub fn next_entry(&mut self) -> Statements {
        if self.yarn_asset.is_none() {
            panic!("no yarn asset for this dialogue runner")
        }
        let yarn_asset = self.yarn_asset.as_mut().unwrap();
        // println!("next entry");

        //FIXME yuck: not an ideal way to deal with choice selection
        // this is to deal with choices
        let current_entry = self.current_branch.statements[self.current_statement_index].clone();
        match current_entry {
            Statements::Choice(ref choice) => {
                println!("choice");
                self.branches_stack.push(self.current_branch.clone());
                self.indices_stack
                    .push((self.current_choice_index, self.current_statement_index));

                self.current_branch = choice.branches[self.current_choice_index].clone();
                self.current_choice_index = 0;
                self.current_statement_index = 0;
                return self.current_branch.statements[self.current_statement_index].clone();
            }
            Statements::Exit => {
                println!("dialogues done");
                return Statements::Exit;
            }
            _ => {}
        }

        if self.current_statement_index + 1 < self.current_branch.statements.len() {
            self.current_statement_index += 1;
        } else {
            // FIXME: not super clean way to pop until empty/ back in a normal flow
            while self.current_statement_index <= self.current_branch.statements.len()
                && self.branches_stack.len() > 0
            {
                self.current_branch = self.branches_stack.pop().unwrap();
                let (_choice_index, statement_index) = self.indices_stack.pop().unwrap();
                self.current_choice_index = 0; // reset choice to first choice // FIXME: should it use the choice index above ?
                self.current_statement_index = statement_index + 1; // FIXME: check if this is a valid statement !!
            }
        }
        let current_entry = self.current_branch.statements[self.current_statement_index].clone();

        match current_entry {
            Statements::Command(command) => {
                match command.command_type {
                    Commands::Declare => {
                        // TODO: remove duplicate code
                        // TODO: implement

                        if self.current_statement_index + 1 < self.current_branch.statements.len() {
                            self.current_statement_index += 1;
                        }
                        return self.current_branch.statements[self.current_statement_index]
                            .clone();
                    }
                    Commands::Set => {
                        // TODO: remove duplicate code
                        // TODO: implement
                        if self.current_statement_index + 1 < self.current_branch.statements.len() {
                            self.current_statement_index += 1;
                        }
                        return self.current_branch.statements[self.current_statement_index]
                            .clone();
                    }
                    Commands::Jump => {
                        if yarn_asset.nodes.contains_key(&command.params) {
                            // we jump to the other named node and return the first item from there
                            // we also reset everything
                            self.current_statement_index = 0;
                            self.current_choice_index = 0;
                            self.current_node_name = command.params.clone();
                            self.current_branch =
                                yarn_asset.nodes[&self.current_node_name].branch.clone();
                            return self.current_branch.statements[self.current_statement_index]
                                .clone();
                        } else {
                            panic!("no node named {} found in the yarn file!", &command.params);
                        }
                    }
                    Commands::Stop => {
                        // TODO: remove duplicate code
                        return Statements::Exit;
                    }
                    _ => {
                        // for any non internal / Generic command, you need to handle going to the next entry yourself by calling runner.next_entry()
                        return self.current_branch.statements[self.current_statement_index]
                            .clone();
                    }
                }
            }
            _ => {
                return current_entry;
            }
        }
    }

    /// go to the next choice, goes to 0 when overflowing
    pub fn next_choice(&mut self) {
        match self.current_statement() {
            Statements::Choice(ref choice) => {
                self.current_choice_index += 1;
                if self.current_choice_index >= choice.branches.len() {
                    self.current_choice_index = 0;
                }
            }
            _ => {
                println!("not a choice !");
            }
        }
    }

    /// go to the previous choice, goes to choices.len() -1 when underflowing
    pub fn prev_choice(&mut self) {
        match self.current_statement() {
            Statements::Choice(ref choice) => {
                if self.current_choice_index == 0 {
                    self.current_choice_index = choice.branches.len() - 1;
                } else {
                    self.current_choice_index -= 1;
                }
            }
            _ => {
                println!("not a choice !");
            }
        }
    }

    ///
    pub fn specific_choice(&mut self, choice_index: usize) {
        match self.current_statement() {
            Statements::Choice(ref choice) => {
                if choice_index != 0 && choice_index < choice.branches.len() {
                    self.current_choice_index = choice_index;
                }
            }
            _ => {
                println!("not a choice !");
            }
        }
    }

    // TODO: these two functions are only needed because we do no keep a Dialogue in the branch data structure ... (a valid Choice HAS to have one, the root branch does not have one, obviously)
    pub fn get_current_choice_branch_first(&self) -> Result<Dialogue, String> {
        let current_statement_index = self.current_statement();
        match current_statement_index {
            Statements::Choice(ref choice) => {
                let current_choice_index = &choice.branches[self.current_choice_index];
                let first = &current_choice_index.statements[0];
                match first {
                    Statements::Dialogue(dialogue) => Ok(dialogue.clone()),
                    _ => Err("the first entry in the choice is not a Line".to_string()),
                }
            }
            _ => Err("the current item is not a choice".to_string()),
        }
    }

    /// helper function for choices: gives you a list of dialogues (ie, who, what), for example when
    /// you want to display the list current choices to the player
    pub fn get_current_choices(&self) -> (Vec<Dialogue>, usize) {
        let current_statement = self.current_statement();
        match current_statement {
            Statements::Choice(ref choice) => {
                return (
                    choice
                        .branches
                        .iter()
                        .map(|branch| {
                            let first = &branch.statements[0];
                            match first {
                                Statements::Dialogue(dialogue) => return dialogue.clone(),
                                _ => {
                                    return Dialogue {
                                        ..Default::default()
                                    }
                                }
                            }
                        })
                        .collect(),
                    self.current_choice_index,
                );
            }
            _ => {
                return (vec![], self.current_choice_index);
            }
        }
    }
}
