use nom::{
    character::complete::{alphanumeric1, newline, space0, not_line_ending},
    multi::{separated_list1, many_till},
    sequence::{pair, preceded, terminated, tuple},
    IResult,
    combinator:: {recognize, eof},
    branch:: {alt},
    bytes::complete::{tag, take_until}
};
use nom::combinator::opt;
use crate::{Statements, Dialogue};

use super::{identifier, spacey, statement_dialogue, statement_command};
use nom_supreme::error::ErrorTree;

#[derive(Debug)]
pub struct Branch2  {
    pub lines: Vec<Statements>
}

#[derive(Debug)]
pub struct Choice {
    pub branches: Vec<Branch2>
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


fn line_w(input: &str) -> IResult<&str, Statements> {
    let (input, what) = spacey(not_line_ending)(input)?;
    println!("TUTU {:?} INPUT {:?}", what, input);
    let result =  Statements::Dialogue(Dialogue { who: "nobody".to_string(), what: what.to_string(), ..Default::default() });
    Ok((input, result))
}

fn line_ww(input: &str) -> IResult<&str, Statements> {
    let (input, what) = tuple( (spacey(identifier) , spacey(tag(":")), not_line_ending  ))(input)?;
    let (who, _, what) = what;
    println!("TOTO {:?} INPUT {:?}", what, input);

    let result =  Statements::Dialogue(Dialogue { who: who.to_string(), what: what.to_string(), ..Default::default() });
    Ok((input, result))
}

fn line_test(input: &str) -> IResult<&str, Statements> {

   let (input, result) = alt((
        line_ww,
        line_w,
        //not_line_ending,
    ))
    (input)?;
 

    Ok((input, result))
}

fn foobazbar0(input: &str) -> IResult<&str, Statements> {
    let (input, what) = spacey(identifier)(input)?;

    let result =  Statements::Dialogue(Dialogue { who: "nobody".to_string(), what: what.to_string(), ..Default::default() });

    Ok((input, result))
}


/// TODO : this would actually be the generic line parser
fn line_inner(input: &str) -> IResult<&str, Statements> {
     
     let (input, what) =  alt((
        statement_command,
        line_ww,
        // line_w,
        foobazbar0,
       // spacey(line_w)
     ))
        (input)?; // this could be a good escape hatch
    // println!("taking {:?}", what);

    // let (input, what) =  not_line_ending(input)?; // this could be a good escape hatch
    
    // let (rest, ww) = line_test(what)?;
    //tuple(( spacey(identifier), spacey(tag(":")) ))(what)?;
    // println!("BLA BLA REST: {} {:?}", rest, bla);
    //spacey(identifier)(input)?;
    Ok((
        input,
        what
        // Statements::Dialogue(Dialogue { who: "who".to_string(), what: what.to_string() })
    ))
    /*alt((
        // statement_command,
        spacey(statement_dialogue),
        // spacey(identifier)
    ))(input)*/
}

fn branch_content(input: &str) -> IResult<&str, Branch2> {
    //println!("CONTENT: {:?}", input);
    let(input, _) = opt(spacey(tag("->")))(input) ?;
    let(rest, lines) = separated_list1(
        newline, 
        line_inner
    )(input)?;
    //println!("branch_content {:?} {:?}", rest, bla);

    let branch = Branch2 {lines };

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


pub fn parse_all_yarn(input: &str) -> IResult<&str, Vec<Choice>> {
    root_blocks(input)
}
