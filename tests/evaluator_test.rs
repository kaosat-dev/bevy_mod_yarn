use bevy_mod_yarn::{ prelude::{Statements, Dialogue, parse_yarn_nodes_nom, Branch, Choice, YarnCommand, Commands, YarnAsset, DialogueTracker}};

#[test]
fn test_evaluate_minimal() {
    let micro = "title: Test_node
    ---
    Dona: what is wrong ?
    Grumpy: ...
    ===
    ";

    let parsed = parse_yarn_nodes_nom(micro);
    let yarn_asset = YarnAsset {
        raw: micro.into(),
        nodes: parsed
    };

    let mut dialogue_tracker = DialogueTracker::new(&yarn_asset, "Test_node".into());//{ current_node: "Test_node".into(), ..Default::default() };
    // dialogue_tracker.set_current_branch(&yarn_asset);

    let current_statement = dialogue_tracker.current_statement();
    let expected  = Statements::Dialogue(Dialogue { who: "Dona".into(), what: "what is wrong ?".into(), ..Default::default() });
    assert_eq!(current_statement, expected);

    // go to next entry
    dialogue_tracker.next_entry();
    let current_statement = dialogue_tracker.current_statement();
    let expected  = Statements::Dialogue(Dialogue { who: "Grumpy".into(), what: "...".into(), ..Default::default() });
    assert_eq!(current_statement, expected);

    dialogue_tracker.next_entry();
    let current_statement = dialogue_tracker.current_statement();
    let expected  = Statements::Exit;
    assert_eq!(current_statement, expected);
}



#[test]
fn test_evaluate_branching_basic(){

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

    let parsed = parse_yarn_nodes_nom(choices);
    let yarn_asset = YarnAsset {
        raw: choices.into(),
        nodes: parsed
    };

    let mut dialogue_tracker = DialogueTracker::new(&yarn_asset, "Test_node".into());//{ current_node: "Test_node".into(), ..Default::default() };
    // dialogue_tracker.set_current_branch(&yarn_asset);

    let current_statement = dialogue_tracker.current_statement();
    let expected  = Statements::Dialogue(Dialogue { who: "nobody".into(), what: "it was a beautiful day , said nobody".into(), ..Default::default() });
    assert_eq!(current_statement, expected);

    dialogue_tracker.next_entry();
    let current_statement = dialogue_tracker.current_statement();
    let expected  = Statements::Dialogue(Dialogue { who: "Lamik".into(), what: "hi !".into(), ..Default::default() });
    assert_eq!(current_statement, expected);

    dialogue_tracker.next_entry();
    let current_statement = dialogue_tracker.current_statement();
    let expected  = Statements::Dialogue(Dialogue { who: "Dona".into(), what: "good morning , how are you ?".into(), ..Default::default() });
    assert_eq!(current_statement, expected);

    dialogue_tracker.next_entry();
    let current_statement = dialogue_tracker.current_statement();
    let expected  =  Statements::Choice(Choice { branches: vec![
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
    ], ..Default::default() } );
    assert_eq!(current_statement, expected);

    // we check our choices helper
    let current_choices = dialogue_tracker.get_current_choices();
    let expected = vec![
        Dialogue { who: "Lamik".into(), what: "are you asking me ?".into(), ..Default::default() },
        Dialogue { who: "Lamik".into(), what: "fine !".into(), ..Default::default() },
    ];
    assert_eq!(current_choices, expected);

    // choose the other choice
    dialogue_tracker.next_choice();
    dialogue_tracker.next_entry(); // FIXME: still not sure about this way of validating choices
    let current_statement = dialogue_tracker.current_statement();
    let expected  = Statements::Dialogue(Dialogue { who: "Lamik".into(), what: "fine !".into(), ..Default::default() });
    assert_eq!(current_statement, expected);


    dialogue_tracker.next_entry();
    let current_statement = dialogue_tracker.current_statement();
    let expected  = Statements::Dialogue(Dialogue { who: "Dona".into(), what: "good to hear".into(), ..Default::default() });
    assert_eq!(current_statement, expected);

    dialogue_tracker.next_entry();
    let current_statement = dialogue_tracker.current_statement();
    let expected  = Statements::Exit;
    assert_eq!(current_statement, expected);
}

