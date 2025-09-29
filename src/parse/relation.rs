use crate::parse::Relation;
use std::collections::BTreeMap;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_until, take_while};
use nom::character::complete::{alpha1, alphanumeric1, char, one_of};
use nom::combinator::{peek, recognize};
use nom::multi::{many0, many0_count};
use nom::sequence::pair;
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

fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, recognize(pair(tag("_"), alpha1)))),
        many0_count(alt((alphanumeric1, recognize(one_of("-_.:/|"))))),
    ))
    .parse(input)
}

fn attribute_key(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, recognize(pair(tag("_"), alpha1)))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))
    .parse(input)
}

fn attribute_value(input: &str) -> IResult<&str, &str> {
    identifier.parse(input)
}

pub fn relation(input: &str) -> IResult<&str, Relation> {
    let (input, (_, identifier, attributes, _)) = (
        opening,
        identifier,
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
