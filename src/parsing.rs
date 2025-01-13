use crate::text_tables::{name_to_symbol, subscript, superscript};

// maybe switch to range
// else think of replacing Vec with Box
pub enum GrammarItem {
    Content(Vec<GrammarItem>),
    Brackets(Brackets),
    Symbol(String),
    // Number(f64),
    Literal(String),
    // Nothing,
}
pub enum Brackets {
    Parentheses(Box<GrammarItem>),
    Squirly(Box<GrammarItem>),
    Square(Box<GrammarItem>),
    Subscript(Box<GrammarItem>),
    Superscript(Box<GrammarItem>),
}

impl Brackets {
    fn render(&self) -> String {
        let mut text: String = "".to_string();
        match self {
            Brackets::Parentheses(content) => {
                text.push('(');
                text.push_str(&content.render());
                text.push(')');
            }
            Brackets::Squirly(content) => {
                text.push('{');
                text.push_str(&content.render());
                text.push('}');
            }
            Brackets::Square(content) => {
                text.push('[');
                text.push_str(&content.render());
                text.push(']');
            }
            Brackets::Subscript(content) => {
                text.push_str(&subscript(content.render()));
            }
            Brackets::Superscript(content) => {
                text.push_str(&superscript(content.render()));
            }
        }
        text
    }
}

impl GrammarItem {
    pub fn render(&self) -> String {
        let mut text: String = "".to_string();
        match self {
            GrammarItem::Content(content) => {
                for e in content {
                    text.push_str(&e.render());
                }
            }
            GrammarItem::Brackets(content) => {
                text.push_str(&content.render());
            }
            GrammarItem::Symbol(symbol) => {
                text.push_str(symbol);
            }
            // GrammarItem::Number(number) => {
            //     text.push_str(&number.to_string());
            // }
            GrammarItem::Literal(literal) => {
                text.push_str(literal);
            } // GrammarItem::Nothing => {}
        };
        text
    }
    pub fn parse_string(typst: String) -> GrammarItem {
        let mut item: Vec<GrammarItem> = Vec::new();
        let mut i = 0;
        let mut iter = typst.char_indices();
        while i < typst.len() {
            i += 1;
        }
        while let Some(elem) = iter.next() {
            let char = elem.1;
            let i = elem.0;
            item.push(if char == '(' {
                match typst[i..].find(')') {
                    Some(end_index) => {
                        iter.nth(end_index - 1);
                        GrammarItem::Brackets(Brackets::Parentheses(Box::new(
                            GrammarItem::parse_string(typst[i + 1..end_index + i].to_string()),
                        )))
                    }
                    None => GrammarItem::Symbol('('.to_string()),
                }
            } else if char == '{' {
                match typst[i..].find('}') {
                    Some(end_index) => {
                        iter.nth(end_index - 1);
                        GrammarItem::Brackets(Brackets::Squirly(Box::new(
                            GrammarItem::parse_string(typst[i + 1..end_index + i].to_string()),
                        )))
                    }
                    None => GrammarItem::Symbol('{'.to_string()),
                }
            } else if char == '[' {
                match typst[i..].find(']') {
                    Some(end_index) => {
                        iter.nth(end_index - 1);
                        GrammarItem::Brackets(Brackets::Square(Box::new(
                            GrammarItem::parse_string(typst[i + 1..end_index + i].to_string()),
                        )))
                    }
                    None => GrammarItem::Symbol('['.to_string()),
                }
            } else if char == '_' {
                let next_char = iter.next();
                if next_char == Some((i + 1, '(')) {
                    match typst[i..].find(')') {
                        Some(end_index) => {
                            iter.nth(end_index - 2);
                            GrammarItem::Brackets(Brackets::Subscript(Box::new(
                                GrammarItem::parse_string(typst[i + 2..end_index + i].to_string()),
                            )))
                        }
                        None => GrammarItem::Brackets(Brackets::Subscript(Box::new(
                            GrammarItem::Symbol('('.to_string()),
                        ))),
                    }
                } else if next_char.is_none() {
                    GrammarItem::Symbol('_'.to_string())
                } else {
                    GrammarItem::Brackets(Brackets::Subscript(Box::new(GrammarItem::Symbol(
                        typst[i + 2..i + 3].to_string(),
                    ))))
                }
            } else if char == '^' {
                let next_char = iter.next();
                if next_char == Some((i + 1, '(')) {
                    match typst[i..].find(')') {
                        Some(end_index) => {
                            iter.nth(end_index - 2);
                            GrammarItem::Brackets(Brackets::Superscript(Box::new(
                                GrammarItem::parse_string(typst[i + 2..end_index + i].to_string()),
                            )))
                        }
                        None => GrammarItem::Brackets(Brackets::Superscript(Box::new(
                            GrammarItem::Symbol('('.to_string()),
                        ))),
                    }
                } else if next_char.is_none() {
                    GrammarItem::Symbol('^'.to_string())
                } else {
                    GrammarItem::Brackets(Brackets::Superscript(Box::new(GrammarItem::Symbol(
                        typst[i + 2..i + 3].to_string(),
                    ))))
                }
            } else if char.is_ascii_alphabetic() || char == '_' {
                let next_char = typst.chars().nth(i + 1).unwrap_or(' ');
                if next_char.is_ascii_alphabetic() || next_char == '_' {
                    let end_index = typst[i..].find(|c: char| !c.is_ascii_alphabetic() && c != '_');
                    match end_index {
                        Some(end_index) => {
                            iter.nth(end_index);
                            GrammarItem::Symbol(name_to_symbol(typst[i..i + end_index].to_string()))
                        }
                        None => {
                            item.push(GrammarItem::Symbol(name_to_symbol(typst[i..].to_string())));
                            return GrammarItem::Content(item);
                        }
                    }
                    // GrammarItem::Symbol(())
                } else {
                    GrammarItem::Symbol(char.to_string())
                }
            } else if char == '"' {
                match typst[i..].find('"') {
                    Some(end_index) => {
                        iter.nth(end_index - 1);
                        GrammarItem::Literal(typst[i + 1..end_index + i].to_string())
                    }
                    None => GrammarItem::Symbol('"'.to_string()),
                }
            } else {
                GrammarItem::Symbol(char.to_string())
            })
        }
        GrammarItem::Content(item)
    }
}

#[test]
fn render_test() {
    let nodes: GrammarItem = GrammarItem::Content(vec![
        GrammarItem::Symbol('a'.to_string()),
        GrammarItem::Brackets(Brackets::Subscript(Box::new(GrammarItem::Literal(
            "text".to_string(),
        )))),
        GrammarItem::Symbol(' '.to_string()),
        GrammarItem::Brackets(Brackets::Squirly(Box::new(GrammarItem::Content(vec![
            GrammarItem::Symbol('a'.to_string()),
            GrammarItem::Symbol(' '.to_string()),
            GrammarItem::Symbol('+'.to_string()),
            GrammarItem::Symbol(' '.to_string()),
            GrammarItem::Symbol('b'.to_string()),
        ])))),
        GrammarItem::Brackets(Brackets::Superscript(Box::new(GrammarItem::Content(vec![
            GrammarItem::Symbol("36.6".to_string()),
            GrammarItem::Brackets(Brackets::Superscript(Box::new(GrammarItem::Symbol(
                '2'.to_string(),
            )))),
        ])))),
    ]);
    assert_eq!(nodes.render(), "aₜₑₓₜ {a + b}^(36.6²)".to_string())
}
