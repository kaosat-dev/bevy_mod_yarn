use std::collections::HashMap;

use crate::prelude::{Branch, Statements, Dialogue, YarnNode, Commands};

// FIXME: temporary mock of Yarnasset
#[derive(Debug, Default)] 
pub struct YarnAsset {
    pub raw: String,
    pub nodes: HashMap<String, YarnNode>
}

#[derive(Debug)] // FIXME: the only needed dependency from bevy is the Component trait
pub struct DialogueTracker<'a>{
    /// what yarn script are we using for this dialogue tracker
    pub yarn_asset:  Option<&'a YarnAsset>,
    pub current_node_name: String,
    pub current_statement_index: usize,
    pub current_choice_index: usize,

    pub current_branch: Branch,

    branches_stack: Vec<Branch>,
    indices_stack: Vec<(usize, usize)> // we want to resume where we where
}

impl Default for DialogueTracker<'_> {
    fn default() -> Self { 
      
        DialogueTracker { 
            yarn_asset: None, 
            current_node_name: "".into(), 
            current_statement_index: 0, 
            current_choice_index: 0, 
            current_branch: Branch { statements: vec![] },
             
            branches_stack: vec![],
            indices_stack: vec![]
        }
     }

}

impl DialogueTracker<'_>  {
    pub fn new(yarn_asset: &YarnAsset, start_node_name: String) -> DialogueTracker {
        DialogueTracker {
            yarn_asset: Some(yarn_asset),
            current_node_name: start_node_name.clone(),
            current_statement_index: 0,
            current_choice_index: 0 ,
            current_branch: yarn_asset.nodes[&start_node_name.clone()].branch.clone(),

            branches_stack: vec![],
            indices_stack: vec![]
        }
    }

    // FIXME: add this back once we have added bevy as a dependency and added yarn_loader
    pub fn set_current_branch(&mut self, yarn_asset: &YarnAsset) {
        //let default = &yarn_asset.nodes[&self.current_node_name];
        self.current_branch = yarn_asset.nodes[&self.current_node_name].branch.clone(); // FIXME: self.current_node_name might not be set correcly, add safeguard
    }

    /// go to next entry if available, currently also validates the selected choice
    /// TODO: perhaps this should be called next_statement() ? or even just next() ?
    pub fn next_entry(&mut self) -> Statements {
        if self.yarn_asset.is_none() {
            // FIXME: not graceful at all !!
            panic!("no yarn asset for this dialogue runner")
        }
        let yarn_asset = self.yarn_asset.unwrap();
        println!("next entry");
        //FIXME yuck
        // this is to deal with choices
        let old_entry = self.current_branch.statements[self.current_statement_index].clone();
        match  old_entry {
            Statements::Choice(ref choice) => {
                println!("choice");
                self.branches_stack.push(self.current_branch.clone());
                self.indices_stack.push((self.current_choice_index, self.current_statement_index));

                self.current_branch = choice.branches[self.current_choice_index].clone();
                self.current_choice_index = 0;
                self.current_statement_index = 0;
                return self.current_branch.statements[self.current_statement_index].clone();
            },
            Statements::Exit => {
                println!("dialogues done")
            },
            _=> {}
        }

        if self.current_statement_index + 1 < self.current_branch.statements.len() {
            self.current_statement_index +=1;
        }
        else { 
            println!("last in current branch reached");
            if self.branches_stack.len() > 0 {
                self.current_branch = self.branches_stack.pop().unwrap();
                self.current_choice_index = 0;
                self.current_statement_index = 0;
                // reset to previous indices ?
            }
        }
        let current_entry = self.current_branch.statements[self.current_statement_index].clone();
        println!("current entry {:?}",current_entry);
        match  current_entry {
            Statements::Command(command) => {
                println!("EXECUTE COMMAND {:?}", command);
                match command.command_type {
                    Commands::Jump => {
                        if yarn_asset.nodes.contains_key(&command.params){
                            // we jump to the other named node and return the first item from there
                            // we also reset everything
                            self.current_statement_index = 0;
                            self.current_choice_index = 0;
                            self.current_node_name = command.params.clone();
                            self.current_branch = yarn_asset.nodes[&self.current_node_name].branch.clone();
                        
                            return self.current_branch.statements[self.current_statement_index].clone();
                        }else {
                            return self.next_entry();
                        }
                    }
                    _=> {
                        return self.next_entry();
                    }
                }        
            },
            Statements::Choice(ref choices) => {
                println!("choice");
                // self.current_branch = choices[self.current_choice_index].clone();
                // self.current_choice_index = 0;
                // here we select the current choice: FIXME: should it be explictely another , seperate command ? like "validate choice ??"
                return  current_entry;
            },
            _=> {
                println!("line");
                return  current_entry;
            }
        }
    }

    pub fn next_choice(&mut self){
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

    pub fn prev_choice(&mut self){
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

    pub fn current_statement(&self) -> Statements {
        let current_statement = self.current_branch.statements[self.current_statement_index].clone();
        current_statement
    }
    // TODO: these two functions are only needed because we do no keep a Dialogue in the branch data structure ... (a valid Choice HAS to have one, the root branch does not have one, obviously)
    pub fn get_current_choice_branch_first(&self) -> Result<Dialogue, String> {
        let current_statement_index = self.current_statement();
        match current_statement_index {
            Statements::Choice(ref choice) => {
                let current_choice_index = &choice.branches[self.current_choice_index];
                let first = &current_choice_index.statements[0];
                match first {
                    Statements::Dialogue(dialogue) => {
                        Ok(dialogue.clone())
                    },
                    _ => {
                        Err("the first entry in the choice is not a Line".to_string())
                    }
                }
                
            },
            _ => {
                Err("the current item is not a choice".to_string())
            } 
        }
    }

    /// helper function for choices: gives you a list of dialogues (ie, who, what), for example when
    /// you want to display the list current choices to the player 
    pub fn get_current_choices (&self) -> Vec<Dialogue> {
        let current_statement_index = self.current_statement();
        match current_statement_index {
            Statements::Choice(ref choice) => {
                return choice.branches
                    .iter()
                    .map(|branch| {
                        let first = &branch.statements[0];
                        match first {
                            Statements::Dialogue(dialogue) => {
                                return dialogue.clone()
                            },
                            _=> {
                                return Dialogue{..Default::default()}
                            }
                        }
                    }).collect();
            },
            _=> {
                return vec![];
            } 
        }
    }
}
