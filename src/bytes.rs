use std::fmt;

pub struct Bytes {
    content: String,
    count: usize,
}

impl Bytes {
    pub fn from(content: String, count: usize) -> Self {
        Bytes { content, count }
    }
}

impl fmt::Display for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = &self.content[..=self.count].to_string().clone();
        write!(f, "{}", result)
    }
}
