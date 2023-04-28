// dump for unused code, will get removed later


use nom::{
    bytes::complete::{tag, is_not, take_till, take_until, take_while_m_n, take_while},
    branch::alt,
    error::ParseError,
    
    IResult, 
    multi::{separated_list1, many0_count, many0, separated_list0, many1, many_till, count}, 
    character::complete::{newline, alphanumeric0, anychar, alpha1, alphanumeric1, multispace0, space0, digit0, one_of, char, line_ending, not_line_ending}, 
    sequence::{delimited, preceded, terminated, pair, separated_pair, tuple }, 
    combinator::{recognize, opt, not, eof, map}, 
    InputTakeAtPosition,
    number::complete::{float, recognize_float}
};
use nom::{
    bytes::complete::{tag, is_not, take_till, take_until},
    branch::alt,
    error::ParseError,
    
    IResult, multi::{separated_list1, many0_count, many0, separated_list0, many1}, 
    character::complete::{newline, alphanumeric0, anychar, alpha1, alphanumeric1, multispace0, space0, digit0, one_of, char}, 
    sequence::{delimited, preceded, terminated, pair, separated_pair, tuple, }, 
    combinator::{recognize, opt, not}, 
    InputTakeAtPosition,
    number::complete::{float, recognize_float}
};


/* 
pub fn get_current_branch(mut choices_stack: Vec<Choice>, current_branch: Branch) -> Branch{

    if choices_stack.len()> 0 {
       return *choices_stack.last_mut()
            .expect("we should always have one item in the stack here")
            .branches.last_mut()
                    .expect("we always have at least one branch")
    }else {
        return current_branch
    }
}*/


pub fn state_pop(mut stack: Vec<Branch>, mut current_branch : Branch, mut current_branches: Vec<Branch>) -> Branch{
    current_branches.push(current_branch.clone());

    if stack.len() > 0 {
        current_branch = stack.pop().unwrap();
        if current_branches.len() > 0 {
            current_branch.statements.push( // need to be pushed to the parent branch, so that is why we pop() first
                Statements::Choice(Choice { branches: current_branches.clone() , ..Default::default()} )
            );
        }   
    }
    println!("nesting level {}", stack.len());

    current_branches = vec![];
    
    current_branch
}
