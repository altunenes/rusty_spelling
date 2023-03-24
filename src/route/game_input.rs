use std::io::{self, BufRead};
use std::fs::File;
use std::io::BufReader;
use colored::*;

pub fn read_input_and_select_option() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let selected_option = input.trim();

    let mut words: Vec<String> = Vec::new();

    if selected_option == "1" {
        println!("{}", "Enter the words you want to practice, separated by commas. Type '/exit' to end the game:".green());
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        words = input.split(',')
            .map(|word| word.trim().to_string())
            .filter(|word| !word.is_empty())
            .collect();
    } else if selected_option == "2" {
    let file = File::open("word_list.txt").map_err(|_| "Could not find word_list.txt file. Please make sure the file exists in the current directory.")?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(word) = line {
                words.push(word.trim().to_string());
            }
        }
    } else {
        println!("{}", "Invalid option selected. Exiting game.".red());
        return Ok(vec![]);
    }

    if words.is_empty() {
        println!("{}", "No words were entered. Please try again.".red());
        return Ok(vec![]);
    }

    Ok(words)
}
