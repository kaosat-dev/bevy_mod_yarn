use std::collections::HashMap;

use crate::prelude::{Branch, Statements, Dialogue, YarnNode};

#[derive(Debug, Default)] // FIXME: the only needed dependency from bevy is the Component trait
pub struct DialogueTracker {
    pub current_node: String,
    pub current: usize,
    pub current_choice: usize,
    pub current_branch: Branch,
}

// FIXME: temporary mock of Yarnasset
pub struct YarnAsset {
    pub raw: String,
    pub nodes: HashMap<String, YarnNode>
}

impl DialogueTracker  {

    // FIXME: add this back once we have added bevy as a dependency and added yarn_loader
    pub fn set_current_branch(&mut self, yarn_asset: &YarnAsset) {
        //let default = &yarn_asset.nodes[&self.current_node];
        self.current_branch = yarn_asset.nodes[&self.current_node].branch.clone(); // FIXME: self.current_node might not be set correcly, add safeguard
    }

    /// go to next entry if available, currently also validates the selected choice
    /// TODO: perhaps this should be called next_statement() ? or even just next() ?
    pub fn next_entry(&mut self, yarn_asset: &YarnAsset) -> Statements {
        //FIXME yuck
        let old_entry = self.current_branch.statements[self.current].clone();
        match  old_entry {
            Statements::Choice(ref choice) => {
                println!("choice");
                self.current_branch = choice.branches[self.current_choice].clone();
                self.current_choice = 0;
                self.current = 0;
                return self.current_branch.statements[self.current].clone();
            },
            Statements::Exit => {
                println!("dialogues done")
            },
            _=> {}
        }

        if self.current + 1 < self.current_branch.statements.len() {
            self.current +=1;
        }
        let current_entry = self.current_branch.statements[self.current].clone();
        println!("current entry{:?}",current_entry);
        match  current_entry {
            Statements::Command(command) => {
                println!("EXECUTE COMMAND {:?}", command);
                if command.name == "jump" { // FIXME: meh, string matching, although given the dynamic nature of this, do not see how I could use an enum
                    if yarn_asset.nodes.contains_key(&command.params){
                        // we jump to the other named node and return the first item from there
                        // we also reset everything
                        self.current = 0;
                        self.current_choice = 0;
                        self.current_node = command.params.clone();
                        self.current_branch = yarn_asset.nodes[&self.current_node].branch.clone();
                    
                        return self.current_branch.statements[self.current].clone();
                    }else {
                        return self.next_entry(yarn_asset);
                    }
                }else {
                    // we just jump to the next entry
                    return self.next_entry(yarn_asset);
                }
            },
            Statements::Choice(ref choices) => {
                println!("choice");
                // self.current_branch = choices[self.current_choice].clone();
                // self.current_choice = 0;
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
                self.current_choice += 1;
                if self.current_choice >= choice.branches.len() {
                    self.current_choice = 0;
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
                if self.current_choice == 0 {
                    self.current_choice = choice.branches.len() - 1;
                } else {
                    self.current_choice -= 1;
                }
            }
            _ => {
                println!("not a choice !");
            }
        }
    }

    pub fn current_statement(&self) -> Statements {
        let current_statement = self.current_branch.statements[self.current].clone();
        current_statement
    }
    // TODO: these two functions are only needed because we do no keep a Dialogue in the branch data structure ... (a valid Choice HAS to have one, the root branch does not have one, obviously)
    pub fn get_current_choice_branch_first(&self) -> Result<Dialogue, String> {
        let current = self.current_statement();
        match current {
            Statements::Choice(ref choice) => {
                let current_choice = &choice.branches[self.current_choice];
                let first = &current_choice.statements[0];
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
        let current = self.current_statement();
        match current {
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
