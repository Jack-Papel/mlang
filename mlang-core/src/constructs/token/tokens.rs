use std::fmt::Debug;

use crate::constructs::token::Token;

#[derive(Clone)]
pub struct Tokens<'a> {
    tokens: &'a Vec<Token>,
    index: usize,
    end: usize,
}

impl<'a> Debug for Tokens<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("TokenQueue: ")?;
        if self.index >= self.end {
            f.write_str("[]")
        } else {
            self.tokens[self.index..self.end].fmt(f)
        }
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = &'a Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.end {
            return None;
        }
        let token = self.tokens.get(self.index);
        self.index += 1;
        token
    }
}

impl<'a> Tokens<'a> {
    pub fn new(tokens: &Vec<Token>) -> Tokens {
        Tokens {
            tokens,
            index: 0,
            end: tokens.len(),
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        if self.index >= self.end {
            return None;
        }
        self.tokens.get(self.index)
    }

    pub fn peek_n(&self, n: isize) -> Option<&Token> {
        self.tokens.get((self.index as isize + n) as usize)
    }

    pub fn skip(&mut self, n: usize) {
        self.index += n;
    }

    pub fn take(&mut self, n: usize) -> Tokens {
        let queue = Tokens {
            tokens: self.tokens,
            index: self.index,
            end: self.index + n,
        };
        
        self.index += n;

        queue
    }
}