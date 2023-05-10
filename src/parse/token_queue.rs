use std::fmt::Debug;

use crate::constructs::token::TokenKind;

#[derive(Clone)]
pub struct TokenQueue<'a> {
    tokens: &'a Vec<TokenKind>,
    index: usize,
    end: usize,
}

impl<'a> Debug for TokenQueue<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("TokenQueue: ")?;
        if self.index >= self.end {
            f.write_str("[]")
        } else {
            self.tokens[self.index..self.end].fmt(f)
        }
    }
}

impl<'a> Iterator for TokenQueue<'a> {
    type Item = &'a TokenKind;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.end {
            return None;
        }
        let token = self.tokens.get(self.index);
        self.index += 1;
        token
    }
}

impl<'a> TokenQueue<'a> {
    pub fn new(tokens: &Vec<TokenKind>) -> TokenQueue {
        TokenQueue {
            tokens,
            index: 0,
            end: tokens.len(),
        }
    }

    pub fn peek(&self) -> Option<&TokenKind> {
        if self.index >= self.end {
            return None;
        }
        self.tokens.get(self.index)
    }

    pub fn peek_n(&self, n: isize) -> Option<&TokenKind> {
        self.tokens.get((self.index as isize + n) as usize)
    }

    pub fn skip(&mut self, n: usize) {
        self.index += n;
    }

    pub fn take(&mut self, n: usize) -> TokenQueue {
        let queue = TokenQueue {
            tokens: self.tokens,
            index: self.index,
            end: self.index + n,
        };
        
        self.index += n;

        queue
    }
}