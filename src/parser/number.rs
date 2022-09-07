use combine::{
    many1,
    parser::char::{digit, spaces},
    ParseError, Parser, Stream,
};

use super::json::*;

pub fn number_parser<Input>() -> impl Parser<Input, Output = Json>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    spaces().with(
        many1::<Vec<_>, _, _>(digit())
            .map(|x| Json::JsonNumber(x.into_iter().collect::<String>().parse::<f32>().unwrap())),
    )
}
