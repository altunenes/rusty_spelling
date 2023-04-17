use druid::{
    widget::{Button, Flex, Label, TextBox},
    AppLauncher, Data, Lens, LocalizedString, Widget, WidgetExt, WindowDesc,
};

#[derive(Clone, Data, Lens)]
struct AppState {
    custom_words: String,
}

pub fn run_gui() {
    let main_window = WindowDesc::new(build_ui())
        .title(LocalizedString::new("spelling-game-title").with_placeholder("Rusty Spelling"))
        .window_size((400.0, 300.0));

    let app_state = AppState {
        custom_words: String::new(),
    };

    AppLauncher::with_window(main_window)
        .launch(app_state)
        .expect("Failed to launch application");
}

fn build_ui() -> impl Widget<AppState> {
    let custom_word_label = Label::new("Enter custom words (comma separated):");
    let custom_word_input = TextBox::new().lens(AppState::custom_words);

    let start_custom_game = Button::new("Start Custom Game")
        .on_click(|_ctx, data: &mut AppState, _env| {
            println!("Starting custom game with words: {}", data.custom_words);
            //  TODO Call  game logic here with the words from the custom_words field
        });

    let start_txt_game = Button::new("Start Game with word_list.txt").on_click(|_ctx, _data, _env| {
        println!("Starting game with word_list.txt");
        // TODO Call  game logic here with the words from  word_list.txt
    });

    let start_mistake_game =
        Button::new("Start Game with Previous Mistakes").on_click(|_ctx, _data, _env| {
            println!("Starting game with previous mistakes");
            // TODO Call  game logic here with the words from the previous mistakes
        });

    Flex::column()
        .with_child(custom_word_label)
        .with_child(custom_word_input)
        .with_spacer(8.0)
        .with_child(start_custom_game)
        .with_spacer(8.0)
        .with_child(start_txt_game)
        .with_spacer(8.0)
        .with_child(start_mistake_game)
}