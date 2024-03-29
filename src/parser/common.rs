use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{alpha1, alphanumeric1, space0},
    combinator::recognize,
    error::ParseError,
    multi::{many0_count, separated_list1},
    number::complete::recognize_float,
    sequence::{delimited, pair},
    IResult,
};

///Parsing identifiers that may start with a letter (or underscore) and may contain underscores, letters and numbers
pub fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))(input)
}

///Parsing identifiers that must start with a #  and may contain underscores, letters and numbers and :
pub fn tag_identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        tag("#"),
        many0_count(alt((alphanumeric1, tag("_"), tag(":")))),
    ))(input)
}

///Parsing variables that must start with a letter (or underscore) and may contain underscores, letters and numbers
pub fn variable_identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        tag("$"),
        many0_count(alt((alphanumeric1, tag("_"), tag(".")))),
    ))(input)
}

pub fn variable(input: &str) -> IResult<&str, &str> {
    let (input, variable) = identifier(input)?;
    Ok((input, variable))
}

pub fn operator(input: &str) -> IResult<&str, &str> {
    alt((
        tag("=="),
        tag("<"),
        tag(">"),
        tag("<="), // FIXME: does not work ?
        tag(">="), // FIXME: does not work ?
    ))(input)
}

pub fn expression(input: &str) -> IResult<&str, &str> {
    let (input, variable) = variable_identifier(input)?;
    // println!("expression {}", variable);
    Ok((input, variable))
}

pub fn parse_params(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, params) = separated_list1(
        tag(" "),
        alt((variable, recognize_float, expression, operator)), // each item in parameters can be either a variable, a number (float), an expression (FIXME: operator should not be here)
    )(input)?; // should be alt(variable, numeric)
    Ok((input, params))
}

pub fn any_non_whitespace(input: &str) -> IResult<&str, &str> {
    is_not(" ")(input)
    // not(space0)(input)
}

// TODO move to utils
/*
use nom::{
    IResult,
    error::ParseError,
    combinator::value,
    sequence::delimited,
    character::complete::multispace0,
  };*/

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn spacey<'a, F, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(space0, inner, space0)
}
