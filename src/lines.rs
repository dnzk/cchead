use std::fmt;

pub struct Lines {
    content: String,
    count: usize,
}

impl Lines {
    pub fn from(content: String, count: usize) -> Self {
        Lines { content, count }
    }
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
