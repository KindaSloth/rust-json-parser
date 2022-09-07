use combine::parser::char::spaces;
use combine::{between, choice, sep_by1, token, ParseError, Parser, Stream};

use super::bool::bool_parser;
use super::json::*;
use super::null::null_parser;
use super::number::number_parser;
use super::string::string_parser;

pub fn array_parser<Input>() -> impl Parser<Input, Output = Json>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    spaces().with(
        between(
            spaces().with(token('[')).with(spaces()),
            spaces().with(token(']')).with(spaces()),
            spaces().with(sep_by1::<Vec<_>, _, _, _>(
                choice((
                    null_parser(),
                    bool_parser(),
                    number_parser(),
                    string_parser(),
                )),
                spaces().with(token(',')).with(spaces()),
            )),
        )
        .map(|x| Json::JsonArray(x)),
    )
}
