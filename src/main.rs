use std::io;
use rand::prelude::SliceRandom;
use tts::*;

const ASCII_ART_1: &str = "\
 _____ _____ _____ _____ _____ _____ _____ _____ 
|     |     |     |     |     |     |     |     |
|  S  |  P  |  E  |  L  |  L  |  I  |  N  |  G  |
|_____|_____|_____|_____|_____|_____|_____|_____|\n";

const ASCII_ART_2: &str = "\
Type 'exit' to end the program.\n";

fn main() -> Result<(), Error> {
    println!("{}", ASCII_ART_1);
    println!("{}", ASCII_ART_2);
    println!("Enter the words you want to practice, separated by commas:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let words: Vec<String> = input
        .split(',')
        .map(|word| word.trim().to_string())
        .filter(|word| !word.is_empty())
        .collect();

    if words.is_empty() {
        println!("No words were entered. Please try again.");
        return Ok(());
    }

    let mut points = 0;
    let mut incorrect_guesses = 0;
    let mut correct_words = Vec::new();
    let mut rng = rand::thread_rng();
    let mut tts = Tts::default()?;

    'game_loop: while let Some(word) = words.choose(&mut rng) {
        tts.speak(word, true)?;
        let hidden_word: String = word.chars().map(|_| '_').collect();
        println!("The word is: {}", hidden_word);
        println!("Enter your guess:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        guess = guess.trim().to_string();

        if guess == word {
            points += 1;
            correct_words.push(word.clone());
            println!("Correct! You now have {} points.", points);
        } else if guess == "exit" {
            break 'game_loop;
        } else {
            incorrect_guesses += 1;
            println!("Incorrect. The word was {}.", word);
        }
    }

    println!("\nYou scored {} points!", points);
    println!("\nYou made {} incorrect guesses.", incorrect_guesses);
    println!("\nYou got the following words correct:");
    for word in correct_words {
        println!("{}", word);
    }

    Ok(())
}