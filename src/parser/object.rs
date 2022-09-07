use std::collections::HashMap;

use combine::parser::char::spaces;
use combine::{between, choice, many, satisfy, sep_by1, token, ParseError, Parser, Stream};

use super::array::array_parser;
use super::bool::bool_parser;
use super::json::*;
use super::null::null_parser;
use super::number::number_parser;
use super::string::string_parser;

pub fn object_parser<Input>() -> impl Parser<Input, Output = Json>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    spaces().with(
        between(
            spaces().with(token('{')).with(spaces()),
            spaces().with(token('}')).with(spaces()),
            spaces().with(sep_by1::<Vec<_>, _, _, _>(
                (
                    spaces().with(
                        between(
                            token('"'),
                            token('"'),
                            many::<Vec<_>, _, _>(satisfy(|c| c != '"' && c != ':')),
                        )
                        .map(|x| x.into_iter().collect::<String>()),
                    ),
                    spaces().with(token(':')).with(spaces()),
                    spaces().with(choice((
                        array_parser(),
                        null_parser(),
                        bool_parser(),
                        number_parser(),
                        string_parser(),
                    ))),
                )
                    .map(|(key, _, value)| (key, value)),
                spaces().with(token(',')).with(spaces()),
            )),
        )
        .map(|x| Json::JsonObject(x.into_iter().collect::<HashMap<String, Json>>())),
    )
}
