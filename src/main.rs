use std::io;
use rand::prelude::SliceRandom;
use tts::*;
use colored::*;

fn main() -> Result<(), Error> {
    println!("{}", "Enter the words you want to practice, separated by commas. Type '/exit' to end the game:".green());

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let words: Vec<String> = input
        .split(',')
        .map(|word| word.trim().to_string())
        .filter(|word| !word.is_empty())
        .collect();

    if words.is_empty() {
        println!("{}", "No words were entered. Please try again.".red());
        return Ok(());
    }

    let mut points = 0;
    let mut incorrect_guesses = 0;
    let mut correct_words = Vec::new();
    let mut incorrect_words = Vec::new();
    let mut rng = rand::thread_rng();
    let mut tts = Tts::default()?;

    'game_loop: while let Some(word) = words.choose(&mut rng) {
        tts.speak(word, true)?;
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
    println!("\n{}", format!("You made {} incorrect guesses.", incorrect_guesses).yellow().bold());

    if !correct_words.is_empty() {
        println!("\n{}", "You got the following words correct:".yellow().bold());
        let mut correct_counts = std::collections::HashMap::new();
        for word in &correct_words {
            *correct_counts.entry(word).or_insert(0) += 1;
        }
        for (word, count) in correct_counts.iter() {
            println!("{} ({})", word.green().bold(), count);
        }
    }

    if !incorrect_words.is_empty() {
        println!("\n{}", "You got the following words incorrect:".yellow().bold());
        let mut incorrect_counts = std::collections::HashMap::new();
        for word in &incorrect_words {
            *incorrect_counts.entry(word).or_insert(0) += 1;
        }
        for (word, count) in incorrect_counts.iter() {
            println!("{} ({})", word.red().bold(), count);
        }
    }

    Ok(())
}