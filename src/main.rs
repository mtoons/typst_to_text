mod text_tables;
use std::env;
use text_tables::{subscript, superscript};
pub enum GrammarItem {
    Parentheses(Vec<GrammarItem>),
    Symbol(String),
    Number(f64),
    Literal(String),
    Character(char),
    Subscript(Vec<GrammarItem>),
    Superscript(Vec<GrammarItem>),
}

// maybe switch to range
fn render(typst: Vec<GrammarItem>) -> String {
    let mut text: String = "".to_string();
    for e in typst {
        match e {
            GrammarItem::Parentheses(content) => {
                text.push('(');
                text.push_str(&render(content));
                text.push(')');
            }
            GrammarItem::Symbol(symbol) => {
                text.push_str(&symbol);
            }
            GrammarItem::Number(number) => {
                text.push_str(&number.to_string());
            }
            GrammarItem::Literal(literal) => {
                text.push_str(&literal);
            }
            GrammarItem::Character(char) => {
                text.push(char);
            }
            GrammarItem::Subscript(content) => {
                text.push_str(&subscript(render(content)));
            }
            GrammarItem::Superscript(content) => {
                text.push_str(&superscript(render(content)));
            }
        };
    }
    text
}

pub fn parse(typst: String) -> Vec<GrammarItem> {
    todo!()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input: String = Default::default();
    for elem in args {
        input.push(' ');
        input.push_str(&elem);
    }
    let nodes = parse(input);
    let output = render(nodes);
    print!("{}", output);
}

#[test]
fn render_test() {
    let nodes: Vec<GrammarItem> = vec![
        GrammarItem::Character('a'),
        GrammarItem::Subscript(vec![GrammarItem::Literal("text".to_string())]),
    ];
    // print!("{}", render(nodes));
    assert_eq!(render(nodes), "aₜₑₓₜ")
}
