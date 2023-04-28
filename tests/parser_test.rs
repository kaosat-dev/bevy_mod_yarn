use std::collections::HashMap;

use bevy_mod_yarn::prelude::*;


#[test]
fn test_parse_minimal() {
    let micro = "title: Test_node
    ---
    Dona: what is wrong ?
    Grumpy: ...
    ===
    ";

    let mut expected : HashMap<String,YarnNode> = HashMap::new();
    expected.insert(
        "Test_node".into(), 
        YarnNode { 
            title: "Test_node".into(), 
             branch: {
                Branch {
                    statements: vec![
                        Statements::Dialogue(Dialogue { who: "Dona".into(), what: "what is wrong ?".into(), ..Default::default() }),
                        Statements::Dialogue(Dialogue { who: "Grumpy".into(), what: "...".into(), ..Default::default() }),
                        Statements::Exit,
                    ]
                }
            }, 
             ..Default::default()
        })
        ;
    assert_eq!(parse_yarn_nodes(micro), expected);

    // assert_eq!(yarn_commands("<<stop>>"), Ok(("", vec!["stop"])));
    // assert_eq!(yarn_commands("<<say hello>>"), Ok(("", vec!["say", "hello"])));
    // assert_eq!(yarn_commands("<<jump Other_node>>"), Ok(("", vec!["jump", "Other_node"])));
}


#[test]
fn test_branching_basic_whiteline_seperator(){

    let choices = "title: Test_node
    ---
    it was a beautiful day , said nobody
    Lamik: hi !
    Dona: good morning , how are you ?
    -> Lamik: are you asking me ?
       Dona: yes
    -> Lamik: fine !
       Dona: good to hear

    ===
    ";
    let mut expected : HashMap<String,YarnNode> = HashMap::new();
    expected.insert(
        "Test_node".into(), 
        YarnNode { 
            title: "Test_node".into(), 
             branch: {
                Branch {
                    statements: vec![
                        Statements::Dialogue(Dialogue { who: "nobody".into(), what: "it was a beautiful day , said nobody".into(), ..Default::default() }),
                        Statements::Dialogue(Dialogue { who: "Lamik".into(), what: "hi !".into(), ..Default::default() }),
                        Statements::Dialogue(Dialogue { who: "Dona".into(), what: "good morning , how are you ?".into(), ..Default::default() }),
                        Statements::Choice(Choice { branches: vec![
                            Branch {
                                statements: vec![
                                    Statements::Dialogue(Dialogue { who: "Lamik".into(), what: "are you asking me ?".into(), ..Default::default() }),
                                    Statements::Dialogue(Dialogue { who: "Dona".into(), what: "yes".into(), ..Default::default() }),
                                ]
                            }, 
                            Branch {
                                statements: vec![
                                    Statements::Dialogue(Dialogue { who: "Lamik".into(), what: "fine !".into(), ..Default::default() }),
                                    Statements::Dialogue(Dialogue { who: "Dona".into(), what: "good to hear".into(), ..Default::default() }),
                                ]
                            }
                        ], ..Default::default() } ) ,
                        Statements::Exit,
                    ]
                }
            }, 
             ..Default::default()
        })
        ;
    assert_eq!(parse_yarn_nodes(choices), expected);
}

#[test]
fn test_branching_basic_eof_seperator(){

    let choices = "title: Test_node
    ---
    it was a beautiful day , said nobody
    Lamik: hi !
    Dona: good morning , how are you ?
    -> Lamik: are you asking me ?
       Dona: yes
    -> Lamik: fine !
       Dona: good to hear
    ===
    ";
    let mut expected : HashMap<String,YarnNode> = HashMap::new();
    expected.insert(
        "Test_node".into(), 
        YarnNode { 
            title: "Test_node".into(), 
             branch: {
                Branch {
                    statements: vec![
                        Statements::Dialogue(Dialogue { who: "nobody".into(), what: "it was a beautiful day , said nobody".into(), ..Default::default() }),

                        Statements::Dialogue(Dialogue { who: "Lamik".into(), what: "hi !".into(), ..Default::default() }),
                        Statements::Dialogue(Dialogue { who: "Dona".into(), what: "good morning , how are you ?".into(), ..Default::default() }),
                        Statements::Choice(Choice { branches: vec![
                            Branch {
                                statements: vec![
                                    Statements::Dialogue(Dialogue { who: "Lamik".into(), what: "are you asking me ?".into(), ..Default::default() }),
                                    Statements::Dialogue(Dialogue { who: "Dona".into(), what: "yes".into(), ..Default::default() }),
                                ]
                            }, 
                            Branch {
                                statements: vec![
                                    Statements::Dialogue(Dialogue { who: "Lamik".into(), what: "fine !".into(), ..Default::default() }),
                                    Statements::Dialogue(Dialogue { who: "Dona".into(), what: "good to hear".into(), ..Default::default() }),
                                ]
                            }
                        ], ..Default::default() } ) ,
                        Statements::Exit,
                    ]
                }
            }, 
             ..Default::default()
        })
        ;
    assert_eq!(parse_yarn_nodes(choices), expected);
}


