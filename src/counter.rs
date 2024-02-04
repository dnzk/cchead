use std::fmt;

pub struct Bytes {
    content: String,
    count: usize,
}

impl fmt::Display for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = &self.content[..=self.count].to_string().clone();
        write!(f, "{}", result)
    }
}

pub struct Lines {
    content: String,
    count: usize,
}

impl fmt::Display for Lines {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result: Vec<&str> = vec![];
        for l in self.content.lines() {
            if result.len() < self.count {
                result.push(l);
            } else {
                break;
            }
        }
        let result = result.join("\n");
        write!(f, "{}", result)
    }
}

pub enum Counter {
    Bytes(Bytes),
    Lines(Lines),
}

impl Counter {
    pub fn from(counter_type: &str, count: usize, content: String) -> Result<Counter, String> {
        match counter_type {
            "-c" => {
                let b = Bytes { count, content };
                Ok(Counter::Bytes(b))
            }
            "-n" => {
                let n = Lines { count, content };
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
