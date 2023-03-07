mod route;
mod results;
use tts::*;
use colored::*;
use results::*;
mod game;

fn main() -> Result<(), Error> {
    route::print_intro();
    let result = game::play_game();
    let output= format!("You scored {} points!", result.score);
    println!("{}", output.yellow().bold());
    let mut results = Results::new();
    results.add_result(result);
    results.save_results("results.json").unwrap();

    Ok(())
}