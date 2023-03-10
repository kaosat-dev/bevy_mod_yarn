
fn namespaced_identifier(input: &str) -> IResult<&str, &str> {
    /*let foo = recognize(
        identifier,
        tag(":"),
        identifier
    )(input)?;*/
    let (input, id) = identifier(input)?;
    // let (input, _) = tag(":")(input)?;
    // let (input, id2) = identifier(input)?;
    // let result = format!("{}:{}", id, id2);
    Ok((input, id))
}

fn tag_bla (input: &str) -> IResult<&str, &str> {
    preceded(tag("#"),identifier)(input)
}


fn bliblibli (input: &str) -> IResult<&str, &str> {
    Ok((input, ""))
}
 //let (input, data ) = separated_list1(tag("==="), anychar)(input)?;
    //Ok((input, data))
    // input.split_at_position_complete(tag("===")) // char::is_whitespace
    // take_till(|c| c == "===")(input)
    // preceded(multispace0, take_until(tag(".rar")))
    //let (input, node_raw) = take_until("===")(input)?;
    //let (input, second_raw) = take_until("===")(input)?;

    // let (input, out ) = separated_list1(tag("==="), anything)(input)?;
    //many0(take_until("==="))

       //let (input, node) = separated_list1(tag(node_seperator), line)(input)?;
        // separated_list1(newline(input), line)(input)?;

fn foo () {
    let (input, tags) = opt(
        alt((
            separated_list0(tag(" "), spacey(tag_identifier)), // either a whitespace seperate list
            many0(spacey(tag_identifier)), // or a single item
        ))
        )(input)?;
        
        let (input, tags) = many0(spacey(tag_identifier))(input)?;
}


pub fn inner(input: &str) -> IResult<&str, Vec<&str>> {

    let (input, res) = many_till(  delimited( 
        spacey(tag("(")), 
        identifier, 
        spacey(tag(")"))
    ),  alt(( tag("\n"), eof ))  )(input)?;

    Ok((input, res.0))
}


fn nesting_test() {
    let test_input = "
    (aa)(fine)
    (dsf)(sdf)(sdfezae)(azezae)
(other)";
    let (input, bli) = many_till(inner,  eof  )(test_input) ?;
}





pub fn inner(input: &str) -> IResult<&str, Vec<&str>> {

    let (input, res) = many_till(  delimited( 
        spacey(tag("(")), 
        identifier, 
        spacey(tag(")"))
    ),  alt(( tag("\n"), eof ))  )(input)?;

    Ok((input, res.0))
}


pub fn inner2(input: &str) -> IResult<&str, &str> {

    let (input, _bla) = spacey(tag("aaa"))(input)?;
    /*let (input, res) = many_till(  delimited( 
        spacey(tag("(")), 
        identifier, 
        spacey(tag(")"))
    ),  empty_line  )(input)?;*/

    Ok((input, ""))
}


pub fn ggdgdfgdf(input: &str) -> IResult<&str, (&str, &str, &str, &str, &str)> {
    tuple(( spacey(tag("aaa")) ,tag("\n"), spacey(tag("aaa")),tag("\n"), empty_line2 ))(input)
}

pub fn six(input: &str) -> IResult<&str, (&str, &str, &str, &str, &str)> {
    tuple(( spacey(identifier) , alt(( tag("\n"), eof )), spacey(identifier), alt(( tag("\n"), eof )), alt(( empty_line2, eof )) ))(input)
}

pub fn seventh_inner(input: &str) -> IResult<&str, &str> {
    terminated(spacey(identifier), tag("\n")) (input)
}

pub fn seventh(input: &str) -> IResult<&str, (&str, &str)> {
    terminated(
        tuple(( seventh_inner ,tag("\n") )),
        alt(( empty_line2, eof ))
    )(input)
    // separated_list1(sep, f)
    //pair(first, second)
}

pub fn eighth_inner(input: &str) -> IResult<&str, (&str, &str)> {
    // terminated(spacey(identifier), tag("\n")) (input)
    tuple(( 
        spacey(identifier) , 
        alt(( tag("\n"), eof ))
    ))(input)
}

pub fn eigth(input: &str) -> IResult<&str, ((&str, &str), (&str, &str), &str)> {
    tuple((
        eighth_inner,
        eighth_inner, // this does not work many0(eighth_inner),
        alt(( empty_line2, eof ))
    ))
    (input)
}


pub fn ninth(input: &str) -> IResult<&str,(Vec<(&str, &str)>, &str)> {
    many_till(eighth_inner, alt(( empty_line2, eof )))(input)
}


