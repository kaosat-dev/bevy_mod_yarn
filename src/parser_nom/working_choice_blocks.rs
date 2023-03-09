use nom::{
    character::complete::{alphanumeric1, newline, space0},
    multi::{separated_list1, many_till},
    sequence::{pair, preceded, terminated},
    IResult,
    combinator:: {recognize, eof},
    branch:: {alt},
    bytes::complete::{tag, take_until}
};
use nom::combinator::opt;
use crate::Statements;

use super::{identifier, spacey, statement_dialogue, statement_command};
use nom_supreme::error::ErrorTree;

#[derive(Debug)]
pub struct Branch  {
    pub lines: Vec<String>
}

#[derive(Debug)]
pub struct Choice {
    pub branches: Vec<Branch>
}


fn seperator_empty_lines(input: &str) -> IResult<&str, (char, &str)> {
    pair(newline, empty_line)(input)
}
   
fn seperator_other(input: &str) -> IResult<&str, (char, &str)> {
    pair(newline, tag(":"))(input)
}
   

fn empty_line(input: &str) -> IResult<&str, &str> {
    recognize(
        many_till(space0, alt(( tag("\n"), eof )) )
    )(input)
}


/// TODO : this would actually be the generic line parser
fn line_inner(input: &str) -> IResult<&str, &str> {
    spacey(identifier)(input)
}

fn branch_content(input: &str) -> IResult<&str, Branch> {
    //println!("CONTENT: {:?}", input);
    let(input, _) = opt(spacey(tag("->")))(input) ?;
    let(rest, lines) = separated_list1(
        newline, 
        line_inner
    )(input)?;
    //println!("branch_content {:?} {:?}", rest, bla);

    let branch = Branch {
        lines: lines.iter().map(|x|x.to_string()).collect() 
    };

    Ok((rest, branch))
}

/// parses out blocks containing multiple choices
fn choices_block(input: &str) -> IResult<&str, Choice, > {

    let (input, branches) = 
        separated_list1(
         newline, 
         branch_content
        
    )(input) ?;

    Ok((input, Choice{branches}))
}


///parses blocks containing multiple choices, seperate by empty lines
/// TODO: this not QUITE right, as we would need to only use this when starting with a choice thingy '->'
fn root_blocks(input: &str) -> IResult<&str, Vec<Choice>> {
    separated_list1(
        seperator_empty_lines,  choices_block // alt((seperator_empty_lines, seperator_other))
    )(input)
}


pub fn parse_all_yeah(input: &str) -> IResult<&str, Vec<Choice>> {
    root_blocks(input)
}
