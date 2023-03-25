use std::io;
use rand::prelude::SliceRandom;
use tts::*;
use colored::*;
use crate::route::take_game_inputs;
use crate::results::*;
use structopt::StructOpt;
#[derive(StructOpt)]
pub struct GameInputs {
    #[structopt(short, long, default_value = "1.0")]
    pub speed_ratio: f32,
}

pub fn play_game() -> GameResult {
    let words = take_game_inputs().unwrap();

    let mut points = 0;
    let mut incorrect_guesses = 0;
    let mut correct_words = Vec::new();
    let mut incorrect_words = Vec::new();
    let mut rng = rand::thread_rng();
    let mut tts = Tts::default().unwrap();
    let game_inputs = GameInputs::from_args();

    if game_inputs.speed_ratio < 0.5 || game_inputs.speed_ratio > 10.0 {
        eprintln!("Error: speed_ratio must be between 0.5 and 10.0");
        std::process::exit(1);
    }

    tts.set_rate(game_inputs.speed_ratio).unwrap();


    'game_loop: while let Some(word) = words.choose(&mut rng) {
        tts.speak(word, true).unwrap();
        let hidden_word: String = word.chars().map(|_| "_ ".bold().to_string()).collect();
        println!("The word is: {}", hidden_word);
        println!("{}", "Enter your guess:".bold());

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let word = word.to_lowercase();
        let guess = guess.trim().to_lowercase();

        if guess == word {
            points += 1;
            correct_words.push(word.clone());
            println!("{} {}", "Correct!".green().bold(), format!("You now have {} points.", points).green());
        } else if guess == "/exit" {
            println!("Thanks for playing! Here are your results:");
            break 'game_loop;
        } else {
            incorrect_guesses += 1;
            incorrect_words.push(word.clone());
            println!("{} {}", "Incorrect.".red().bold(), format!("The word was {}.", word).red());
        }
    }

    println!("\n{}", format!("You scored {} points!", points).yellow().bold());
    println!("{}", "-".repeat(60).red());    
    println!("\n{}", format!("You made {} incorrect guesses.", incorrect_guesses).yellow().bold());

    if !correct_words.is_empty() {
        println!("\n{}", "You got the following words correct:".yellow().bold());
        let mut correct_counts = std::collections::HashMap::new();
        for word in &correct_words { *correct_counts.entry(word).or_insert(0) += 1; }
        for (word, count) in correct_counts {
            println!("{} {}", word, format!("({})", count).yellow());
        }
    }

    if !incorrect_words.is_empty() {
        println!("\n{}", "You got the following words incorrect:".yellow().bold());
        let mut incorrect_counts = std::collections::HashMap::new();
        for word in &incorrect_words { *incorrect_counts.entry(word).or_insert(0) += 1; }
        for (word, count) in incorrect_counts {
            println!("{} {}", word, format!("({})", count).yellow());
        }
    }
    GameResult {
        date_time: chrono::Local::now().to_string(),
        score: points,
        correct_words,
        incorrect_words,
        wrong_attempts: incorrect_guesses,
    }
}