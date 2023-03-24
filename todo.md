

- [x] basic nodes parsing (header + body)
- [ ] details
    - [x] dialogues: with or without character names
    - [ ] dialogues: interpolated values
    - [ ] dialogues: attributes
    - [ ] dialogues: Escaping Text

    - [x] choices: blank line to close a list of choices
    - [x] choices: nested/ indentation handling 
    - [x] commands: basic parsing

    - [ ] conditional expressions
    - [ ] inject tags into statements (might require bigger changes given the current approach)
    - [x] fix empties being pushed into statements list
    - [x] fix choice arrow still present in the first Statement in a choice
    - [x] tags parsing
    - [ ] expressions parsing
    - [ ] add testing
        - [x] yarn files with title & some basic dialogues
        - [x] yarn files with title & multiple nested levels of choices

    - [x] fix overly sensitive indentation issues
        - [x] we might need to track choice indentation seperatly from Dialogue/Command indentation
        ie this works :
            -> Lamik: everything!
            <<jump Foo>>
        but this does not
            -> Lamik: everything!
                <<jump Foo>> // notice the space at the start of the line: that changes the "current_indentation" in our tracking, messing things up


            
    - [ ] evaluator:
        - [ ] fix issue with reaching end of a branch and not jumping back to previous level/root level
        - [x] remove need to pass yarn_asset to every next_entry call
    - [ ] add support for multiple yarn files in a managed way : see https://docs.yarnspinner.dev/using-yarnspinner-with-unity/importing-yarn-files/yarn-projects