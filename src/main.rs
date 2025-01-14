mod parsing;
mod text_tables;
use parsing::GrammarItem;
use std::env;

fn main() {
    let mut args = env::args();
    let mut input: String = Default::default();
    args.next(); // skip the file path

    // Put all argument as a string so the users don't have to do it
    for elem in args {
        input.push_str(&elem);
        input.push(' ');
    }

    let nodes = GrammarItem::parse_string(&input); // Call the parsing function
    println!("{}", nodes.render()); // Call the rendering function
}
