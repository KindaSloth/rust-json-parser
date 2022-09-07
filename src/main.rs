use combine::Parser;
use rust_json_parser::parser::parser;

fn main() {
    let result =
        parser().parse(r#"{ "name": "minguinhas", "age": 10, "array": [null, true, 123] }"#);

    match result {
        Ok(x) => println!("{:?}", x.0),
        Err(err) => panic!("{}", err),
    }
}
