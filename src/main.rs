mod parsing;
mod text_tables;
use parsing::GrammarItem;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input: String = Default::default();
    for elem in args {
        input.push(' ');
        input.push_str(&elem);
    }
    let nodes = GrammarItem::parse_string(input);

    println!("{}", nodes.render());
}