pub fn lines_test(input: &str) -> IResult<&str, Vec<&str>> {
    // let (input, bla) =separated_list1(newline, line)(input)?;

    let test_input = "
    (aa)(fine)
    (dsf)(sdf)(sdfezae)(azezae)
       (other)";
    let (input, bli) = many_till(inner,  eof  )(test_input) ?;
    /* returns a 
    result: [
        ["aa", "fine"], 
        ["dsf", "sdf", "sdfezae", "azezae"]
    ]
    now imagine if those where Vec<Statements> / branches ...
     */

    println!("FIRST TEST: input:{} /// result: {:?}", input, bli.0);


    /*let test_input = "aaa
    aaa

    aaa";
    let(input , bli) = many_till(inner2,  eof  )(test_input)?;
    println!("SECOND TEST: input:{} /// result: {:?}", input, bli.0);*/

    let test_input ="                    \n";
    let bli = empty_line2(test_input);
    println!("THIRD TEST: result: {:?}", bli);


 let test_input = "aaa
 aaa

 ";
    let (input, bli) = ggdgdfgdf(test_input) ?;
    println!("FOURTH TEST: input:{} /// result: {:?}", input, bli);


    let test_input = "aaa
    aaa
   
   aaa
   aaa
    ";
    let (input, bli) = many0(ggdgdfgdf)(test_input) ?;
    println!("FIFTH TEST: input:{} /// result: {:?}", input, bli);

    let test_input = "foo
    bar
    
    baz
    biz
    
    ";
    let (input, bli) = many0(six)(test_input) ?;
    println!("SIXTH TEST: input:{} /// result: {:?}", input, bli);

    // FAILS
    let test_input = "foo
    bar
    
    baz
    biz
    ";
    let (input, bli) = many0(seventh)(test_input) ?;
    println!("SEVENTH TEST: input:{} /// result: {:?}", input, bli);

    let test_input = "foo
    bar

    baz
    biz
    ";
    let (input, bli) = many0(eigth)(test_input) ?;
    println!("EIGHT TEST: input:{} /// result: {:?}", input, bli);

    let test_input = "
    foo
    bar

    baz
    biz
    ";
    let (input, bli) = many0(ninth)(test_input) ?;
    println!("NINTH TEST: input:{} /// result: {:?}", input, bli);


    let bla = vec![];
    Ok((input, bla))
}



fn whitespaces_count(input: &str) -> usize {
    input
        .chars()
        .take_while(|ch| ch.is_whitespace() && *ch != '\n')
        .count()
}

pub fn hack_parse_choice_block(input: &str) -> IResult<&str, &str> {
    //let (input, white_spaces) =  many0_count(tag(" "))(input)?;
    //let (input, (white_spaces, _) ) = tuple(( many0_count(space0), tag("->") ))(input)?;
    let (input, (white_spaces, _) ) = tuple(( many0_count(tag(" ")), tag("->") ))(input)?;
     println!("HACK {} whitespaces: {}", input, white_spaces);

    // hack to find an choice block ending with a blank line
    let lines_raw: Vec<&str> = input.split("\n").collect();
    let mut matching_index = 0;
    
    for (index, line) in lines_raw.iter().enumerate() {
        println!("line {}", line);
        if is_line_empty(format!("{}\n", line).as_str()) { // horrible
            /*if whitespaces_count(line) == white_spaces  {
                println!("matching whitespaces");
            }*/
            println!("found an empty line, we are at end of choice block");
            matching_index = index;
            break;
        }
    }

    let binding = lines_raw[0..matching_index].join("\n");
    let mut block:&str = binding.as_str();
    let rest = lines_raw[matching_index..].join("\n").as_str(); // impossible to return, we would need to get to the same line index using the nom parsers & return that remain


    // let (input, _) = many0(take_until(newline))(input)?;//separated_list1(newline, line)(input)?;

    println!("block {}", block);
    

    Ok(("", ""))
}

/* 
fn seperator_empty_lines(input &str) -> IResult<&str, &str> {

}*/

/*

let yarn_text =      "Bob: Hi !
Grumpy: Grumble !
Bob: Oh hello there grumpy !
Dona: fine and you ?

-> block1_choiceA: grumble fsdf sdfsdf sdfds fsd sdf sdf 
fg: fuck off
<<jump off cliff>>
-> block1_choiceB: some dialogue
<<jump node_con>>

-> block2_choiceA
dfs
df
qsdqsd

-> block3_choiceA
-> block3_choiceB
    sdfdsf
    sdf
-> block3_choiceC
";
if let Ok((_, tree)) = parse_all_yarn(yarn_text) {
    println!("blocks {}", tree.len());
    for choice in tree.iter(){
        println!("choice: ");
        for branch in choice.branches.iter() {
            println!("   branch ");
            for line in branch.lines.iter() {
                println!("       line {:?}", line);
                
            }
        }
    }
    //    println!("parse_all_yeah {:?}", parse_all_yarn(

}else {
    println!("failed to parse");
} */

 // we reverse iterate two by two do un pop & create things as they should be
        /*let mut two_by_two = choices_stack.iter().rev()
            .zip(choices_stack.iter().rev().skip(1))
            .collect::<Vec<_>>();
        for (cur, mut prev) in two_by_two.iter_mut() {
            println!("cur {:?} prev {:?}", cur, prev);
            prev.branches.last_mut().unwrap().statements.push(
                Statements::Choice(**cur)
            );
            //current_branch.statements.push(Statements::Choice(choice));

        }*/