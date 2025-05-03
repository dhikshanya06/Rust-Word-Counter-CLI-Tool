use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process;

fn main() {
    println!("📖 RUST FILE READER & WORD COUNTER 📖");
    println!("-------------------------------------");

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let file_path = &args[1];
        match count_words_in_file(file_path) {
            Ok(count) => {
                println!("File: {}", file_path);
                println!("Word count: {}", count);
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                process::exit(1);
            }
        }
    } else {
        interactive_mode();
    }
}

fn count_words_in_file(file_path: &str) -> io::Result<usize> {
    let contents = fs::read_to_string(file_path)?;
    Ok(count_words(&contents))
}

fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn interactive_mode() {
    loop {
        println!("\nPlease select an option:");
        println!("1. Count words in a file");
        println!("2. Count words from user input");
        println!("3. Create a sample text file");
        println!("4. Exit");
        print!("\nYour choice (1-4): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => count_words_from_file_interactive(),
            "2" => count_words_from_input(),
            "3" => create_sample_file(),
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn count_words_from_file_interactive() {
    print!("Enter the path to your text file: ");
    io::stdout().flush().unwrap();

    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path).unwrap();
    let file_path = file_path.trim();

    if !Path::new(file_path).exists() {
        println!("Error: File '{}' does not exist.", file_path);
        return;
    }

    match count_words_in_file(file_path) {
        Ok(count) => {
            println!("\n📊 RESULTS 📊");
            println!("File: {}", file_path);
            println!("Word count: {}", count);
        }
        Err(err) => {
            println!("Error reading file: {}", err);
        }
    }
}

fn count_words_from_input() {
    println!("Enter text (press Enter twice to finish):");

    let mut input = String::new();
    let mut line = String::new();

    loop {
        line.clear();
        io::stdin().read_line(&mut line).unwrap();

        if line.trim().is_empty() {
            break;
        }

        input.push_str(&line);
    }

    let count = count_words(&input);

    println!("\n📊 RESULTS 📊");
    println!("Word count: {}", count);
}

fn create_sample_file() {
    print!("Enter a filename for the sample text: ");
    io::stdout().flush().unwrap();

    let mut filename = String::new();
    io::stdin().read_line(&mut filename).unwrap();
    let filename = filename.trim();

    let sample_text = "This is a sample text file created by the Rust Word Counter application.\nIt contains multiple lines and several words to demonstrate the functionality.\nYou can use this file to test the word counting feature of this application.\nThe total word count of this sample text should be 50 words.";

    match fs::write(filename, sample_text) {
        Ok(_) => println!("Sample file '{}' created successfully.", filename),
        Err(err) => println!("Error creating file: {}", err),
    }
}

