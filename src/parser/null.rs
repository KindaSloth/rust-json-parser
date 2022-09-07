use combine::{
    parser::char::{spaces, string},
    ParseError, Parser, Stream,
};

use super::json::*;

pub fn null_parser<Input>() -> impl Parser<Input, Output = Json>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    spaces().with(string("null").map(|_| Json::JsonNull))
}
