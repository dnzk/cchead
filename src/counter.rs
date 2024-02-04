pub trait Countable<T> {
    fn count(&self) -> T;
}

#[derive(Debug)]
pub struct Bytes {
    content: String,
    count: usize,
}

impl Countable<String> for Bytes {
    fn count(&self) -> String {
        let mut result: Vec<char> = vec![];
        let content_chars = self.content.chars();
        for c in content_chars {
            if result.len() <= self.count {
                result.push(c);
            }
        }
        let mut str_result = String::new();
        for r in result {
            str_result.push(r);
        }
        str_result
    }
}

#[derive(Debug)]
pub struct Lines {
    content: String,
    count: usize,
}

impl Countable<Vec<String>> for Lines {
    fn count(&self) -> Vec<String> {
        let mut lines: Vec<String> = vec![];
        let content: Vec<&str> = self.content.split('\n').collect();
        for l in content {
            if lines.len() < self.count {
                lines.push(l.to_string());
            } else {
                break;
            }
        }
        lines
    }
}

#[derive(Debug)]
pub enum Counter {
    Bytes(Bytes),
    Lines(Lines),
}

impl Counter {
    pub fn from(counter_type: &str, count: usize, content: String) -> Self {
        match counter_type {
            "-c" => {
                let b = Bytes { count, content };
                Counter::Bytes(b)
            }
            "-n" => {
                let n = Lines { count, content };
                Counter::Lines(n)
            }
            _ => panic!("Unrecognized option {}", counter_type),
        }
    }

    pub fn debug(&self) {
        if let Counter::Bytes(b) = self {
            dbg!(b.count());
        }
        if let Counter::Lines(l) = self {
            dbg!(l.count());
        }
    }
}
