use colored::*;

pub fn messages() {
        println!("\n{}\n", "Welcome to Rusty Spelling".green().bold().blink());
    }

    pub fn author() {
        println!("\n{}\n", "Author: altunenes".truecolor(118, 3, 250));
    }

    pub fn separator() {
        println!("{}", "-".repeat(60).red());
    }

    pub fn play_prompt() {
        println!("{}", "If you want to play with words you want to practice print 1 and enter".truecolor(0, 255, 0));
    }

    pub fn list_prompt() {
        println!("{}", "If you want to practice from a word_list.txt print 2 and enter".truecolor(0, 255, 136));
    }

    pub fn exit_prompt() {
        println!("{}", "if you want to exit the game enter /exit and see your stats".truecolor(0, 255, 136));
    }
