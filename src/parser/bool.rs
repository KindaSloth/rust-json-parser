use combine::{
    choice,
    parser::char::{spaces, string},
    ParseError, Parser, Stream,
};

use super::json::*;

pub fn bool_parser<Input>() -> impl Parser<Input, Output = Json>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    spaces().with(choice((
        spaces().with(string("true").map(|_| Json::JsonBool(true))),
        spaces().with(string("false").map(|_| Json::JsonBool(false))),
    )))
}
