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
        let prompt = "if you want to exit the game enter ".truecolor(0, 255, 0);
        let stats = " and see your stats".truecolor(0, 255, 0);
        let exit = "/exit".truecolor(255, 0, 0);
        let message = format!("{}{}{}",
            prompt,
            exit,
            stats,
        );
        println!("{}", message);
    }
    pub fn mistake_prompt() {
        println!("{}", "if you want to see your previous mistakes print 4 and enter".truecolor(0, 255, 136));
    }
    pub fn play_mistake_promt(){
        println!("{}", "if you want to play with your previous mistakes print 3 and enter".truecolor(0, 255, 136));
    }
