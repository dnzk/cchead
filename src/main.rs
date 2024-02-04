use cchead::Counter;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Arguments error");
        return;
    }
    let counting_type = &args[1];
    let count = &args[2];
    let file_path = &args[3];
    if let Ok(content) = fs::read_to_string(file_path) {
        let counter = Counter::from(counting_type, count.parse::<usize>().unwrap(), content);
        counter.debug();
    }
}
