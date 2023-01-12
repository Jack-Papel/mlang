use super::tokens::Token;

pub fn tokenize(mlg_str: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = mlg_str.chars().peekable();

    while let Some(c) = chars.next() {
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
            '!' => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::EXCLAMATION_EQUAL);
                } else {
                    tokens.push(Token::EXCLAMATION);
                }
            }
            '=' => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::EQUAL_EQUAL);
                } else {
                    tokens.push(Token::EQUAL);
                }
            }
            '>' => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::GREATER_EQUAL);
                } else {
                    tokens.push(Token::GREATER);
                }
            }
            '<' => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::LESS_EQUAL);
                } else {
                    tokens.push(Token::LESS);
                }
            }
            '&' => {
                if chars.peek() == Some(&'&') {
                    chars.next();
                    if chars.peek() == Some(&'&') {
                        chars.next();
                        tokens.push(Token::TRIPLE_AMP);
                    } else {
                        tokens.push(Token::DOUBLE_AMP);
                    }
                } else {
                    panic!("Unexpected character: {}", c);
                }
            }
            '|' => {
                if chars.peek() == Some(&'|') {
                    chars.next();
                    if chars.peek() == Some(&'|') {
                        chars.next();
                        tokens.push(Token::TRIPLE_BAR);
                    } else {
                        tokens.push(Token::DOUBLE_BAR);
                    }
                } else {
                    tokens.push(Token::BAR);
                }
            }
            '.' => {
                if chars.peek() == Some(&'.') {
                    chars.next();
                    tokens.push(Token::DOT_DOT);
                } else {
                    tokens.push(Token::DOT);
                }
            }
            ':' => {
                if chars.peek() == Some(&':') {
                    chars.next();
                    tokens.push(Token::COLON_COLON);
                } else {
                    tokens.push(Token::COLON);
                }
            }
            '*' => {
                if chars.peek() == Some(&'/') {
                    while let Some(c) = chars.next() {
                        if c == '*' && chars.peek() == Some(&'/') {
                            chars.next();
                            break;
                        }
                    }
                } else {
                    tokens.push(Token::STAR);
                }
            }
            '"' => {
                let mut string = String::new();
                while let Some(c) = chars.next() {
                    if c == '"' {
                        break;
                    }
                    string.push(c);
                }
                tokens.push(Token::STRING(string));
            }
            '0'..='9' => {
                let mut number = String::new();
                number.push(c);
                while let Some(c) = chars.peek() {
                    if c.is_digit(10) {
                        number.push(*c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if chars.peek() == Some(&'.') {
                    chars.next();
                    if chars.peek() == Some(&'.') {
                        chars.next();
                        tokens.push(Token::INT(number.parse().unwrap()));
                        tokens.push(Token::DOT_DOT);
                        continue;
                    }
                    number.push('.');
                    while let Some(c) = chars.peek() {
                        if c.is_digit(10) {
                            number.push(*c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::FLOAT(number.parse().unwrap()));
                } else {
                    tokens.push(Token::INT(number.parse().unwrap()));
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut identifier = String::new();
                identifier.push(c);
                while let Some(c) = chars.peek() {
                    if is_valid_identifier_character(*c) {
                        identifier.push(*c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                match identifier.as_str() {
                    "struct" => {
                        tokens.push(Token::STRUCT);
                    },
                    "impl" => {
                        tokens.push(Token::IMPL);
                    },
                    "false" => {
                        tokens.push(Token::FALSE);
                    }
                    "yield" => {
                        tokens.push(Token::YIELD);
                    }
                    "return" => {
                        tokens.push(Token::RETURN);
                    }
                    "true" => {
                        tokens.push(Token::TRUE);
                    }
                    "let" => {
                        tokens.push(Token::LET);
                    }
                    _ => {
                        tokens.push(Token::IDENTIFIER(identifier));
                    }
                }
            }
            '/' => {
                if chars.peek() == Some(&'/') {
                    while let Some(c) = chars.next() {
                        if c == '\n' {
                            break;
                        }
                    }
                } else if chars.peek() == Some(&'*') {
                    chars.next();
                    while let Some(c) = chars.next() {
                        if c == '*' && chars.peek() == Some(&'/') {
                            chars.next();
                            break;
                        }
                    }
                } else {
                    tokens.push(Token::SLASH);
                }
            }
            ' ' => {
                if chars.peek() == Some(&' ') {
                    chars.next();
                    if let Some(Token::NEWLINE(prev_indent)) = tokens.last() {
                        let indent = *prev_indent;
                        tokens.pop();
                        tokens.push(Token::NEWLINE(indent + 1));
                    }
                }
            }
            '\t' => {
                if let Some(Token::NEWLINE(prev_indent)) = tokens.last() {
                    let indent = *prev_indent;
                    tokens.pop();
                    tokens.push(Token::NEWLINE(indent + 1));
                }
            }
            '\r' => {}
            '\n' => {
                if let Some(Token::NEWLINE(_)) = tokens.last() {
                    tokens.pop();
                }
                tokens.push(Token::NEWLINE(0));
            }
            _ => panic!("Unexpected character: {}", c),
        }
    }
    
    // Remove newline at the end of the file
    if let Some(Token::NEWLINE(_)) = tokens.last() {
        tokens.pop();
    }

    tokens
}

fn is_valid_identifier_character(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}