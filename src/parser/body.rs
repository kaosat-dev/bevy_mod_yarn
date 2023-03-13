
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

use super::{YarnCommand, Statements, Dialogue, Choice, Branch, variable_identifier};
use super::{spacey, parse_params, identifier, tag_identifier };

// TODO, replace parse_params with an EXPRESSION
pub fn yarn_commands(input: &str) -> IResult<&str, YarnCommand> {
    let (input, params) = delimited(spacey(tag("<<")), take_until(">>"), spacey(tag(">>")))(input)?;
    let (_, params) = parse_params(params)?;
    let command = YarnCommand{name: params[0].to_string(), params: params[1..].join(","), ..Default::default()};
    Ok((input, command))
}

pub fn statement_command(input: &str) -> IResult<&str, Statements> {
    map(spacey(yarn_commands), |command: YarnCommand| Statements::Command(command))(input)
}


pub fn statement_choice(input: &str) -> IResult<&str, Statements> { //(Statements, usize)
    let (input, indentations)= take_until("->")(input)?;     //tuple(( many0_count(space0), tag("->") ))(input)?;
    let (input, _) = tag("->")(input)?;
    let (input, rest) = till_end(input)?;
    let (input, dialogue) = statement_dialogue(rest)?;
    let choice = Statements::ChoiceBranch(Branch{ statements: vec![dialogue], ..Default::default() });
    Ok(( input, choice))
    //Ok(( input, (dialogue, indentations.len()) ))
}


// TODO, replace parse_params with an EXPRESSION
pub fn if_start(input: &str) -> IResult<&str, &str> {

    let (input, inside_if) = delimited( spacey(tag("<<if")), take_until(">>"), tag(">>"))(input) ?;
    println!("inside_if {} ", inside_if);
    
    /*let (input , _) = tag("<<if ")(input)?;
    let (input, bla) = take_until(">>")(input)?;
    let (input, expression) = parse_params(input)?;
    let (input , _) = tag(" >>")(input)?;*/
    Ok((input, inside_if))
}

pub fn yarn_conditionals(input: &str) -> IResult<&str, Vec<&str>> {
    // FIXME: delimited here is wrong, we do not want to discard the content of if_start
    let (input, params) = delimited(spacey(if_start), take_until("<<endif>>"), tag("<<endif>>"))(input)?;
    println!("conditional body: {}", params);
    // let (input, params) = parse_params(params)?;
    Ok((input, vec![params]))
}


/// node tags are only allowed at the END of a line
pub fn node_tags(input: &str) -> IResult<&str, &str> {
    // let (input, _) = tag("#")(input)?;
    let (input, aa) = take_until("#")(input)?;// separated_list0(tag("#"), identifier)(input) ?;
    println!("bla bla {:?}", aa);
    let (input, tag) = take_until(" ")(input)?;
    println!("tag result {:?}",tag);
    Ok((input, tag))
}

pub fn till_end(input: &str) -> IResult<&str, &str> {
    terminated(not_line_ending, alt(( tag("\n"), eof ))) (input) //take_until("\n")(input)?;
}


pub fn rest (input: &str) -> IResult<&str, &str> {
    // CAREFULL ! swallows the whole input
    Ok(("", input))
}


/// a bit more complex
/// [wave size=2]Wavy![/wave] size=2 is an expression, an assignment expression
pub fn attributes(input: &str) -> IResult<&str, (String, Vec<&str>)> {
    // this is a special one, as we want to extract the tags, but keep the rest of the text
    let mut withouth_attributes:Vec<&str>= vec![];
    let mut attributes:Vec<&str>= vec![];
    let (input , before_attribute) = take_until("[")(input)?;
    withouth_attributes.push(before_attribute);

    let (input, attribute_name) = delimited(tag("["), identifier, tag("]"))(input)?;
    attributes.push(attribute_name);
    //println!("ATTRIBUTES start input {}, attribute_name {}", input, attribute_name);
    let (input , inside) = take_until("[/")(input)?;
    withouth_attributes.push(inside);

    let (input, closing_attribute_name) = delimited(tag("[/"), identifier, tag("]"))(input)?;
    // println!("ATTRIBUTES end input {}, bla {}", input, closing_attribute_name);
    withouth_attributes.push(input);
    // TODO: detect un matching attribute names & throw an error ?
    println!("text withouth attributes {:?}",withouth_attributes.join(" "));
    println!("attributes {:?}", attributes);

    let text_withouth_attributes = withouth_attributes.join(" ");

    Ok((input, (text_withouth_attributes, attributes)))
}


/// https://github.com/YarnSpinnerTool/YarnSpinner/blob/main/Documentation/Yarn-Spec.md#dialogue-statement
/// we want to return the text BEFORE and AFTER the tagged part
/// possible solutions: peek, success, consume
/// alternate: line terminated((alt("{", "<<<",  ) ))
pub fn interpolated_value(input: &str) -> IResult<&str, &str> {
    delimited(tag("{"), variable_identifier, tag("}"))(input)
}

