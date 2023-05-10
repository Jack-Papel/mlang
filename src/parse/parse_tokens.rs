use crate::constructs::token::symbol::{Symbol, builtin_symbols};
use crate::prelude::*;
use crate::constructs::token::{TokenKind, LiteralKind, Literal};

enum Parsing {
    Tokens,
    String,
    Number,
    SingleLineComment,
    MultilineComment,
    Symbol,
}

pub fn tokenize(mlg_str: &str) -> Result<Vec<TokenKind>> {
    let mut tokens = Vec::new();
    let mut chars = mlg_str.chars().peekable();

    let mut indent = 0;

    let mut prev_two_chars = (None, None);

    // buffer for building strings, numbers, etc.
    let mut buf: String = String::new();
    let mut currently_parsing = Parsing::Tokens;

    for c in chars.by_ref() {
        indent += 1;
        match currently_parsing {
            Parsing::String => {
                if '"' == c {
                    tokens.push(TokenKind::Literal(Literal {
                        kind: LiteralKind::String,
                        symbol: Symbol::from(buf.as_str())
                    }));
                    buf.clear();
                    // We need to do this manually because we're not using the loop
                    prev_two_chars = (Some('"'), prev_two_chars.0);
                    currently_parsing = Parsing::Tokens;
                    continue;
                } else {
                    buf.push(c);
                }
            }
            Parsing::Number => {
                if c.is_numeric() {
                    buf.push(c);
                } else {
                    tokens.push(TokenKind::Literal(Literal {
                        kind: if buf.contains('.') { LiteralKind::Float } else { LiteralKind::Int },
                        symbol: Symbol::from(buf.as_str())
                    }));
                    buf.clear();
                    currently_parsing = Parsing::Tokens;
                }
            }
            Parsing::Symbol => {
                if c.is_alphanumeric() || '_' == c {
                    buf.push(c);
                } else {
                    tokens.push(get_token_from_symbol_string(&buf));
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
                '(' => tokens.push(TokenKind::LEFT_PAREN),
                ')' => tokens.push(TokenKind::RIGHT_PAREN),
                '[' => tokens.push(TokenKind::LEFT_SQR_BRACE),
                ']' => tokens.push(TokenKind::RIGHT_SQR_BRACE),
                ',' => tokens.push(TokenKind::COMMA),
                '-' => tokens.push(TokenKind::MINUS),
                '+' => tokens.push(TokenKind::PLUS),
                ';' => tokens.push(TokenKind::SEMICOLON),
                '$' => tokens.push(TokenKind::DOLLAR),
                '@' => tokens.push(TokenKind::AT),
                '#' => tokens.push(TokenKind::HASH),
                '~' => tokens.push(TokenKind::TILDE),
                '%' => tokens.push(TokenKind::PERCENT),
                '!' => tokens.push(TokenKind::EXCLAMATION),
                '>' => tokens.push(TokenKind::GREATER),
                '<' => tokens.push(TokenKind::LESS),
                '=' => match prev_two_chars {
                    (Some('='), _) => {
                        tokens.pop();
                        tokens.push(TokenKind::EQUAL_EQUAL)
                    },
                    (Some('!'), _) => {
                        tokens.pop();
                        tokens.push(TokenKind::EXCLAMATION_EQUAL)
                    },
                    (Some('<'), _) => {
                        tokens.pop();
                        tokens.push(TokenKind::LESS_EQUAL)
                    },
                    (Some('>'), _) => {
                        tokens.pop();
                        tokens.push(TokenKind::GREATER_EQUAL)
                    },
                    _ => tokens.push(TokenKind::EQUAL)
                }
                '&' => match prev_two_chars {
                    (Some('&'), Some('&')) => {
                        tokens.pop();
                        tokens.push(TokenKind::TRIPLE_AMP)
                    },
                    (Some('&'), _) => {
                        tokens.push(TokenKind::DOUBLE_AMP)
                    }
                    _ => {}
                }
                '|' => match prev_two_chars {
                    (Some('|'), Some('|')) => {
                        tokens.pop();
                        tokens.push(TokenKind::TRIPLE_BAR)
                    },
                    (Some('|'), _) => {
                        tokens.push(TokenKind::DOUBLE_BAR)
                    }
                    _ => tokens.push(TokenKind::BAR(indent - 1))
                }
                '.' => {
                    if let (Some('.'), _) = prev_two_chars {
                        tokens.pop();
                        tokens.push(TokenKind::DOT_DOT);
                    } else {
                        tokens.push(TokenKind::DOT);
                    }
                }
                ':' => {
                    if let (Some(':'), _) = prev_two_chars {
                        tokens.pop();
                        tokens.push(TokenKind::COLON_COLON);
                    } else {
                        tokens.push(TokenKind::COLON);
                    }
                }
                '*' => {
                    if let (Some('/'), _) = prev_two_chars {
                        currently_parsing = Parsing::MultilineComment;
                    } else {
                        tokens.push(TokenKind::STAR);
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
                        tokens.push(TokenKind::SLASH);
                    }
                }
                ' ' => {
                    if let Some(TokenKind::NEWLINE(_)) = tokens.last() {
                        tokens.pop();
                        tokens.push(TokenKind::NEWLINE(indent - 1));
                    }
                }
                '\t' => {
                    return parse_err!("Tabs are currently not allowed. Use spaces instead.");
                }
                '\r' => {}
                '\n' => {
                    if let Some(TokenKind::NEWLINE(_)) = tokens.last() {
                        tokens.pop();
                    }
                    tokens.push(TokenKind::NEWLINE(0));
                    indent = 0;
                }
                _ => return parse_err!("Unexpected character: {}", c),
            }
        }
        
        // Update previous two characters
        prev_two_chars.1 = prev_two_chars.0;
        prev_two_chars.0 = Some(c);
    }

    match currently_parsing {
        Parsing::String => return parse_err!("Unterminated string: {}", buf),
        Parsing::Number => {
            tokens.push(TokenKind::Literal(Literal {
                kind: if buf.contains('.') { LiteralKind::Float } else { LiteralKind::Int },
                symbol: Symbol::from(buf.as_str())
            }));
        },
        Parsing::Symbol => {
            tokens.push(get_token_from_symbol_string(&buf));
        },
        _ => {}
    }
    
    // Remove newline at the end of the file
    if let Some(TokenKind::NEWLINE(_)) = tokens.last() {
        tokens.pop();
    }

    Ok(tokens)
}

fn get_token_from_symbol_string(buf: &String) -> TokenKind {
    match buf.as_str() {
        "let" => TokenKind::Keyword(*builtin_symbols::LET),
        "struct" => TokenKind::Keyword(*builtin_symbols::STRUCT),
        "impl" => TokenKind::Keyword(*builtin_symbols::IMPL),
        "return" => TokenKind::Keyword(*builtin_symbols::RETURN),
        "true" => TokenKind::Literal(Literal {kind: LiteralKind::Bool, symbol: *builtin_symbols::TRUE}),
        "false" => TokenKind::Literal(Literal {kind: LiteralKind::Bool, symbol: *builtin_symbols::FALSE}),
        "yield" => TokenKind::Keyword(*builtin_symbols::YIELD),
        _ => TokenKind::IDENTIFIER(Symbol::from(buf.as_str()))
    }
}