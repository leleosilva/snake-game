use fltk::{app, prelude::*, *};
use std::path::Path;

fn get_parent_path() -> String {
    
    // Looking for the executable's parent directory
    let current_exe: std::path::PathBuf = std::env::current_dir()
        .unwrap();
    let exe_dir: &std::path::Path = current_exe.as_path()
        .parent()
        .unwrap();
    exe_dir.to_str().unwrap().to_owned()
}

fn main() {
    let app = app::App::default();

    // Changing default font from Helvetica to Edit Undo
    let font = enums::Font::load_font(&Path::new(&(get_parent_path() + "/fonts/SilomBol.ttf"))).unwrap();
    enums::Font::set_font(enums::Font::Helvetica, &font);
    
    // Setting up main menu window
    let mut main_menu_window = window::MenuWindow::default()
        .with_size(700, 550)
        .center_screen()
        .with_label("The Snake Game");
    main_menu_window.set_color(enums::Color::Black);
    main_menu_window.set_border(true);
    main_menu_window.make_resizable(true);

    /*
    // Creating border frame
    let mut border = frame::Frame::default()
        .with_size(600, 400)
        .below_of(&main_menu_window, -515)
        .center_x(&main_menu_window);
    border.set_frame(enums::FrameType::BorderFrame);
    border.set_color(enums::Color::White);
    */
    
    // Creating title frame
    let mut title_frame = frame::Frame::default()
        .with_size(250, 200)
        .above_of(&main_menu_window, -220)
        .center_x(&main_menu_window)
        .with_label("Snake");
    title_frame.set_label_color(enums::Color::White);
    title_frame.set_label_size(150);
    title_frame.set_label_type(enums::LabelType::Shadow);

    // Creating button to start the game
    let mut start_button = button::Button::default()
        .with_label("START");
    start_button.set_color(enums::Color::Black);
    start_button.set_label_color(enums::Color::White);
    start_button.set_label_size(24);

    // Creating button to show how to play the game
    let mut how_to_play_button = button::Button::default()
        .with_label("HOW TO\nPLAY");
    how_to_play_button.set_color(enums::Color::Black);
    how_to_play_button.set_label_color(enums::Color::White);
    how_to_play_button.set_label_size(24);

    // Creating button to quit the application
    let mut quit_button = button::Button::default()
        .with_label("QUIT");
    quit_button.set_color(enums::Color::Black);
    quit_button.set_label_color(enums::Color::White);
    quit_button.set_label_size(24);

    // Grouping buttons in main menu
    let mut buttons_grid = group::VGrid::default()
    .with_size(440, 70)
    .below_of(&main_menu_window, -300)
    .center_x(&main_menu_window);
    buttons_grid.set_params(1, 3, 75);
    
    buttons_grid.add(&start_button);
    buttons_grid.add(&how_to_play_button);
    buttons_grid.add(&quit_button);
    buttons_grid.end();
    
    main_menu_window.end();
    main_menu_window.show();

    app.run().unwrap();
}