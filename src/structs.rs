use std::{fs, collections::HashMap, fmt::format};

#[derive(Debug, Clone, Default)]
pub struct YarnNode {
    pub title: String,
    pub tags: Vec<String>,
    pub branch: Branch
}

#[derive(Debug, Clone, Default)]
pub struct Branch {
   pub statements: Vec<Statements>,
}


#[derive(Debug, Clone, Default)]
pub struct Dialogue {
    pub who: String,
    pub what: String,
    pub attributes: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Choice{
    pub branches: Vec<Branch>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct YarnCommand {
    pub name: String,
    pub params: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Statements {
    Dialogue(Dialogue),
    Choice(Choice),
    Command(YarnCommand),

    // Fixme not sure, these are convenience enums make parsing easier but might not be the most practical
    ChoiceBranch(Branch),
    Empty,
    Exit,
}

// TODO: perhaps add a trait for all statements, and attach tags there ? or add tags to all base enum entries, and change Vec<Branch> to an iterable "Branches"