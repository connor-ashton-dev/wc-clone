use std::{env, fs};

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let mut flags: Vec<&String> = vec![];
    let mut file_name: Option<&String> = None;

    for arg in args.iter().skip(1) {
        if arg.starts_with('-') {
            flags.push(arg)
        } else if arg.contains('.') {
            file_name = Some(arg);
        }
    }

    let file_name = file_name.ok_or_else(|| {
        eprintln!("No file name provided");
        "No file name provided".to_string()
    })?;

    let file_string = fs::read_to_string(file_name).map_err(|e| {
        eprintln!("Failed to read file: {} because {}", file_name, e);
        "Failed to read file".to_string()
    })?;

    if flags.is_empty() {
        println!(
            "{} {} {} {}",
            get_lines(&file_string),
            get_words(&file_string),
            get_bytes(&file_string),
            file_name
        )
    }

    for flag in flags {
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

    Ok(())
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
