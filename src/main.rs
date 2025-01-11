mod text_tables;
use std::env;
//use text_tables::text_tables::{expand_math_shortcut, name_to_symbol};
#[derive(Debug, Clone)]
pub enum GrammarItem {
    Paren,
    Parentheses(u8),
    Operation(u16),
    Number(u64),
    Literal(String)
    Character(char)
    Function(String)
}

#[derive(Debug, Clone)]
pub struct ParseNode {
    pub children: Vec<ParseNode>,
    pub entry: GrammarItem,
}
impl ParseNode {
    pub fn new() -> ParseNode {
        ParseNode {
            children: Vec::new(),
            entry: GrammarItem::Paren,
        }
    }
}

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
