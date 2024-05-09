use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn find_word_in_csv(filename: &str, word: &str) -> Result<Option<usize>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;
        if line.contains(word) {
            return Ok(Some(line_number)); // u can add + 1 because line numbers start from 1
        }
    }

    Ok(None)
}


fn find_word_in_file(file_path: &str, word_to_find: &str) -> io::Result<Vec<usize>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut line_number = 0;
    let mut found_line_numbers = Vec::new();

    for line in reader.lines() {
        line_number += 1;
        let line = line?;
        if line.contains(word_to_find) {
            found_line_numbers.push(line_number);
        }
    }

    Ok(found_line_numbers)
}


fn main() {
    let filenamecsv = "./src/data/hsk_simplified.csv"; 
    let filenametxt = "./src/data/characters_simplified.txt";
    let word_to_find = "人口";

    match find_word_in_csv(filenamecsv, word_to_find) {
        Ok(Some(line_number)) => println!("The word '{}' was found in line {}.", word_to_find, line_number),
        Ok(None) => println!("The word '{}' was not found in the CSV file.", word_to_find),
        Err(err) => eprintln!("Error: {}", err),
    }
    match find_word_in_file(filenametxt, word_to_find) {
        Ok(line_numbers) => {
            if line_numbers.is_empty() {
                println!("Word '{}' not found in the file.", word_to_find);
            } else {
                println!("Word '{}' found at line(s): {:?}", word_to_find, line_numbers);
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
