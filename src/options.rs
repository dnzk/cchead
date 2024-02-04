const C: &str = "c";
const N: &str = "n";

#[cfg_attr(test, derive(PartialEq, Debug))]
pub enum Options {
    Bytes(usize),
    Lines(usize),
}

impl Options {
    pub fn from(raw: Option<String>, count: Option<usize>) -> Option<Self> {
        if let Some(options) = raw {
            let options: Vec<&str> = options.trim().split("").collect();
            let mut result = None;
            for o in options.iter() {
                match o.to_lowercase().as_str() {
                    C => {
                        result = Some(Options::Bytes(count.unwrap()));
                        break;
                    }
                    N => {
                        result = Some(Options::Lines(count.unwrap()));
                        break;
                    }
                    _ => result = None,
                }
            }
            return result;
        }
        None
    }
}
