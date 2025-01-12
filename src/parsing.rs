use crate::text_tables::{subscript, superscript};
use std::{fmt, str::FromStr};

// maybe switch to range
pub enum GrammarItem {
    Content(Vec<GrammarItem>),
    Parentheses(Vec<GrammarItem>),
    Symbol(String),
    Number(f64),
    Literal(String),
    Character(char),
    Subscript(Vec<GrammarItem>),
    Superscript(Vec<GrammarItem>),
}

impl fmt::Display for GrammarItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text: String = "".to_string();
        match self {
            GrammarItem::Content(content) => {
                for e in content {
                    text.push_str(&e.to_string());
                }
            }
            GrammarItem::Parentheses(content) => {
                text.push('(');
                for e in content {
                    text.push_str(&e.to_string());
                }
                text.push(')');
            }
            GrammarItem::Symbol(symbol) => {
                text.push_str(symbol);
            }
            GrammarItem::Number(number) => {
                text.push_str(&number.to_string());
            }
            GrammarItem::Literal(literal) => {
                text.push_str(literal);
            }
            GrammarItem::Character(char) => {
                text.push(*char);
            }
            GrammarItem::Subscript(content) => {
                let mut content_string: String = "".to_string();
                for e in content {
                    content_string.push_str(&e.to_string());
                }
                text.push_str(&subscript(content_string));
            }
            GrammarItem::Superscript(content) => {
                let mut content_string: String = "".to_string();
                for e in content {
                    content_string.push_str(&e.to_string());
                }
                text.push_str(&superscript(content_string));
            }
        };
        write!(f, "{}", text)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseGrammarError;

impl FromStr for GrammarItem {
    type Err = ParseGrammarError;
    fn from_str(typst: &str) -> Result<GrammarItem, ParseGrammarError> {
        let mut i = 0;
        while i < typst.len() {
            i += 1;
        }
        todo!()
    }
}

#[test]
fn render_test() {
    let nodes: GrammarItem = GrammarItem::Content(vec![
        GrammarItem::Character('a'),
        GrammarItem::Subscript(vec![GrammarItem::Literal("text".to_string())]),
    ]);
    assert_eq!(nodes.to_string(), "aₜₑₓₜ".to_string())
}
