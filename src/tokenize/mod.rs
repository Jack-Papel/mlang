use crate::constructs::token::span::Span;
use crate::constructs::token::symbol::{Symbol, builtin_symbols};
use crate::prelude::*;
use crate::constructs::token::{TokenKind, LiteralKind, Literal, Token};

enum Parsing {
    Tokens,
    String,
    Number,
    SingleLineComment,
    MultilineComment,
    Symbol,
}

pub fn parse_tokens<'a>(mlg_str: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut chars = mlg_str.chars().peekable();

    let mut indent = 0;

    let mut prev_two_chars = (None, None);

    // buffer for building strings, numbers, etc.
    let mut buf: String = String::new();
    let mut currently_parsing = Parsing::Tokens;

    for (index, c) in chars.by_ref().enumerate() {
        let index = index as u32;
        macro_rules! push_token {
            ($kind:expr) => {
                tokens.push(Token ($kind, Span {index, len: 1}))
            };
            ($kind:expr, $len:expr) => {
                tokens.push(Token ($kind, Span {index: index + 1 - $len, len: $len}))
            };
        }
        indent += 1;
        match currently_parsing {
            Parsing::String => {
                if '"' == c {
                    tokens.push(Token(
                        TokenKind::Literal(Literal {
                            kind: LiteralKind::String,
                            symbol: Symbol::from(buf.as_str())
                        }), 
                        Span {
                            index: index - buf.len() as u32,
                            len: buf.len() as u16
                        }
                    ));
                    buf.clear();
                    // We need to do this manually because we're not using the loop
                    prev_two_chars = (Some('"'), prev_two_chars.0);
                    currently_parsing = Parsing::Tokens;
                    continue;
                } else {
                    match (buf.chars().last(), c) {
                        (Some('\\'), 'n') => {
                            buf.pop();
                            buf.push('\n')
                        },
                        _ => buf.push(c)
                    }
                }
            }
            Parsing::Number => {
                if c.is_numeric() {
                    buf.push(c);
                } else {
                    tokens.push(Token(
                        TokenKind::Literal(Literal {
                            kind: if buf.contains('.') { LiteralKind::Float } else { LiteralKind::Int },
                            symbol: Symbol::from(buf.as_str())
                        }),
                        Span {
                            index: index - buf.len() as u32,
                            len: buf.len() as u16
                        }
                    ));
                    buf.clear();
                    currently_parsing = Parsing::Tokens;
                }
            }
            Parsing::Symbol => {
                if c.is_alphanumeric() || '_' == c {
                    buf.push(c);
                } else {
                    push_token!(get_kind_from_symbol_string(&buf));
                    buf.clear();
                    currently_parsing = Parsing::Tokens;
                }
            }
            Parsing::SingleLineComment => {
                if '\n' == c {
                    currently_parsing = Parsing::Tokens;
                }
            }
            Parsing::MultilineComment => {
                if '/' == c {
                    if let (Some('*'), _) = prev_two_chars {
                        currently_parsing = Parsing::Tokens;
                    }
                }
            }
            _ => {}
        }
        if let Parsing::Tokens = currently_parsing {
            match c {
                '(' => push_token!(TokenKind::LeftParen),
                ')' => push_token!(TokenKind::RightParen),
                '[' => push_token!(TokenKind::LeftSqrBrace),
                ']' => push_token!(TokenKind::RightSqrBrace),
                ',' => push_token!(TokenKind::Comma),
                '-' => push_token!(TokenKind::Minus),
                '+' => push_token!(TokenKind::Plus),
                ';' => push_token!(TokenKind::Semicolon),
                '$' => push_token!(TokenKind::Dollar),
                '@' => push_token!(TokenKind::At),
                '#' => push_token!(TokenKind::Hash),
                '~' => push_token!(TokenKind::Tilde),
                '%' => push_token!(TokenKind::Percent),
                '!' => push_token!(TokenKind::Exclamation),
                '>' => push_token!(TokenKind::Greater),
                '<' => push_token!(TokenKind::Less),
                '=' => match prev_two_chars {
                    (Some('='), _) => {
                        tokens.pop();
                        push_token!(TokenKind::EqualEqual, 2)
                    },
                    (Some('!'), _) => {
                        tokens.pop();
                        push_token!(TokenKind::ExclamationEqual, 2)
                    },
                    (Some('<'), _) => {
                        tokens.pop();
                        push_token!(TokenKind::LessEqual, 2)
                    },
                    (Some('>'), _) => {
                        tokens.pop();
                        push_token!(TokenKind::GreaterEqual, 2)
                    },
                    _ => push_token!(TokenKind::Equal)
                }
                '&' => match prev_two_chars {
                    (Some('&'), Some('&')) => {
                        tokens.pop();
                        push_token!(TokenKind::TripleAmp, 3)
                    },
                    (Some('&'), _) => {
                        tokens.pop();
                        push_token!(TokenKind::DoubleAmp, 2)
                    }
                    _ => push_token!(TokenKind::Amp)
                }
                '|' => match prev_two_chars {
                    (Some('|'), Some('|')) => {
                        tokens.pop();
                        push_token!(TokenKind::TripleBar, 3)
                    },
                    (Some('|'), _) => {
                        push_token!(TokenKind::DoubleBar, 2)
                    }
                    _ => push_token!(TokenKind::Bar(indent - 1))
                }
                '.' => {
                    if let (Some('.'), _) = prev_two_chars {
                        tokens.pop();
                        push_token!(TokenKind::DotDot, 2)
                    } else {
                        push_token!(TokenKind::Dot)
                    }
                }
                ':' => {
                    if let (Some(':'), _) = prev_two_chars {
                        tokens.pop();
                        push_token!(TokenKind::ColonColon, 2)
                    } else {
                        push_token!(TokenKind::Colon)
                    }
                }
                '*' => {
                    if let (Some('/'), _) = prev_two_chars {
                        currently_parsing = Parsing::MultilineComment;
                    } else {
                        push_token!(TokenKind::Star)
                    }
                }
                '"' => {
                    currently_parsing = Parsing::String;
                    buf.clear();
                }
                '0'..='9' => {
                    currently_parsing = Parsing::Number;
                    buf.clear();
                    buf.push(c);
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    currently_parsing = Parsing::Symbol;
                    buf.clear();
                    buf.push(c);
                }
                '/' => {
                    if let (Some('/'), _) = prev_two_chars {
                        currently_parsing = Parsing::SingleLineComment;
                    } else {
                        push_token!(TokenKind::Slash)
                    }
                }
                ' ' => {
                    if let Some(Token(TokenKind::Newline(_), Span{index: newline_start, ..})) = tokens.last() {
                        let newline_start = newline_start.clone();
                        tokens.pop();
                        tokens.push(Token(
                            TokenKind::Newline(indent - 1), 
                            Span {
                                index: newline_start, 
                                len: (index - newline_start) as u16
                            }
                        ));
                    }
                }
                '\t' => {
                    return syntax_err!(Some(Span::unit(index)), "Tabs are currently not allowed. Use spaces instead.");
                }
                '\r' => {}
                '\n' => {
                    if let Some(Token(TokenKind::Newline(_), ..)) = tokens.last() {
                        tokens.pop();
                    }
                    push_token!(TokenKind::Newline(0), 0);
                    indent = 0;
                }
                _ => return syntax_err!(Some(Span::unit(index)), "Unexpected character: {}", c),
            }
        }
        
        // Update previous two characters
        prev_two_chars.1 = prev_two_chars.0;
        prev_two_chars.0 = Some(c);
    }

    match currently_parsing {
        Parsing::String => return syntax_err!(Some(Span { 
            index: (mlg_str.len() - buf.len()) as u32,
            len: buf.len() as u16
        }), "Unterminated string: {}", buf),
        Parsing::Number => {
            tokens.push(Token(
                TokenKind::Literal(Literal {
                    kind: if buf.contains('.') { LiteralKind::Float } else { LiteralKind::Int },
                    symbol: Symbol::from(buf.as_str())
                }),
                Span {
                    index: (mlg_str.len() - buf.len()) as u32,
                    len: buf.len() as u16
                }
            ));
        },
        Parsing::Symbol => {
            tokens.push(Token(
                get_kind_from_symbol_string(&buf),
                Span {
                    index: (mlg_str.len() - buf.len()) as u32,
                    len: buf.len() as u16
                }
            ));
        },
        _ => {}
    }
    
    // Remove newline at the end of the file
    if let Some(Token(TokenKind::Newline(_), ..)) = tokens.last() {
        tokens.pop();
    }

    if let Some(token) = tokens.iter().find(|tok| tok.0 == TokenKind::Amp) {
        return syntax_err!(Some(token.1), "Ampersands are not currently supported!");
    }

    Ok(tokens)
}

fn get_kind_from_symbol_string(buf: &String) -> TokenKind {
    match buf.as_str() {
        "let" => TokenKind::Keyword(*builtin_symbols::LET),
        "struct" => TokenKind::Keyword(*builtin_symbols::STRUCT),
        "impl" => TokenKind::Keyword(*builtin_symbols::IMPL),
        "return" => TokenKind::Keyword(*builtin_symbols::RETURN),
        "true" => TokenKind::Literal(Literal {kind: LiteralKind::Bool, symbol: *builtin_symbols::TRUE}),
        "false" => TokenKind::Literal(Literal {kind: LiteralKind::Bool, symbol: *builtin_symbols::FALSE}),
        "yield" => TokenKind::Keyword(*builtin_symbols::YIELD),
        _ => TokenKind::Identifier(Symbol::from(buf.as_str()))
    }
}