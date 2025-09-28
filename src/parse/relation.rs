use crate::parse::Relation;
use std::collections::BTreeMap;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_until, take_while};
use nom::character::complete::{alpha1, alphanumeric1, char};
use nom::combinator::{opt, recognize};
use nom::multi::{many0, many0_count};
use nom::sequence::pair;
use nom::{IResult, Parser};

const RELATION: &str = "@relation";

pub fn next(input: &str) -> IResult<&str, &str> {
    take_until(RELATION)(input)
}

fn hspace(input: &str) -> IResult<&str, ()> {
    let (input, _) = opt(take_while(|c| c == ' ' || c == '\t')).parse(input)?;
    Ok((input, ()))
}

fn open(input: &str) -> IResult<&str, ()> {
    let (input, _) = (char('('), hspace).parse(input)?;
    Ok((input, ()))
}

fn close(input: &str) -> IResult<&str, ()> {
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

fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, alt((tag("_"), tag("-")))))),
    ))
    .parse(input)
}

fn attribute_key(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))
    .parse(input)
}

fn attribute_value(input: &str) -> IResult<&str, &str> {
    identifier.parse(input)
}

pub fn relation(input: &str) -> IResult<&str, Relation> {
    let (input, result) = (
        tag(RELATION),
        open,
        identifier,
        many0((comma, attribute_key, equals, attribute_value)),
        close,
    )
        .parse(input)?;
    let mut relation = Relation {
        identifier: result.2.to_string(),
        attributes: BTreeMap::new(),
    };
    for (_, key, _, value) in result.3 {
        relation
            .attributes
            .insert(key.to_string(), value.to_string());
    }
    Ok((input, relation))
}
