use crate::prelude::*;
use crate::constructs::token::Token;

enum Parsing {
    Tokens,
    String,
    Number,
    SingleLineComment,
    MultilineComment,
    Identifier,
}

pub fn tokenize(mlg_str: &str) -> Result<Vec<Token>> {
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
                    tokens.push(Token::STRING(buf.clone()));
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
                    if buf.contains('.') {
                        tokens.push(Token::FLOAT(buf.parse().unwrap()));
                    } else {
                        tokens.push(Token::INT(buf.parse().unwrap()));
                    }
                    buf.clear();
                    currently_parsing = Parsing::Tokens;
                }
            }
            Parsing::Identifier => {
                if c.is_alphanumeric() || '_' == c {
                    buf.push(c);
                } else {
                    match buf.as_str() {
                        "let" => tokens.push(Token::LET),
                        "struct" => tokens.push(Token::STRUCT),
                        "impl" => tokens.push(Token::IMPL),
                        "return" => tokens.push(Token::RETURN),
                        "true" => tokens.push(Token::TRUE),
                        "false" => tokens.push(Token::FALSE),
                        "yield" => tokens.push(Token::YIELD),
                        _ => tokens.push(Token::IDENTIFIER(buf.clone()))
                    }
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
                '(' => tokens.push(Token::LEFT_PAREN),
                ')' => tokens.push(Token::RIGHT_PAREN),
                '[' => tokens.push(Token::LEFT_SQR_BRACE),
                ']' => tokens.push(Token::RIGHT_SQR_BRACE),
                ',' => tokens.push(Token::COMMA),
                '-' => tokens.push(Token::MINUS),
                '+' => tokens.push(Token::PLUS),
                ';' => tokens.push(Token::SEMICOLON),
                '$' => tokens.push(Token::DOLLAR),
                '@' => tokens.push(Token::AT),
                '#' => tokens.push(Token::HASH),
                '~' => tokens.push(Token::TILDE),
                '%' => tokens.push(Token::PERCENT),
                '!' => tokens.push(Token::EXCLAMATION),
                '>' => tokens.push(Token::GREATER),
                '<' => tokens.push(Token::LESS),
                '=' => match prev_two_chars {
                    (Some('='), _) => {
                        tokens.pop();
                        tokens.push(Token::EQUAL_EQUAL)
                    },
                    (Some('!'), _) => {
                        tokens.pop();
                        tokens.push(Token::EXCLAMATION_EQUAL)
                    },
                    (Some('<'), _) => {
                        tokens.pop();
                        tokens.push(Token::LESS_EQUAL)
                    },
                    (Some('>'), _) => {
                        tokens.pop();
                        tokens.push(Token::GREATER_EQUAL)
                    },
                    _ => tokens.push(Token::EQUAL)
                }
                '&' => match prev_two_chars {
                    (Some('&'), Some('&')) => {
                        tokens.pop();
                        tokens.push(Token::TRIPLE_AMP)
                    },
                    (Some('&'), _) => {
                        tokens.push(Token::DOUBLE_AMP)
                    }
                    _ => {}
                }
                '|' => match prev_two_chars {
                    (Some('|'), Some('|')) => {
                        tokens.pop();
                        tokens.push(Token::TRIPLE_BAR)
                    },
                    (Some('|'), _) => {
                        tokens.push(Token::DOUBLE_BAR)
                    }
                    _ => tokens.push(Token::BAR(indent - 1))
                }
                '.' => {
                    if let (Some('.'), _) = prev_two_chars {
                        tokens.pop();
                        tokens.push(Token::DOT_DOT);
                    } else {
                        tokens.push(Token::DOT);
                    }
                }
                ':' => {
                    if let (Some(':'), _) = prev_two_chars {
                        tokens.pop();
                        tokens.push(Token::COLON_COLON);
                    } else {
                        tokens.push(Token::COLON);
                    }
                }
                '*' => {
                    if let (Some('/'), _) = prev_two_chars {
                        currently_parsing = Parsing::MultilineComment;
                    } else {
                        tokens.push(Token::STAR);
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
                    currently_parsing = Parsing::Identifier;
                    buf.clear();
                    buf.push(c);
                }
                '/' => {
                    if let (Some('/'), _) = prev_two_chars {
                        currently_parsing = Parsing::SingleLineComment;
                    } else {
                        tokens.push(Token::SLASH);
                    }
                }
                ' ' => {
                    if let Some(Token::NEWLINE(_)) = tokens.last() {
                        tokens.pop();
                        tokens.push(Token::NEWLINE(indent - 1));
                    }
                }
                '\t' => {
                    return parse_err!("Tabs are currently not allowed. Use spaces instead.");
                }
                '\r' => {}
                '\n' => {
                    if let Some(Token::NEWLINE(_)) = tokens.last() {
                        tokens.pop();
                    }
                    tokens.push(Token::NEWLINE(0));
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
            if buf.contains('.') {
                tokens.push(Token::FLOAT(buf.parse().unwrap()));
            } else {
                tokens.push(Token::INT(buf.parse().unwrap()));
            }
        },
        Parsing::Identifier => {
            tokens.push(Token::IDENTIFIER(buf));
        },
        _ => {}
    }
    
    // Remove newline at the end of the file
    if let Some(Token::NEWLINE(_)) = tokens.last() {
        tokens.pop();
    }

    Ok(tokens)
}