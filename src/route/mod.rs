mod messages;
mod game_input;

pub fn print_intro() {
    messages::messages();
    messages::author();
    messages::separator();
    messages::play_prompt();
    messages::separator();
    messages::list_prompt();
    messages::separator();
    messages::exit_prompt();
    messages::separator();
}

pub fn take_game_inputs() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let words = game_input::read_input_and_select_option()?;
    Ok(words)
}