#[test]
fn test_evaluate_branching_nested_multinode(){

    let choices = "title: Test_node
    ---
    it was a beautiful day , said nobody
    Lamik: hi !
    Dona: good morning , how are you ?
    -> Lamik: are you asking me ?
        Dona: yes
    -> Lamik: fine !
        Dona: good to hear
    Dona: so... what have you been up to ?
    -> Lamik: hmmm...
    -> Lamik: i have started working on the most AMAZING project ever !
        Dona: ohh cool , tell me more !!
        -> Lamik: short version then
            Lamik:  a mechanical bunny of doom !
        -> Lamik: the long version ? here goes
            <<jump project_long>>
    -> Lamik: too early to tell
        Dona: oh ok, well, anyway, gotta go !
    Lamik: ok, bye !
    Dona: see you soon !
    ===
    title: project_long
    ---
    Lamik: so as I was saying, a mechanical bunny of doom ! 
    Lamik: ... with floppy ears of course
    ===
    ";

    let parsed = parse_yarn_nodes_nom(choices);
    let yarn_asset = YarnAsset {
        raw: choices.into(),
        nodes: parsed
    };

    let mut dialogue_tracker = DialogueTracker::new(&yarn_asset, "Test_node".into());//{ current_node: "Test_node".into(), ..Default::default() };
    // dialogue_tracker.set_current_branch(&yarn_asset);

    let current_statement = dialogue_tracker.current_statement();
    let expected  = Statements::Dialogue(Dialogue { who: "nobody".into(), what: "it was a beautiful day , said nobody".into(), ..Default::default() });
    assert_eq!(current_statement, expected);

    dialogue_tracker.next_entry();
    let current_statement = dialogue_tracker.current_statement();
    let expected  = Statements::Dialogue(Dialogue { who: "Lamik".into(), what: "hi !".into(), ..Default::default() });
    assert_eq!(current_statement, expected);

    dialogue_tracker.next_entry();
    let current_statement = dialogue_tracker.current_statement();
    let expected  = Statements::Dialogue(Dialogue { who: "Dona".into(), what: "good morning , how are you ?".into(), ..Default::default() });
    assert_eq!(current_statement, expected);

    dialogue_tracker.next_entry();
    let current_statement = dialogue_tracker.current_statement();
    let expected  =  Statements::Choice(Choice { branches: vec![
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
    ], ..Default::default() } );
    assert_eq!(current_statement, expected);

    // we check our choices helper
    let current_choices = dialogue_tracker.get_current_choices();
    let expected = vec![
        Dialogue { who: "Lamik".into(), what: "are you asking me ?".into(), ..Default::default() },
        Dialogue { who: "Lamik".into(), what: "fine !".into(), ..Default::default() },
    ];
    assert_eq!(current_choices, expected);

    // choose the other choice
    dialogue_tracker.next_choice();
    dialogue_tracker.next_entry(); // FIXME: still not sure about this way of validating choices
    let current_statement = dialogue_tracker.current_statement();
    let expected  = Statements::Dialogue(Dialogue { who: "Lamik".into(), what: "fine !".into(), ..Default::default() });
    assert_eq!(current_statement, expected);


    dialogue_tracker.next_entry();
    let current_statement = dialogue_tracker.current_statement();
    let expected  = Statements::Dialogue(Dialogue { who: "Dona".into(), what: "good to hear".into(), ..Default::default() });
    assert_eq!(current_statement, expected);

    dialogue_tracker.next_entry();
    let current_statement = dialogue_tracker.current_statement();
    let expected  = Statements::Dialogue(Dialogue { who: "Dona".into(), what: "so... what have you been up to ?".into(), ..Default::default() });
    assert_eq!(current_statement, expected);


    dialogue_tracker.next_entry();
    let current_statement = dialogue_tracker.current_statement();
    let expected  =  Statements::Choice(Choice { branches: vec![
        Branch {
            statements: vec![
                Statements::Dialogue(Dialogue { who: "Lamik".into(), what: "hmmm...".into(), ..Default::default() }),
            ]
        },
        Branch {
            statements: vec![
                Statements::Dialogue(Dialogue { who: "Lamik".into(), what: "i have started working on the most AMAZING project ever !".into(), ..Default::default() }),
                Statements::Dialogue(Dialogue { who: "Dona".into(), what: "ohh cool , tell me more !!".into(), ..Default::default() }),      
                Statements::Choice(Choice { branches: vec![
                    Branch {
                        statements: vec![
                            Statements::Dialogue(Dialogue { who: "Lamik".into(), what: "short version then".into(), ..Default::default() }),
                            Statements::Dialogue(Dialogue { who: "Lamik".into(), what: "a mechanical bunny of doom !".into(), ..Default::default() }),
                        ]
                    }, 
                    Branch {
                        statements: vec![
                            Statements::Dialogue(Dialogue { who: "Lamik".into(), what: "the long version ? here goes".into(), ..Default::default() }),
                            Statements::Command(YarnCommand { name: "jump".into(), params: "project_long".to_string(), command_type: Commands::Jump, ..Default::default() }),
                        ]
                    }
                ], ..Default::default() } )          
            ]
        }, 
        Branch {
            statements: vec![
                Statements::Dialogue(Dialogue { who: "Lamik".into(), what: "too early to tell".into(), ..Default::default() }),
                Statements::Dialogue(Dialogue { who: "Dona".into(), what: "oh ok, well, anyway, gotta go !".into(), ..Default::default() }),
            ]
        }
    ], ..Default::default() } );
    assert_eq!(current_statement, expected);

    // we check our choices helper
    let current_choices = dialogue_tracker.get_current_choices();
    let expected = vec![
         Dialogue { who: "Lamik".into(), what: "hmmm...".into(), ..Default::default() },
         Dialogue { who: "Lamik".into(), what: "i have started working on the most AMAZING project ever !".into(), ..Default::default() },
         Dialogue { who: "Lamik".into(), what: "too early to tell".into(), ..Default::default() },
    ];
    assert_eq!(current_choices, expected);

    // choose a specific choice (by index)
    dialogue_tracker.specific_choice(1);

    dialogue_tracker.next_entry();
    let current_statement = dialogue_tracker.current_statement();
    let expected  = Statements::Dialogue(Dialogue { who: "Lamik".into(), what: "i have started working on the most AMAZING project ever !".into(), ..Default::default() });
    assert_eq!(current_statement, expected);

    dialogue_tracker.next_entry();
    let current_statement = dialogue_tracker.current_statement();
    let expected  = Statements::Dialogue(Dialogue { who: "Dona".into(), what: "ohh cool , tell me more !!".into(), ..Default::default() });
    assert_eq!(current_statement, expected);


    dialogue_tracker.next_entry();
    let current_statement = dialogue_tracker.current_statement();
    let expected  =  Statements::Choice(Choice { branches: vec![
        Branch {
            statements: vec![
                Statements::Dialogue(Dialogue { who: "Lamik".into(), what: "short version then".into(), ..Default::default() }),
                Statements::Dialogue(Dialogue { who: "Lamik".into(), what: "a mechanical bunny of doom !".into(), ..Default::default() }),
            ]
        }, 
        Branch {
            statements: vec![
                Statements::Dialogue(Dialogue { who: "Lamik".into(), what: "the long version ? here goes".into(), ..Default::default() }),
                Statements::Command(YarnCommand { name: "jump".into(), params: "project_long".to_string(), command_type: Commands::Jump, ..Default::default() }),
            ]
        }
    ], ..Default::default() } );
    assert_eq!(current_statement, expected);

     // we check our choices helper
     let current_choices = dialogue_tracker.get_current_choices();
     let expected = vec![
          Dialogue { who: "Lamik".into(), what: "short version then".into(), ..Default::default() },
          Dialogue { who: "Lamik".into(), what: "the long version ? here goes".into(), ..Default::default() },
     ];
     assert_eq!(current_choices, expected);
    
    // choose a specific choice (by index)
    dialogue_tracker.specific_choice(1);

    // validate choice
    dialogue_tracker.next_entry();
    let current_statement = dialogue_tracker.current_statement();
    let expected  = Statements::Dialogue(Dialogue { who: "Lamik".into(), what: "the long version ? here goes".into(), ..Default::default() });
    assert_eq!(current_statement, expected);

    dialogue_tracker.next_entry();
    let current_statement = dialogue_tracker.current_statement();
    let expected  = Statements::Dialogue(Dialogue { who: "Lamik".into(), what: "so as I was saying, a mechanical bunny of doom ! ".into(), ..Default::default() });
    assert_eq!(current_statement, expected);

    dialogue_tracker.next_entry();
    let current_statement = dialogue_tracker.current_statement();
    let expected  = Statements::Dialogue(Dialogue { who: "Lamik".into(), what: "... with floppy ears of course".into(), ..Default::default() });
    assert_eq!(current_statement, expected);

    dialogue_tracker.next_entry();
    let current_statement = dialogue_tracker.current_statement();
    let expected  = Statements::Exit;
    assert_eq!(current_statement, expected);
}