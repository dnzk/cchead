use crate::bytes::Bytes;
use crate::lines::Lines;
use std::fmt;

pub enum Counter {
    Bytes(Bytes),
    Lines(Lines),
}

impl Counter {
    pub fn from(counter_type: &str, count: usize, content: String) -> Result<Counter, String> {
        match counter_type {
            "-c" => {
                let b = Bytes::from(content, count);
                Ok(Counter::Bytes(b))
            }
            "-n" => {
                let n = Lines::from(content, count);
                Ok(Counter::Lines(n))
            }
            _ => Err(format!(
                "cchead: invalid option {}\nusage: cchead [-n lines | -c bytes] [file ...]",
                counter_type
            )),
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
