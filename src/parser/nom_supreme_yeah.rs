use nom::{
    bytes::complete::{take_till, take_until, take_while_m_n, take_while}, 
    sequence::{tuple, pair},
    multi::{separated_list1, many0_count, many0, separated_list0, many1, many_till}, 
    IResult, Parser, combinator::{eof, recognize},
    branch::alt, character::{complete::alpha1, streaming::alphanumeric1},

};
use nom_supreme::{error::ErrorTree, final_parser::final_parser};
use nom_supreme::tag::streaming::tag;
use nom_supreme::ParserExt;
// use nom_supreme::tag::streaming::tag;

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

fn from_hex(
    input: &str,
) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(
    input: &str,
) -> IResult<&str, u8, ErrorTree<&str>> {
    take_while_m_n(2, 2, is_hex_digit)
        .context("Should be a 2 digit hex code")
        .map_res(from_hex)
        .parse(input)
}

fn hex_color(
    input: &str,
) -> IResult<&str, Color, ErrorTree<&str>> {
    tuple((hex_primary, hex_primary, hex_primary))
        .preceded_by(tag("#"))
        .parse(input)
        .map(|(input, (red, green, blue))| {
            (input, Color { red, green, blue })
        })
}



pub fn identifier2(input: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    recognize(
        pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_"))))
        )
    )(input)
}


pub fn foo_bar_baz(input: &str) -> IResult<&str, &str, ErrorTree<&str>> {

  
    //let bla = many_till( eof)
    tag("->").precedes(tag(" ")).precedes(identifier2).parse(input)
    // tag("foo").preceded_by(tag("#")).complete().parse(input)
    // tuple(())
}

pub fn hex_color_final(
    input: &str,
) -> Result<&str, ErrorTree<&str>> {
    let test_input = "
    (aa)(fine)
    (dsf)(sdf)(sdfezae)(azezae)
       (other)";
    let foo = "#foobar";
    let foo = "-> yes";

    final_parser(foo_bar_baz)(foo)
}