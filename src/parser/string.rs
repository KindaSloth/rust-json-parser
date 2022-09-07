use combine::{between, many, parser::char::spaces, satisfy, token, ParseError, Parser, Stream};

use super::json::*;

pub fn string_parser<Input>() -> impl Parser<Input, Output = Json>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    spaces().with(
        between(
            token('"'),
            token('"'),
            many::<Vec<_>, _, _>(satisfy(|c| c != '"')),
        )
        .map(|x| Json::JsonString(x.into_iter().collect::<String>())),
    )
}
