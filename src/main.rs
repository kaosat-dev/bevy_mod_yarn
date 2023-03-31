use std::{fs, collections::HashMap};

mod parser;

use bevy_mod_yarn::parser::interpolated_value;

use crate::{parser::{title, yarn_commands, identifier, tag_identifier, variable_identifier, statement_dialogue, parse_params, yarn_conditionals, header_tags, attributes, header, parse_yarn_nodes_nom, statement_base, statement_choice, body::{self, display_dialogue_tree}}};

fn main() {
    /*println!("title {:?}",title("title: Start\n"));
    println!("title {:?}",title("title: Other_node\n"));
    println!("title {:?}",title("   title   :   Other_node\n"));
    println!("title {:?}",title(" title: Other_node   \n"));
    println!("title {:?}",title(" title: Or_nodeç:sdfsdfs___sdf   \n")); // INVALID only letters, numbers & underscores allowed
    println!("title {:?}",title(" title: __Or_nodeçsdfsdfs___sdf   \n")); // INVALID: underscore a the start, a node title must start with a letter


    println!("header_tags {:?}",header_tags("tags: \n"));
    println!("header_tags {:?}",header_tags("  tags: \n"));
    println!("header_tags {:?}",header_tags("  tags: #blabla\n"));
    println!("header_tags {:?}",header_tags(" tags: #camera2 #background:conductor_cabin \n"));
    println!("header_tags {:?}",header_tags("  tags: blabla\n")); // again yarn's "no need for pound for tags in the header vs tags in the body ..."
    println!("header_tags {:?}",header_tags(" tags: #camera2 background:conductor_cabin \n")); // same


    println!("header {:?}",header("
    title: Bar
tags: #my_tag other and_another
colorID: 0
position: 567,-265
---
    "));



    // commands
    println!("commands {:?}", yarn_commands("<<stop>>"));
    println!("commands {:?}", yarn_commands("<<wait 2>>"));
    println!("commands {:?}", yarn_commands("<<say hello>>"));
    println!("commands {:?}", yarn_commands("  <<say hello  >>  "));
    println!("commands {:?}", yarn_commands("<<jump Other_node>>"));
    println!("commands {:?}", yarn_commands("<<fade_out 1.5>>"));
    println!("commands {:?}", yarn_commands("<<set $gold_amount to 5>>"));

    // identifiers
    println!("identifier {:?}", identifier("foo"));
    println!("identifier {:?}", identifier("foo_bar"));
    println!("identifier {:?}", identifier("#_foo_bar"));// INVALID

    // tag identifiers
    println!("tag_identifier {:?}", tag_identifier("#_foo_bar"));
    println!("tag_identifier {:?}", tag_identifier("#foo:bar"));
    println!("tag_identifier {:?}", tag_identifier("#foo:bar_baz:biz:boz"));
    println!("tag_identifier {:?}", tag_identifier("#camera2"));
    println!("tag_identifier {:?}", tag_identifier("#background:conductor_cabin"));

    // variable identifiers
    println!("variable_identifier {:?}", variable_identifier("$_foo_bar"));
    println!("variable_identifier {:?}", variable_identifier("_foo_bar")); // INVALID
    println!("variable_identifier {:?}", variable_identifier("$_foo.bar.baz"));

    //
    println!("attributes {:?}", attributes("Oh, [wave]hello[/wave] there!"));
    println!("attributes {:?}", attributes("Oh, [wave]hello[/wave] there! [dance] party time ![/dance] "));

    // params (FIXME: actually expressions)
    println!("parse_params {:?}", parse_params("$gold_amount == 10"));
    println!("parse_params {:?}", parse_params("$gold_amount < 10"));
    println!("parse_params {:?}", parse_params("$gold_amount > 10"));
    println!("parse_params {:?}", parse_params("$gold_amount >= 10"));
    println!("parse_params {:?}", parse_params("$gold_amount <= 10"));

   
    // line variations

    // simple line cases
    println!("statement_dialogue {:?}", statement_dialogue("  Lamik:Hi I said to him"));
    println!("statement_dialogue {:?}", statement_dialogue("  Lamik:  Hi I said to him\n"));
    println!("statement_dialogue {:?}", statement_dialogue("Lamik:Hi I said to him\n"));
    println!("statement_dialogue {:?}", statement_dialogue("  Lamik:  Hi I said to him ____sdfd__\n"));
    println!("statement_dialogue {:?}", statement_dialogue("Hi I said to him\n"));
    println!("statement_dialogue {:?}", statement_dialogue("Hi I said to him, withouth new line"));

    // simple line cases with tags
    println!("statement_dialogue {:?}", statement_dialogue("Homer: Hi, I'd like to order a tire balancing. #sarcastic #duplicate\n"));
    println!("statement_dialogue {:?}", statement_dialogue("Homer: Hi, I'd like to order a tire balancing. #tone:sarcastic #duplicate\n"));

    println!("statement_choice {:?}", statement_choice("->  Lamik: not so great, sadly :(\n"));
    println!("statement_choice {:?}", statement_choice("    ->  Lamik: not so great, sadly :(\n"));
    println!("statement_choice {:?}", statement_choice("    ->  Lamik: not so great, sadly #tag :(\n"));

    
    // conditional 
    // println!("yarn_conditionals {:?}", yarn_conditionals("<<if>>dfsdfs<<endif>>"));

    
    println!("yarn_conditionals {:?}", yarn_conditionals("-> Sure I am! The boss knows me! <<if $reputation > 10>>"));
    println!("yarn_conditionals {:?}", yarn_conditionals("<<if$gold_amount < 10>>Baker: Well, you can't afford one!<<endif>>")); // INVALID
    println!("yarn_conditionals {:?}", yarn_conditionals("<<if $gold_amount < 10 >> Baker: Well, you can't afford one! <<endif>>"));
    println!("yarn_conditionals {:?}", yarn_conditionals("<<if $gold_amount < 10 >> Baker: Well, you can't afford one! <<else>> Baker: Here you go!  <<endif>>"));
    */
    // simple line with character
    /* 
    assert_eq!(
        statement_dialogue("Lamik: Hi there !"), 
        Ok((
            "", 
            Statements::Dialogue(
                Dialogue { who: "Lamik".into(), what: "Hi there !".into()}
            )
        ))
    );*/
    
    //         opt(space0),

    // println!("header tags: {:?}",header_tags("  tags: sdf_1 zerze  #bla"));
    // commands

    // println!("AAAAAH {:?}", hex_color_final(""));

    /*println!("parse_foo {:?}", parse_foo("-> opt1
    fg
    opt2
    bar

    -> other
    dfs
    df
    "));

    let bla = "-> block1_choiceA
    fg
    -> block1_choiceB
    bar

    -> block2_choiceA
    dfs
    df
    ";
    println!("parse_foo2 {:?}", parse_foo2( "-> block1_choiceA
    fg
    -> block1_choiceB
    bar

    -> block2_choiceA
    dfs
    df
    "));*/

    // println!("parse_bar {:?}",parse_bar("") );
   
   let foobazbar = "Lamik: Hi there !
   Dona: Hello !
   Lamik: how are you doing ?
   <<say hello>> #command_tag_what_the
   Bob: Hi !
   Grumpy: Grumble !
   Bob: Oh hello there grumpy ! #grumpy_thingy
   Dona: fine and you ?
   ->  Lamik: doing ok #an_option_tag
       Dona: good to hear :)
       Grumpy: whatever !
       Bob: cool cool cool !
       Dona: let's have a party then !
       Grumpy: NO!
       Bob: sure, whatever..
       Lamik: yeah !
       <<jump Other_node>>
   ->  Lamik: not so great, sadly :(
       Dona: oh, what is the matter ?
       Lamik: have not caught any crabs.
   
   Lamik: well this was nice, but I am tired now
   Dona: oh ok then
   -> Lamik: want me to stay ?

   -> and another!

   ===";
   let test_text = "some: text here  \n some other text there#hash#otherhash \n something that ends sooner #hashtag #other\n -> a CHOICE ! \n << dsfdsf sdf>> \n sqd[wave]sdfds[/wave] \n";

   //println!("statement_base {:?}", statement_base("some text here  \n some other text there#hash#otherhash \n something that ends sooner #hashtag #other\n -> dsfdsf \n << dsfdsf sdf>> \n sqd[wave]sdfds[/wave] \n"));
   //println!("foobur {:?}", body(foobazbar));

    let other_text = "Lamik: Hi there !
    <<say hello>>
    Dona: Hello !
    ->  Lamik: doing ok

    ";



    /* 
        different cases for nesting
    // basic_eof
    A: how are you
    -> B: fine
    -> B: not ok
    EOF

    basic_blank_line
    A: how are you
    -> B: fine
    -> B: not ok
    BLANK_LINE
    EOF

    // basic_eof + 
    A: how are you
    -> B: fine
       A: cool
    -> B: not ok
       A: oh no
    EOF

    basic_blank_line +
    A: how are you
    -> B: fine
       A: cool
    -> B: not ok
       A: oh no
    BLANK_LINE
    EOF

    // dedent_eof 
    A: how are you
    -> B: fine
    A: unrelated // dedent: ends the choice above
    -> B: another choice
       A: oh no
    EOF

    // multi_indent_eof 
    A: how are you
    -> B: fine
        -> B: a layer
            -> B: another layer // here we would have pop the state & close options until we are back at level 0
    EOF


    */
    println!("interpolated {:?}", interpolated_value("you now have {$coins},  congratulations !"));

    let file_path = "./assets/micro.yarn"; // simple, micro minimal.yarn barebones.yarn

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let parsed = parse_yarn_nodes_nom(&contents);
    for (_node_name, node) in parsed.iter() {
        println!("NODE({}):", node.title);
        println!("  Statements tree");
        display_dialogue_tree(&node.branch, 1);
    }

    let file_path = "./assets/other.yarn"; // simple, micro minimal.yarn barebones.yarn
    println!("indentation return");
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let parsed = parse_yarn_nodes_nom(&contents);
    for (_node_name, node) in parsed.iter() {
        println!("NODE({}):", node.title);
        println!("  Statements tree");
        display_dialogue_tree(&node.branch, 1);
    }

    let file_path = "./assets/complex.yarn"; // simple, micro minimal.yarn barebones.yarn
    println!("COMPLEX");
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let parsed = parse_yarn_nodes_nom(&contents);
    for (_node_name, node) in parsed.iter() {
        println!("NODE({}):", node.title);
        println!("  Statements tree");
        display_dialogue_tree(&node.branch, 1);
    }

   
   
    println!("the end");
   //  parser();
}




