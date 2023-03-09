pub fn body(input: &str) -> IResult<&str, Branch> {
    let (input, lines) = statement_base(input)?; // TODO: use nom's map

    let mut current_branches: Vec<Branch> = vec![]; // stores the data for the current choice node , if any
    let mut current_branch : Branch = Branch { statements: vec![], ..Default::default() };  // this is the root branch after the end of the parsing
    let mut stack: Vec<Branch> = vec![];

    // todo: create a choice when no choice branch is active, append all branches from ChoiceBranches
    // remember choice "groups" are delimited by : 
    // - empty white line
    // - a different indentation 

    // FIXME/ hack, only works for a single level: TODO: use the stack len to determine how deep we are !
    let mut is_in_choice = false;
    let mut previous_indentation:usize = 0;
    for (line, tags, indentation) in lines.iter() {
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
        
        println!("statement  {:?}, tags: {:?}" ,statement.clone(), tags);
        match statement.clone(){
            Statements::ChoiceBranch(branch) => {
                // println!("we have a new branch ! {:?}", branch);
                // IF non nested, the branch is on the same level as previous branches
                if is_in_choice {
                    //println!("already in branch STACK: {:?}", stack);
                    // push the previous choice branch to the list of branches in the choice
                    current_branches.push(current_branch.clone());
                    if stack.len() > 0 {
                        current_branch = stack.pop().unwrap();
                    }
                }else {
                    //println!("not in choice before this");
                    is_in_choice = true;
                }
               
                //println!("current before {:?} STACK: {:?}", current_branch, stack);
                stack.push(current_branch);
                current_branch = branch;
                //println!("current  after {:?}  STACK: {:?}", current_branch, stack);
                println!("nesting level {}", stack.len());

            }
            Statements::Empty => {

                // IF we had an open CHOICE still gathering branches, add a choice with all current branches
                if is_in_choice {
                    current_branches.push(current_branch.clone());
                    // println!("gathering branches into a choice {:?}", current_branches);
                }
                is_in_choice = false; 


                //If we have an empty line, OR if the IDENTATION IS LESS pop back to the previous level branch
                if stack.len() > 0 {
                    current_branch = stack.pop().unwrap();
                    //println!("going back to {:?} ", current_branch);

                    if current_branches.len() > 0 {
                        current_branch.statements.push( // need to be pushed to the parent branch, so that is why we pop() first
                            Statements::Choice(crate::Choice { branches: current_branches.clone() , ..Default::default()} )
                        );
                    }   
                }
                println!("nesting level {}", stack.len());

                current_branches = vec![];
            }
            _=> {
                /*  if indentation < previousIndentation {
                    // TODO: end choice gathering if any
                }*/
                // we push everything else to the current branch
                current_branch.statements.push(statement);
                println!("nesting level {}", stack.len());
            }
        }
        previous_indentation = indentation.clone();
    }
    // push the last branch
    // current_branches.push(current_branch);
    // println!("stack, {:?}", stack);
    // here current_branch should be the root branch
    // display_dialogue_tree(&current_branch, 0);

    /*
    for branch in current_branches.iter(){
        println!("branch: ");
        for statement in branch.statements.iter(){
            println!("    {:?}", statement);
        }
    } */


    Ok((input, current_branch))
}
