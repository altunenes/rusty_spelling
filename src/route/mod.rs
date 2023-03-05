mod messages;

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