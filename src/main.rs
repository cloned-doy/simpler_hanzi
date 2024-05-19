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

fn trim_hanzi(filename: &str, sentence: &str) -> io::Result<Vec<usize>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    
    let mut characters_found: Vec<(String, usize)> = Vec::new();
    let mut checked_characters = 0; // Number of checked characters in sentence
    
    while checked_characters < sentence.chars().count() {
        let mut prefix_len = 4.min(sentence.chars().count() - checked_characters); // Maximum length for the prefix
        
        while prefix_len > 0 {
            let word = &sentence[checked_characters..checked_characters + prefix_len];
            let mut found = false;
            
            for (line_number, line) in reader.lines().enumerate() {
                let line = line?;
                if line.contains(word) {
                    characters_found.push((word.to_string(), line_number + 1)); // Add 1 to line number to start from 1
                    println!("Found in wordlist: {}", word);
                    found = true;
                    break;
                }
            }
            
            if found {
                break; // Exit inner loop if found
            }
            
            prefix_len -= 1; // Decrease prefix length
        }
        
        checked_characters += prefix_len;
    }
    
    if characters_found.is_empty() {
        println!("No match found in wordlist.");
    }
    
    Ok(characters_found.iter().map(|&(_, ln)| ln).collect())
}

fn main() {
    let filenamecsv = "./src/data/hsk_simplified.csv"; 
    // let filenametxt = "./src/data/characters_simplified.txt";
    let sentence = "我感觉我们的人口越来越减少了";

    match find_word_in_csv(filenamecsv, sentence) {
        Ok(Some(line_number)) => println!("The word '{}' was found in line {}.", sentence, line_number),
        Ok(None) => println!("The word '{}' was not found in the CSV file.", sentence),
        Err(err) => eprintln!("Error: {}", err),
    }
    
    match trim_hanzi(filenamecsv, sentence) {
        Ok(line_numbers) => {
            if line_numbers.is_empty() {
                println!("The word '{}' was not found in the CSV file.", sentence);
            } else {
                for line_number in line_numbers {
                    println!("The word '{}' was found in line {}.", sentence, line_number);
                }
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }
    
}