pub fn statement_dialogue_who_what(input: &str) -> IResult<&str, Statements> {
    let (input, (who, _, what)) = tuple((spacey(identifier), spacey(tag(":")), alt((till_end, rest))))(input)?;
    let result =  Statements::Dialogue(Dialogue { who: who.to_string(), what: what.to_string(), ..Default::default() });
    Ok((input, result))
}

pub fn statement_dialogue_what(input: &str) -> IResult<&str, Statements> {
    let (input, what) = till_end(input)?;
    let result =  Statements::Dialogue(Dialogue { who: "nobody".to_string(), what: what.trim_start().to_string(), ..Default::default() });
    Ok((input, result))
}

// (identifier :) (optional) text \n
pub fn statement_dialogue(input: &str) -> IResult<&str, Statements> {
    let (input, result) = 
    alt(
        (statement_dialogue_who_what, statement_dialogue_what)
    )(input) ?;
    // here we have the who + what combo, so we can extract special character like tags etc
    Ok((input, result))
}


// fixme: not sure 
fn statement_empty_line(input: &str) -> IResult<&str, Statements> {
    let (input, _) = empty_line(input)?;
    Ok((input, Statements::Empty))
}


pub fn hashtags(input: &str) -> IResult<&str, Vec<&str>> {
    many0(spacey(tag_identifier))(input)
}

pub fn get_indentation(input: &str) -> IResult<&str, usize> {
    let mut identation = 0;
    // FIXME: damn, whitespace counting needs to include the choice's ->
   
    if input.contains("->") {
        let (bli, (pre_space, tag, post_space, _)) = tuple(( space0, tag("->"),  space0, not_line_ending ))(input)?;
        //println!("WHITESPACE TEST2: {} {:?}", bli, (pre_space.len(), tag.len(), post_space.len()));
        identation = pre_space.len() + tag.len() + post_space.len();
    }else {
        let (_, (white_spaces, rest_of_line)) = tuple(( space0, not_line_ending))(input)?;
        //println!("WHITESPACE TEST: {} {:?}", white_spaces.len(), rest_of_line);
        identation = white_spaces.len();

    }

  Ok(("", identation))
}


// see https://github.com/YarnSpinnerTool/YarnSpinner/blob/040a2436d98e5c0cc72e6a8bc04e6c3fa156399d/Documentation/Yarn-Spec.md#body
pub fn statement_base_line(input: &str) -> IResult<&str, (&str, Vec<&str>, usize)> {
    let (input, content) = terminated(not_line_ending, alt(( tag("#"),tag("\n"))) )(input)?;
    //extract white spaces/indentation
    let (_, indentation) = get_indentation(content)?;
    let (hashtags_raw, con) =  opt(take_until("#"))(content)?;
    let mut tags: Vec<&str> = vec![];
    let mut result = content; // FIXME this whole thing is terrible
    if let Some(c) = con {
        if let Ok(_tags) = hashtags(hashtags_raw) {
            tags = _tags.1;
        }
        result = c;
    }
    Ok((input, (result, tags, indentation)))
}


// see https://github.com/YarnSpinnerTool/YarnSpinner/blob/040a2436d98e5c0cc72e6a8bc04e6c3fa156399d/Documentation/Yarn-Spec.md#body
// returns a Vec<(content, Vec<Tags>)
// ie each line with its tags
pub fn statement_base(input: &str) -> IResult<&str, Vec<(&str, Vec<&str>, usize)>> {
    many1(statement_base_line)(input)
}

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

/*
Alternative impl
do not directly add statements, go through an itermediary data structure
- clear (blank_line /eof) => pops all items until it reaches 0
- add (indentlevel(ie choice stack level), statement )*/
enum HelperOps{
    Add((usize, Statements)),
    Clear,
    None
}
 
// TODO: do not just pop arbitrarly on < identation
// keep track of indentation levels !!!


