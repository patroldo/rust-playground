use std::{collections::HashMap, path::PathBuf};

use pgrep::{
    find_string_in_file, find_string_in_multiple_files_async_native_threads_with_thread_scope,
};

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    search_string: String,

    #[clap(value_delimiter = ' ', num_args = 1..)]
    path: Vec<PathBuf>,

    #[arg(short, long, default_value_t = false)]
    show_line_numbers: bool,
}

fn print_for_single_file(lines: Vec<String>) {
    for line in lines {
        println!("{}", line);
    }
}

fn print_for_multiple_files(
    map: HashMap<&PathBuf, Result<Vec<String>, std::io::Error>>,
    original_file_input: &[PathBuf],
) {
    let mut stdout: String = String::new();
    let mut stderr: String = String::new();
    for file in original_file_input {
        match map.get(file) {
            Some(file_search_result) => match file_search_result {
                Ok(lines) => {
                    for line in lines {
                        let str = format!("{}:{}\n", file.to_str().unwrap(), line);
                        stdout.push_str(&str);
                    }
                }
                Err(e) => {
                    let str = format!("{}:{}\n", file.to_str().unwrap(), e);
                    stderr.push_str(&str);
                }
            },
            None => unreachable!(),
        }
    }
    print!("{}", stdout);
    eprint!("{}", stderr);
}

fn main() -> Result<(), std::io::Error> {
    let args = Cli::parse();
    if args.path.len() == 1 {
        let file_path: &PathBuf = args.path.first().unwrap();
        let lines = find_string_in_file(&args.search_string, file_path)?;
        print_for_single_file(lines);
    } else {
        let lines = find_string_in_multiple_files_async_native_threads_with_thread_scope(
            &args.search_string,
            &args.path,
        );
        print_for_multiple_files(lines, &args.path);
    };
    Ok(())
}
