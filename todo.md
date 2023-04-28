
## Initial release

- [x] add plugin 
    possibly plugins ? (ie to split parsing from runner )
- [x] rename parse_yarn_nodes_nom to something more adapted & nicer
- [ ] change "Commands" to YarnCommands, as it otherwise clashes with bevy's Commands
- [x] for choices, perhaps return the current choice index together with the list of choices from get_current_choices
- error handling if start node name is not found
- [x] fix issue with nested choices in runner
- [ ] add examples
    - [x] two characters (player + 1)
        - very simple display/ui
    - [x] rpg style changing portraits
    - [ ] speech bubbles, multiple characters (3d)
    - [ ] examples should provide at least some command use

- [ ] add basic useage docs
- [ ] auto linefeed ? (ui specific)

## General

- [ ] does the api of the DialogueRunner make sense  ?
    - [ ] ironically as I never do oop , it seems to have some code smell
    - [ ] its api seems off for a component ? should it be an asset ? not quite as there can be a few different runners active ? and specific to entities ? hmm not quite as a runner actually runs dialogues for MULTIPLE characters
    - [ ] there are some parts of Bevy (namely audio), with ASSETS with api (audio.play)
    - [ ] should the api be part of the YarnAsset ??
        -> not really, take a look at bevy_kira_audio , the loaded assets are seperate
        https://github.com/NiklasEi/bevy_kira_audio/tree/main/src 
            -> loaders load an AudioSource , just a struct with TypeUuid like our yarn_asset https://github.com/NiklasEi/bevy_kira_audio/blob/main/src/source/mp3_loader.rs https://github.com/NiklasEi/bevy_kira_audio/blob/main/src/source/mod.rs#L20
            -> then there are audiochannels (a RESOURCE with an api)
            https://github.com/NiklasEi/bevy_kira_audio/blob/main/src/channel/typed.rs#L20
- [ ] default to first node in yarn file if no start node is specified ?

- [ ] accessibility features how to? screen readers etc

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
    - [x] add testing
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
        - [x] fix issue with reaching end of a branch and not jumping back to previous level/root level
        - [x] remove need to pass yarn_asset to every next_entry call
        - [x] add ability to specify a specific choice directly in addition of previous/next choices
        - [ ] rename next_entry() to next()
        - [ ] next_entry() should return an Option<Statement> perhaps ? (closer to an iterator, error handling etc)
    - [ ] add support for multiple yarn files in a managed way : see https://docs.yarnspinner.dev/using-yarnspinner-with-unity/importing-yarn-files/yarn-projects

- [x] bevy 0.10.1 support
- [ ] bevy 0.11 support on bevy_main branch (low priority, only if time allows)

- [ ] add testing
- [ ] add examples
- [ ] add docs