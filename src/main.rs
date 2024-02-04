use cchead::Counter;
use std::env;
use std::fs;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("cchead: invalid option\nusage: cchead [-n lines | -c lines] [file ...]");
        return ExitCode::from(65);
    }
    let counting_type = &args[1];
    let count = &args[2];
    let file_path = &args[3];
    if let Ok(content) = fs::read_to_string(file_path) {
        let counter = Counter::from(counting_type, count.parse::<usize>().unwrap(), content);
        match counter {
            Ok(counter) => {
                println!("{}", counter);
                return ExitCode::from(0);
            }
            Err(error) => {
                eprintln!("{}", error);
                return ExitCode::from(1);
            }
        }
    }
    ExitCode::from(1)
}