#[test]
fn test_branching_basic_eof_seperator_lines_at_root_and_commands(){

    let choices = "title: Test_node
    ---
    it was a beautiful day , said nobody
    Lamik: hi !
    Dona: good morning , how are you ?
    -> Lamik: are you asking me ?
       Dona: yes
       <<jump node_a>>
    -> Lamik: fine !
       Dona: good to hear
       <<jump node_b>>
    Lamik: some other stuff
    <<blowup universe now>>
    ===
    ";
    let mut expected : HashMap<String,YarnNode> = HashMap::new();
    expected.insert(
        "Test_node".into(), 
        YarnNode { 
            title: "Test_node".into(), 
             branch: {
                Branch {
                    statements: vec![
                        Statements::Dialogue(Dialogue { who: "nobody".into(), what: "it was a beautiful day , said nobody".into(), ..Default::default() }),

                        Statements::Dialogue(Dialogue { who: "Lamik".into(), what: "hi !".into(), ..Default::default() }),
                        Statements::Dialogue(Dialogue { who: "Dona".into(), what: "good morning , how are you ?".into(), ..Default::default() }),
                        Statements::Choice(Choice { branches: vec![
                            Branch {
                                statements: vec![
                                    Statements::Dialogue(Dialogue { who: "Lamik".into(), what: "are you asking me ?".into(), ..Default::default() }),
                                    Statements::Dialogue(Dialogue { who: "Dona".into(), what: "yes".into(), ..Default::default() }),
                                    Statements::Command(YarnCommand { name: "jump".into(), params: "node_a".to_string(), command_type: Commands::Jump, ..Default::default() }),
                                ]
                            }, 
                            Branch {
                                statements: vec![
                                    Statements::Dialogue(Dialogue { who: "Lamik".into(), what: "fine !".into(), ..Default::default() }),
                                    Statements::Dialogue(Dialogue { who: "Dona".into(), what: "good to hear".into(), ..Default::default() }),
                                    Statements::Command(YarnCommand { name: "jump".into(), params: "node_b".to_string(), command_type: Commands::Jump, ..Default::default() }),
                                ]
                            }
                        ], ..Default::default() } ) ,
                        Statements::Dialogue(Dialogue { who: "Lamik".into(), what: "some other stuff".into(), ..Default::default() }),
                        Statements::Command(YarnCommand { name: "blowup".into(), params: "universe,now".to_string(), command_type: Commands::Generic, ..Default::default() }),
                        Statements::Exit,
                    ]
                }
            }, 
             ..Default::default()
        })
        ;
    assert_eq!(parse_yarn_nodes(choices), expected);
}


#[test]
fn test_branching_nesting_eof_seperator(){

    let choices = "title: Test_node
    ---
it was a beautiful day , said nobody
-> A
    -> A1
    -> A2
-> B
    ===
    ";
    let mut expected : HashMap<String,YarnNode> = HashMap::new();
    expected.insert(
        "Test_node".into(), 
        YarnNode { 
            title: "Test_node".into(), 
             branch: {
                Branch {
                    statements: vec![
                        Statements::Dialogue(Dialogue { who: "nobody".into(), what: "it was a beautiful day , said nobody".into(), ..Default::default() }),
                        Statements::Choice(Choice { branches: vec![
                            Branch {
                                statements: vec![
                                    Statements::Dialogue(Dialogue { who: "nobody".into(), what: "A".into(), ..Default::default() }),
                                    Statements::Choice(Choice { branches: vec![
                                        Branch {
                                            statements: vec![
                                                Statements::Dialogue(Dialogue { who: "nobody".into(), what: "A1".into(), ..Default::default() }),
                                            ]
                                        },
                                        Branch {
                                            statements: vec![
                                                Statements::Dialogue(Dialogue { who: "nobody".into(), what: "A2".into(), ..Default::default() }),
                                            ]
                                        }
                                    ],
                                    ..Default::default()
                                    })
                                ]
                            }, 
                            Branch {
                                statements: vec![
                                    Statements::Dialogue(Dialogue { who: "nobody".into(), what: "B".into(), ..Default::default() }),
                                ]
                            }
                        ], ..Default::default() } ) ,
                        Statements::Exit,
                    ]
                }
            }, 
             ..Default::default()
        })
        ;
    assert_eq!(parse_yarn_nodes(choices), expected);
}
