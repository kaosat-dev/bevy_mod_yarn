https://stackoverflow.com/questions/36300373/capture-the-entire-contiguous-matched-input-with-nom
https://github.com/rust-bakery/nom/blob/main/doc/nom_recipes.md#rust-style-identifiers
https://github.com/rust-bakery/nom/issues/32
https://stevedonovan.github.io/rust-gentle-intro/nom-intro.html

very good one: https://codeandbitters.com/lets-build-a-parser/

https://github.com/rust-bakery/nom/blob/main/examples/s_expression.rs
https://github.com/rust-bakery/nom/issues/902
//https://docs.rs/nom/latest/nom/sequence/fn.delimited.html


- how the hell do we implement expression parsing & evaluation in rust in a compiled language ???
    https://github.com/jdm/yarn-spool/commit/4583c11aaf8f9b7286d53e315f6672c17ee0b9d6
    https://github.com/rekka/meval-rs


- error handling ?
https://www.christopherbiscardi.com/custom-error-types-with-nom-5-in-rust
- very usefull : alway look here first
https://github.com/rust-bakery/nom/blob/main/doc/choosing_a_combinator.md

https://docs.rs/nom/latest/nom/character/complete/index.html
// second parameter is the result, the first is the non consumed remainder


## Alternative parser

https://github.com/zesterer/chumsky


## YARN SPECS !!!
https://github.com/YarnSpinnerTool/YarnSpinner/blob/040a2436d98e5c0cc72e6a8bc04e6c3fa156399d/Documentation/Yarn-Spec.md


## notes
all '<< />>' contain expressions 
<<(EXPRESSION)>>
variables can be of types: bool | string | int | f64

[spaces][->][who_what][XOR][expression]         => options block
[spaces][identifier][:][text] OR [spaces][text] => who what
[spaces][<<][expression][>>]                    => expression block

## Notes

many0(xx) can be usefull to force a result to a vec


- simple lines (command, whowhat) end at the linefeed (\n)
- options need to eat up a lot more (for now we are going to assume an empty line after options is the delimiter)
    -> foo: bar
        a: bla
        c: bli
    -> foo: biz
        c: gnagna
    <EMPTY LINE>

## Parser rules

 ### Node header
    - title:
        [ ]                     title   [ ]                    :          <TITLE>          \n 
        whitespace  (optional)  marker  whitespace (optional)  seperator  the title itself linefeed

    - tags:
        [ ]                     tags    [ ]                    :          <TAGS>          \n 
        whitespace  (optional)  marker  whitespace (optional)  seperator  the tags        linefeed

        the tags list itself is build like this
        [ ]                     <TAG1> [ ]                                                  <TAG2> [ ]
        whitespace  (optional)  tag1  whitespace: seperator (any amount of whitespace)    tag2

 ### Node body

  - 



## Alternate stuff

- https://github.com/tinaun/yarnspinner-rs uses https://docs.rs/logos/latest/logos/
- xxx uses pest