mod parsing;
mod text_tables;
use parsing::typst_to_text;
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

    println!("{}", typst_to_text(&input)); // Call the rendering function
}
