use crate::parse::Relation;

use std::collections::BTreeMap;

use nom::bytes::complete::{tag, take_until, take_while, take_while1};
use nom::character::complete::char;
use nom::combinator::peek;
use nom::multi::many0;
use nom::{AsChar, IResult, Parser};

const RELATION: &str = "@relation";

pub fn next(input: &str) -> IResult<&str, &str> {
    take_until(RELATION)(input)
}

pub fn is_opening(input: &str) -> bool {
    peek(opening).parse(input).is_ok()
}

fn opening(input: &str) -> IResult<&str, ()> {
    let (input, _) = (tag(RELATION), hspace, char('(')).parse(input)?;
    Ok((input, ()))
}

fn hspace(input: &str) -> IResult<&str, ()> {
    let (input, _) = take_while(AsChar::is_space).parse(input)?;
    Ok((input, ()))
}

fn closing(input: &str) -> IResult<&str, ()> {
    let (input, _) = (hspace, char(')')).parse(input)?;
    Ok((input, ()))
}

fn comma(input: &str) -> IResult<&str, ()> {
    let (input, _) = (hspace, char(','), hspace).parse(input)?;
    Ok((input, ()))
}

fn equals(input: &str) -> IResult<&str, ()> {
    let (input, _) = (hspace, char('='), hspace).parse(input)?;
    Ok((input, ()))
}

// In the future we may want to restrict the characters allowed in identifiers,
// attribute keys, and attribute values. But since we use JSON for interchange,
// for now we allow those tokens to be as general as possible and still
// allow really simple, unambiguous parsing. We will assume the upstream
// application that consumes our output will restrict the tokens further.

fn restricted_ascii(input: &str) -> IResult<&str, &str> {
    // Accept one or more ASCII "graphic" characters excluding: [,=()]
    // - `is_ascii_graphic()` matches '!'..='~' (printable, non-space)
    // - we then explicitly exclude additional disallowed characters
    take_while1(|c: char| c.is_ascii_graphic() && !matches!(c, ',' | '=' | '(' | ')'))(input)
}

fn identifier(input: &str) -> IResult<&str, &str> {
    restricted_ascii.parse(input)
}

fn attribute_key(input: &str) -> IResult<&str, &str> {
    identifier.parse(input)
}

fn attribute_value(input: &str) -> IResult<&str, &str> {
    identifier.parse(input)
}

pub fn relation(input: &str) -> IResult<&str, Relation> {
    let (input, (_, _, identifier, attributes, _)) = (
        opening,
        hspace, identifier,
        many0((comma, attribute_key, equals, attribute_value)),
        closing,
    )
        .parse(input)?;
    let mut relation = Relation {
        identifier: identifier.to_string(), // result.2.to_string(),
        attributes: BTreeMap::new(),
    };
    for (_, key, _, value) in attributes {
        relation
            .attributes
            .insert(key.to_string(), value.to_string());
    }
    Ok((input, relation))
}
