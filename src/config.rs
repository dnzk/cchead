use crate::options::Options;
use std::env::Args;

pub struct Config {
    raw: Vec<String>,
}

impl Config {
    pub fn from(args: Args) -> Option<Self> {
        let args: Vec<String> = args.collect();
        if args.len() < 2 {
            return None;
        }
        Some(Config { raw: args })
    }

    fn find_by_pattern<T>(args: &[T], f: fn(&T) -> bool) -> Option<&T> {
        let mut r: Option<&T> = None;
        for a in args.iter() {
            if f(a) {
                r = Some(a);
            }
        }
        r
    }

    pub fn possible_file_path(&self) -> Option<String> {
        let path_list = self.raw[1..].to_vec().clone();
        if let Some(path) =
            Self::find_by_pattern(&path_list, |p| p.contains(std::path::MAIN_SEPARATOR))
        {
            return Some(path.clone());
        }
        None
    }

    fn count(&self) -> Option<usize> {
        let f = |n: &String| {
            let y = n.parse::<usize>();
            matches!(y, Ok(_))
        };
        if let Some(count) = Self::find_by_pattern(&self.raw, f) {
            return Some(count.parse::<usize>().unwrap());
        }
        None
    }

    pub fn option(&self) -> Option<Options> {
        if let Some(options) = Self::find_by_pattern(&self.raw, |o| o.starts_with('-')) {
            return Options::from(Some(options.clone()), self.count());
        }
        None
    }
}
