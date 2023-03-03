use std::io;
use rand::prelude::SliceRandom;
use tts::*;
use colored::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;


fn main() -> Result<(), Error> {

    println!("\n{}\n", "Welcome to Word Practice Game!".green().bold().blink());
    println!("\n{}\n", "Author: altunenes".truecolor(118, 3, 250));
    println!("{}", "-".repeat(60).red());    
    println!("{}", "If you want to play with words you want to practice print 1 and enter".truecolor(0,255,0));
    println!("{}", "-".repeat(60).red());    
    println!("{}", "If you want to practice from a word_list.txt print 2 and enter".truecolor(0, 255, 136));
    println!("{}", "-".repeat(60).red());
    println!("{}", "if you want to exit the game enter /exit and see your stats".truecolor(0, 255, 136));
    println!("{}", "-".repeat(60).red());


    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let selected_option = input.trim();

    let mut words: Vec<String> = Vec::new();
    if selected_option == "1" {
        println!("{}", "Enter the words you want to practice, separated by commas. Type '/exit' to end the game:".green());
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        words = input
            .split(',')
            .map(|word| word.trim().to_string())
            .filter(|word| !word.is_empty())
            .collect();
    } else if selected_option == "2" {
        let file = File::open("word_list.txt").unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(word) = line {
                words.push(word.trim().to_string());
            }
        }
    } else {
        println!("{}", "Invalid option selected. Exiting game.".red());
        return Ok(());
    }

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



    Ok(())
}
