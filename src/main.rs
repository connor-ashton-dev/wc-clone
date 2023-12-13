use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut flags: Vec<&String> = vec![];
    let mut file_name: Option<&String> = None;

    for arg in &args[1..] {
        if arg.starts_with('-') {
            flags.push(arg)
        } else if arg.contains('.') {
            file_name = Some(arg);
        }
    }

    let file_name = match file_name {
        Some(name) => name,
        None => {
            eprintln!("No file name provided");
            return;
        }
    };

    let file_string = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Failed to read file: {}", file_name);
            return;
        }
    };

    if flags.is_empty() {
        println!(
            "{} {} {} {}",
            get_lines(&file_string),
            get_words(&file_string),
            get_bytes(&file_string),
            file_name
        )
    } else {
        for flag in &flags {
            match flag.as_str() {
                "-c" => {
                    println!("{} {}", get_bytes(&file_string), file_name)
                }
                "-l" => {
                    println!("{} {}", get_lines(&file_string), file_name)
                }
                "-w" => {
                    println!("{} {}", get_words(&file_string), file_name)
                }
                "-m" => {
                    println!("{} {}", get_chars(&file_string), file_name)
                }
                _ => {
                    println!("Invalid flag")
                }
            }
        }
    }
}

fn get_bytes(s: &str) -> usize {
    s.as_bytes().len()
}

fn get_lines(s: &str) -> usize {
    s.lines().count()
}

fn get_words(s: &str) -> usize {
    let mut total_words = 0;
    for line in s.lines() {
        let words = line.split_whitespace().count();
        total_words += words
    }
    total_words
}

fn get_chars(s: &str) -> usize {
    s.chars().count()
}
