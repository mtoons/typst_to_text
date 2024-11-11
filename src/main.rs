mod text_tables;
use std::env;
//use text_tables::text_tables::{expand_math_shortcut, name_to_symbol};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input: String = Default::default();
    for elem in args {
        input.push(' ');
        input.push_str(&elem);
    }
    let output: String = input;
    print!("{}", output);
}
