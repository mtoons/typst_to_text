mod parsing;
mod text_tables;
use parsing::GrammarItem;
use std::env;

fn main() {
    let mut args = env::args();
    let mut input: String = Default::default();
    args.next();
    for elem in args {
        input.push_str(&elem);
        input.push(' ');
    }
    let nodes = GrammarItem::parse_string(&input);

    println!("{}", nodes.render());
}
