use crate::bytes::Bytes;
use crate::lines::Lines;
use crate::options::Options;
use std::fmt;

pub enum Counter {
    Bytes(Bytes),
    Lines(Lines),
}

impl Counter {
    pub fn from(content: String, option: Options) -> Result<Counter, String> {
        match option {
            Options::Bytes(count) => {
                let b = Bytes::from(content, count);
                Ok(Counter::Bytes(b))
            }
            Options::Lines(count) => {
                let l = Lines::from(content, count);
                Ok(Counter::Lines(l))
            }
        }
    }
}

impl fmt::Display for Counter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Counter::Bytes(b) => {
                write!(f, "{}", b)
            }
            Counter::Lines(l) => {
                write!(f, "{}", l)
            }
        }
    }
}
