use cchead::{Config, Counter, Source};
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let config = match Config::from(env::args()) {
        Some(config) => config,
        _ => {
            print_error("invalid option");
            return ExitCode::from(65);
        }
    };
    let source = Source::from(config.possible_file_path());
    let content = match source.get_content() {
        Ok(content) => content,
        Err(_error) => {
            print_error("invalid file path");
            return ExitCode::from(1);
        }
    };
    let counter = match config.option() {
        Some(option) => Counter::from(content, option),
        None => {
            print_error("invalid option");
            return ExitCode::from(1);
        }
    };
    if let Ok(counter) = counter {
        println!("{}", counter);
        return ExitCode::from(0);
    }
    ExitCode::from(1)
}

fn print_error(error: &str) {
    let full_error = format!(
        "cchead: {}\nusage: cchead [-n lines | -c bytes] [file ...]",
        error
    );
    eprintln!("{}", full_error);
}
