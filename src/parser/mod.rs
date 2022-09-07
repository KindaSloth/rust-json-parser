pub mod array;
pub mod bool;
pub mod json;
pub mod null;
pub mod number;
pub mod object;
pub mod string;

use combine::parser::char::spaces;
use combine::{choice, ParseError, Parser, Stream};

use self::array::array_parser;
use self::bool::bool_parser;
use self::json::*;
use self::null::null_parser;
use self::number::number_parser;
use self::object::object_parser;
use self::string::string_parser;

pub fn parser<Input>() -> impl Parser<Input, Output = Json>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    spaces().with(choice((
        array_parser(),
        null_parser(),
        bool_parser(),
        number_parser(),
        string_parser(),
        object_parser(),
    )))
}
