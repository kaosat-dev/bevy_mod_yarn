use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_until},
    character::complete::{alpha1, alphanumeric1, newline, not_line_ending},
    combinator::recognize,
    multi::{many0, many0_count, separated_list1},
    sequence::{pair, tuple},
    IResult,
};

use super::{body::till_end, identifier, spacey};

/// called header tags in the yarn spec , but that is confusing
#[derive(Debug, Clone)]
pub enum HeaderLine {
    Title(String),
    Tags(Vec<String>),
    Discard,
}

///Parsing identifiers that must start with a #  and may contain underscores, letters and numbers and :
pub fn header_tag_identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((
            alpha1,
            tag("_"),
            tag("#"), // for this one, the pound at the start is optional, unlike for the tag_identifier
        )),
        many0_count(alt((alphanumeric1, tag("_"), tag(":")))),
    ))(input)
}

/// from the yarn docs: Node titles must start with a letter, and can contain letters, numbers and underscores.
pub fn title(input: &str) -> IResult<&str, HeaderLine> {
    let (input, _) = tuple((spacey(tag_no_case("title")), tag(":")))(input)?;
    let (input, title) = spacey(identifier)(input)?;
    if input.len() > 0 {
        // Err("Invalid title")
    }

    Ok((input, HeaderLine::Title(title.into())))
}

pub fn header_tags(input: &str) -> IResult<&str, HeaderLine> {
    let (input, _) = tuple((spacey(tag_no_case("tags")), tag(":")))(input)?;
    // 0...n  tags
    let (input, tags) = many0(spacey(header_tag_identifier))(input)?;

    Ok((
        input,
        HeaderLine::Tags(tags.iter().map(|x| x.to_string()).collect()),
    ))
}

/// to discard header tags/lines we do not care about
pub fn discard(input: &str) -> IResult<&str, HeaderLine> {
    let (input, _) = till_end(input)?;
    Ok((input, HeaderLine::Discard))
}

pub fn header(input: &str) -> IResult<&str, (String, Vec<String>)> {
    let (input, header) = take_until("---")(input)?;
    let (input, _) = tag("---")(input)?;
    let (_, header) = separated_list1(newline, not_line_ending)(header)?;

    let mut _title: String = "".into();
    let mut _tags: Vec<String> = vec![];
    for line in header.iter() {
        let (_, header_line) = alt((title, header_tags, discard))(line)?;
        match header_line {
            HeaderLine::Title(title) => {
                _title = title;
            }
            HeaderLine::Tags(tags) => {
                _tags = tags;
            }
            _ => {}
        }
    }

    Ok((input, (_title, _tags)))
}
