use std::ops::Add;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    /// The beginning index of the span.
    pub index: u32,
    pub len: u16,
}

impl Add for Span {
    type Output = Span;

    fn add(self, rhs: Self) -> Self::Output {
        let (first, second) = if self.index < rhs.index {
            (self, rhs)
        } else {
            (rhs, self)
        };
        Span {
            index: first.index,
            len: (second.index - first.index + second.len as u32) as u16
        }
    }
}

impl Span {
    pub fn unit(idx: u32) -> Self {
        Span {
            index: idx,
            len: 1
        }
    }

    pub fn after(&self) -> Self {
        Span {
            index: (self.index + self.len as u32),
            len: 1
        }
    }
}