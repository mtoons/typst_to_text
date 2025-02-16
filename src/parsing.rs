use crate::text_tables::{expand_math_shortcut, name_to_symbol, subscript, superscript};

pub fn typst_to_text(typst: &str) -> String {
    GrammarItem::parse_string(typst).render()
}

// Lexical item can contain several copies of itself or a more concrete type
#[derive(Debug, PartialEq)]
pub enum GrammarItem {
    Content(Vec<GrammarItem>),
    Brackets(Brackets),
    Symbol(String),
    Literal(String),
    Unknown(char),
}

// Different types of brackets
#[derive(Debug, PartialEq)]
pub enum Brackets {
    Parentheses(Box<GrammarItem>),
    Squirly(Box<GrammarItem>),
    Square(Box<GrammarItem>),
    Subscript(Box<GrammarItem>),
    Superscript(Box<GrammarItem>),
}

impl Brackets {
    // render the brackets in suitable fashion
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
    // Render the final output
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
            GrammarItem::Literal(literal) => {
                text.push_str(literal);
            }
            GrammarItem::Unknown(c) => {
                text.push(*c);
            }
        };
        text
    }

    // Parse a string into a Vec of GrammarItems (wrapped inside a GrammarItem::Content type)
    pub fn parse_string(typst: &str) -> GrammarItem {
        let mut items: Vec<GrammarItem> = Vec::new();
        let mut chars = typst.char_indices().peekable();

        while let Some((i, char)) = chars.next() {
            items.push(match char {
                '(' | '{' | '[' => {
                    if let Some((end_index, _)) = find_matching_bracket(&typst[i..], char) {
                        let content = &typst[i + 1..i + end_index];
                        let bracket_type = match char {
                            '(' => Brackets::Parentheses,
                            '{' => Brackets::Squirly,
                            '[' => Brackets::Square,
                            _ => unreachable!(),
                        };
                        skip_to_index(&mut chars, i + end_index + 1);
                        if char == '[' {
                            GrammarItem::Literal(content.to_string())
                        } else {
                            GrammarItem::Brackets(bracket_type(Box::new(
                                GrammarItem::parse_string(content),
                            )))
                        }
                    } else {
                        GrammarItem::Unknown(char)
                    }
                }
                '_' | '^' => {
                    if let Some((_, '(')) | Some((_, '{')) | Some((_, '[')) | Some((_, '"')) =
                        chars.peek()
                    {
                        let mut next_char = chars.next().unwrap().1;
                        let mut offset = if chars.peek().unwrap_or(&(i + 2, ' ')).1 == '"' {
                            next_char = chars.next().unwrap().1;
                            1
                        } else {
                            0
                        };
                        if next_char == '"' {
                            offset = 1;
                        }
                        if let Some((end_index, _)) =
                            find_matching_bracket(&typst[i + offset * 2 + 1..], next_char)
                        {
                            let content = &typst[i + 2..i + end_index + offset * 2 + 1];
                            let script_type = if char == '_' {
                                Brackets::Subscript
                            } else {
                                Brackets::Superscript
                            };
                            skip_to_index(&mut chars, i + end_index + offset * 2 + 2);
                            if next_char == '"' || next_char == '[' {
                                GrammarItem::Brackets(script_type(Box::new(GrammarItem::Literal(
                                    content.to_string(),
                                ))))
                            } else {
                                GrammarItem::Brackets(script_type(Box::new(
                                    GrammarItem::parse_string(content),
                                )))
                            }
                        } else {
                            continue;
                        }
                    } else {
                        let bracket_type = if char == '_' {
                            Brackets::Subscript
                        } else {
                            Brackets::Superscript
                        };
                        match chars.next() {
                            Some((_, c)) => GrammarItem::Brackets(bracket_type(Box::new(
                                GrammarItem::Unknown(c),
                            ))),
                            None => GrammarItem::Unknown('_'),
                        }
                    }
                }
                '"' => {
                    if let Some((end_index, _)) = find_matching_bracket(&typst[i + 1..], '"') {
                        let literal_content = &typst[i + 1..end_index + i + 1];
                        skip_to_index(&mut chars, end_index + i + 2);
                        GrammarItem::Literal(literal_content.to_string())
                    } else {
                        GrammarItem::Unknown('"')
                    }
                }
                c if c.is_ascii_alphabetic() => {
                    let end_index = typst[i..]
                        .find(|c: char| !c.is_ascii_alphabetic() && c != '.')
                        .unwrap_or(typst.len() - i);
                    let symbol = &typst[i..i + end_index];
                    skip_to_index(&mut chars, i + end_index);
                    GrammarItem::Symbol(name_to_symbol(symbol.to_string()))
                }
                c if !c.is_whitespace() => {
                    let end_index = typst[i..]
                        .find(|c: char| c.is_whitespace())
                        .unwrap_or(typst.len() - i);
                    let symbol = expand_math_shortcut(&typst[i..i + end_index]);
                    if symbol.is_empty() {
                        GrammarItem::Unknown(char)
                    } else {
                        skip_to_index(&mut chars, i + end_index);
                        GrammarItem::Symbol(symbol.to_string())
                    }
                }
                _ => GrammarItem::Unknown(char),
            });
        }
        GrammarItem::Content(items)
    }
}
fn skip_to_index<I>(iter: &mut std::iter::Peekable<I>, index: usize)
where
    I: Iterator<Item = (usize, char)>,
{
    while let Some(&(current_idx, _)) = iter.peek() {
        if current_idx >= index {
            break;
        }
        iter.next();
    }
}
fn find_matching_bracket(input: &str, opening: char) -> Option<(usize, char)> {
    let closing = match opening {
        '(' => ')',
        '{' => '}',
        '[' => ']',
        '"' => '"',
        _ => return None,
    };

    let mut depth = 0;
    for (i, char) in input.char_indices() {
        match char {
            c if c == opening && opening != '"' => depth += 1,
            c if c == closing => {
                depth -= 1;
                if depth == 0 || opening == '"' {
                    return Some((i, c));
                }
            }
            _ => {}
        }
    }
    None
}

