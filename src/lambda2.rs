use std::fmt::Display;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::sequence::{delimited, pair, separated_pair};
use nom::{IResult, InputIter};

#[derive(Debug, PartialEq)]
pub struct Var {
    name: String,
}

impl Var {
    pub fn new(name: String) -> Self {
        Var { name }
    }
}

impl Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, PartialEq)]
pub enum LambdaTerm {
    Variable {
        var: Var,
    },
    Abstraction {
        var: Var,
        body: Box<LambdaTerm>,
    },
    Appliction {
        func: Box<LambdaTerm>,
        arg: Box<LambdaTerm>,
    },
}

impl Display for LambdaTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LambdaTerm::Variable { var } => {
                write!(f, "{}", var)
            }
            LambdaTerm::Abstraction { var, body } => {
                write!(f, "(\\{}.{})", var, body)
            }
            LambdaTerm::Appliction { func, arg } => {
                write!(f, "({}${})", func, arg)
            }
        }
    }
}

pub fn parse(input: &str) -> Option<LambdaTerm> {
    let input: String = input
        .iter_elements()
        .filter(|c| !c.is_whitespace())
        .collect();
    let (_, x) = parse_lambda_term(input.as_str()).ok()?;
    Some(x)
}

fn parse_lambda_term(input: &str) -> IResult<&str, LambdaTerm> {
    alt((
        parse_lambda_term_application,
        parse_lambda_term_abstraction,
        parse_lambda_term_variable,
    ))(input)
}

fn parse_variable(input: &str) -> IResult<&str, Var> {
    let (input, name) =
        take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)?;
    Ok((input, Var::new(name.to_string())))
}

fn parse_lambda_term_variable(input: &str) -> IResult<&str, LambdaTerm> {
    let (input, var) = parse_variable(input)?;
    Ok((input, LambdaTerm::Variable { var }))
}

fn parse_lambda_term_abstraction(input: &str) -> IResult<&str, LambdaTerm> {
    // macro_rules! inner_parser {
    //     () => {
    //         pair(
    //             delimited(tag("\\"), parse_variable, tag(".")),
    //             parse_lambda_term,
    //         )
    //     };
    // }
    // let (input, (var, body)) = alt((
    //     inner_parser!(),
    //     delimited(tag("("), inner_parser!(), tag(")")),
    // ))(input)?;
    let (input, (var, body)) = delimited(
        tag("("),
        pair(
            delimited(tag("\\"), parse_variable, tag(".")),
            parse_lambda_term,
        ),
        tag(")"),
    )(input)?;
    Ok((
        input,
        LambdaTerm::Abstraction {
            var,
            body: Box::new(body),
        },
    ))
}

fn parse_lambda_term_application(input: &str) -> IResult<&str, LambdaTerm> {
    let (input, (func, arg)) = delimited(
        tag("("),
        separated_pair(parse_lambda_term, tag("$"), parse_lambda_term),
        tag(")"),
    )(input)?;
    // let (input, (func, arg)) =
    //     separated_pair(parse_lambda_term, tag("$"), parse_lambda_term)(input)?;
    Ok((
        input,
        LambdaTerm::Appliction {
            func: Box::new(func),
            arg: Box::new(arg),
        },
    ))
}
