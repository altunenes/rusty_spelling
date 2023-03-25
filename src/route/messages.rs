use colored::*;

pub fn messages() {
        println!("\n{}\n", "Welcome to Rusty Spelling".green().bold().blink());
    }

    pub fn author() {
        let name = "enes altun".truecolor(118, 3, 250);
        let link = "https://github.com/altunenes".truecolor(255, 165, 0);
        println!("\n{}{}\n{}{}\n", "Author: ".truecolor(0, 255, 136), name, "GitHub: ".truecolor(0, 255, 136), link);
    }
    pub fn separator() {
        println!("{}", "-".repeat(60).red());
    }

    pub fn play_prompt() {
        let message = format!("{}{}{}",
            "If you want to play with words you want to practice print ".truecolor(0, 255, 0),
            "1".truecolor(255, 0, 0).bold(),
            " and enter".truecolor(0, 255, 0),
        );
        println!("{}", message);
    }
    
    pub fn list_prompt() {
        let message = format!("{}{}{}",
            "If you want to practice from a word_list.txt print ".truecolor(0, 255, 0),
            "2".truecolor(255, 0, 0).bold(),
            " and enter".truecolor(0, 255, 0),
        );
        println!("{}", message);
    }
    pub fn exit_prompt() {
        let prompt = "if you want to exit the game enter ".truecolor(0, 255, 0);
        let stats = " and see your stats".truecolor(0, 255, 0);
        let exit = "/exit".truecolor(255, 0, 0).bold();
        let message = format!("{}{}{}",
            prompt,
            exit,
            stats,
        );
        println!("{}", message);
    }
    pub fn play_mistake_promt(){
        let message = format!("{}{}{}",
            "If you want to play with your previous mistakes print ".truecolor(0, 255, 0),
            "3".truecolor(255, 0, 0).bold(),
            " and enter".truecolor(0, 255, 0),
        );
        println!("{}", message);
    }
    
    pub fn mistake_prompt() {
        let message = format!("{}{}{}",
            "If you want to see your previous mistakes print ".truecolor(0, 255, 0),
            "4".truecolor(255, 0, 0).bold(),
            " and enter".truecolor(0, 255, 0),
        );
        println!("{}", message);
    }