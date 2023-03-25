use std::io::{self, BufRead};
use std::fs::File;
use std::io::BufReader;
use colored::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct GameResult {
    date_time: String,
    score: i32,
    correct_words: Vec<String>,
    incorrect_words: Vec<String>,
    wrong_attempts: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct GameResults {
    game_results: Vec<GameResult>,
}

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
    }
    else if selected_option == "3" {
        let file = File::open("results.json").map_err(|_| "Could not find results.json file. Please make sure the file exists in the current directory. To create a JSON file you have to play at least one game")?;
            let reader = BufReader::new(file);
            let game_results: GameResults = serde_json::from_reader(reader)?;
            for result in game_results.game_results {
                for word in result.incorrect_words {
                    words.push(word);
                }
            }
    
        }
    else if selected_option == "4" {
        let file = File::open("results.json").map_err(|_| "Could not find results.json file. Please make sure the file exists in the current directory. To create a JSON file you have to play at least one game")?;
        let reader = BufReader::new(file);
        let game_results: GameResults = serde_json::from_reader(reader)?;
        for result in game_results.game_results {
            println!("Incorrect words from {}: {:?}", result.date_time, result.incorrect_words);
        }
        std::process::exit(0);

    } 


    else {
        println!("{}", "Invalid option selected. Exiting game.".red());
        return Ok(vec![]);
    }

    if words.is_empty() {
        println!("{}", "No words were entered. Please try again.".red());
        return Ok(vec![]);
    }

    Ok(words)
}