/// wraps all the rest
pub fn body(input: &str) -> IResult<&str, Branch> {
    let (input, lines) = statement_base(input)?; // TODO: use nom's map

    let mut root_branch : Branch = Branch { statements: vec![], ..Default::default() };  // this is the root branch after the end of the parsing
    let mut choices_stack: Vec<Choice> = vec![];
    let mut current_branch:& mut Branch;
    current_branch = &mut root_branch;
    // current_branch.statements.push(Statements::Empty);
    // remember choice "groups" are delimited by : 
    // - empty white line
    // - a different indentation 

    let mut previous_indentation:usize = 0;
    let mut previous_choice_indentation:usize = 0;

    let mut nesting_level= 0;

    let mut helper = HelperOps::None;

    for (index, (line, tags, indentation)) in lines.iter().enumerate() {
        // the order of these is important !!
        let (_, statement) = alt((
            statement_empty_line,
            statement_command,
            statement_choice,
            statement_dialogue_who_what,
            statement_dialogue_what,
        )
        )(line)?;
        let tags: Vec<String> = tags.clone().iter().map(|x|x.to_string()).collect();
        let indentation = indentation.clone();

        match statement.clone(){
            Statements::ChoiceBranch(mut branch) => {
                // let clone = branch.clone();
                // current_branch = &mut branch;
                // IF non nested, the branch is on the same level as previous branches
                if indentation > previous_choice_indentation {
                    println!("higher level, we need to nest !");
                    choices_stack.push(Choice { branches: vec![branch], ..Default::default() });
                }else if indentation == previous_choice_indentation {
                    println!("same level , add another branch");
                    // push the previous choice branch to the list of branches in the choice
                    choices_stack.last_mut().expect("we should always have one item in the stack here").branches.push(branch);
                  
                }else {
                    println!("lower level leave this branch");
                    let choice = choices_stack.pop().unwrap();

                    // FIXME: always the same stuff, of different "current branch" if we are at root level or nested in a Choice
                    if choices_stack.len() > 0 {
                        choices_stack.last_mut().unwrap().branches.last_mut().unwrap().statements.push(Statements::Choice(choice));
                        choices_stack.last_mut().unwrap().branches.push(branch);
                     }else {
                        root_branch.statements.push(Statements::Choice(choice));
                   }
                }
                previous_choice_indentation = indentation.clone();
            }
            Statements::Empty => {
                // println!("blank line");
                // Empty lines reset the whole stack to 0: ie we should pop everything one by one to close off the options
                // FIXME: how to dedupe this ? closures do not work , external functions do not work
                helper = HelperOps::Clear;
            }
            _=> {
                // we push everything else to the current branch
                if indentation < previous_indentation {
                    println!("poping");
                    if choices_stack.len()> 0 {
                        let choice = choices_stack.pop().unwrap();
                        if choices_stack.len() > 0 {
                            choices_stack.last_mut().unwrap().branches.last_mut().unwrap().statements.push(Statements::Choice(choice));
                        }else {
                            root_branch.statements.push(Statements::Choice(choice));
                        }
                    }
                }
                if choices_stack.len() > 0 {
                    choices_stack.last_mut().unwrap().branches.last_mut().unwrap().statements.push(statement);
                }else {
                    root_branch.statements.push(statement);
                }
            }
        }

        // generic handling, outside of specific cases
        if indentation < previous_indentation {
            //println!("lower level leave this branch");
        }
        previous_indentation = indentation.clone();

        if index == lines.len() - 1 {
            helper = HelperOps::Clear;
        }

        match helper {
            // we still have items on the stack after finishing everything, so close them off until there is none left
            HelperOps::Clear => {
                if choices_stack.len() > 0 {
                    // println!("remaining {}, poping", choices_stack.len());
                    // FIXME: horrible implementation 
                    let mut child = choices_stack.pop().unwrap();
                    if choices_stack.len()> 0 {
                        while let Some(mut parent_choice) = choices_stack.pop() {
                            println!("in loop");
                            if choices_stack.len()> 0 {
                                parent_choice.branches.last_mut().unwrap().statements.push(Statements::Choice(child.clone()));
                                child = parent_choice;
                            }else {
                                parent_choice.branches.last_mut().unwrap().statements.push(Statements::Choice(child.clone()));
                                root_branch.statements.push(Statements::Choice(parent_choice.clone()));
                            }
                        }
                    }else {
                        root_branch.statements.push(Statements::Choice(child.clone()));
                    } 
                }
            }
            _ => {}
        }
    }
    // lines done 
    // here root_branch should be the root branch
    root_branch.statements.push(Statements::Exit);
    Ok((input, root_branch))
}


pub fn display_dialogue_tree(branch: &Branch, indentation_level: usize) {
    let identation_pattern = "  ";
    let identation = format!("  {}", identation_pattern.repeat(indentation_level));
    for statement in branch.statements.iter(){
        match statement {
            Statements::Choice(choice) => {
                println!("{}statement choices ({}): tags:{:?}", identation, choice.branches.len(), choice.tags);
                for branch in choice.branches.iter() {
                    println!("{}Branch:", identation);
                    display_dialogue_tree(branch, indentation_level +3 );
                }
            }
            _ => {
                println!("{}statement {:?}",identation, statement);
            }
        }
    }
}


// should be empty line OR eof
fn empty_line(input: &str) -> IResult<&str, &str> {
    recognize(
        many_till(space0, alt(( tag("\n"), eof )) )
    )(input)
}