#[test]
fn single_char() {
    let alphabetic = GrammarItem::parse_string("a");
    assert_eq!(
        alphabetic,
        GrammarItem::Content(vec![GrammarItem::Symbol('a'.to_string())])
    );
    assert_eq!(alphabetic.render(), "a".to_string());

    let numeric = GrammarItem::parse_string("5");
    assert_eq!(
        numeric,
        GrammarItem::Content(vec![GrammarItem::Unknown('5')])
    );
    assert_eq!(numeric.render(), "5".to_string());

    let utf8 = GrammarItem::parse_string("β");
    assert_eq!(utf8, GrammarItem::Content(vec![GrammarItem::Unknown('β')]));
    assert_eq!(utf8.render(), "β".to_string());

    let other = GrammarItem::parse_string(";");
    assert_eq!(other, GrammarItem::Content(vec![GrammarItem::Unknown(';')]));
    assert_eq!(other.render(), ";".to_string());
}

#[test]
fn parentheses_test() {
    assert_eq!(typst_to_text("(a_e), {x^2}"), "(aₑ), {x²}".to_string());
}

#[test]
fn litterals_test() {
    assert_eq!(typst_to_text("\"a_e\""), "a_e".to_string());
    assert_eq!(typst_to_text("[a_e]"), "a_e".to_string());
}

#[test]
fn script_test() {
    assert_eq!(typst_to_text("a_e"), "aₑ".to_string());
    assert_eq!(typst_to_text("a^e"), "aᵉ".to_string());

    assert_eq!(typst_to_text("a_(text)"), "aₜₑₓₜ".to_string());
    assert_eq!(typst_to_text("a^(text)"), "aᵗᵉˣᵗ".to_string());

    // Chars for which a subscript/superscript version don't exist (like 'b'/'q') are handled
    assert_eq!(typst_to_text("n_(a + b)"), "n_(a + b)".to_string());
    assert_eq!(typst_to_text("n^(a + q)"), "n^(a + q)".to_string());

    // Parentheses only appear for multiple chars
    assert_eq!(typst_to_text("n_b"), "n_b".to_string());
    assert_eq!(typst_to_text("n^q"), "n^q".to_string());
    assert_eq!(typst_to_text("n_(b)"), "n_b".to_string());
    assert_eq!(typst_to_text("n^(q)"), "n^q".to_string());
}

#[test]
fn render_test() {
    let nodes: GrammarItem = GrammarItem::Content(vec![
        GrammarItem::Unknown('a'),
        GrammarItem::Brackets(Brackets::Subscript(Box::new(GrammarItem::Literal(
            "text".to_string(),
        )))),
        GrammarItem::Unknown(' '),
        GrammarItem::Brackets(Brackets::Squirly(Box::new(GrammarItem::Content(vec![
            GrammarItem::Unknown('a'),
            GrammarItem::Unknown(' '),
            GrammarItem::Unknown('+'),
            GrammarItem::Unknown(' '),
            GrammarItem::Unknown('b'),
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
