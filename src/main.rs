use fltk::{app, prelude::*, *};
use std::path::Path;

fn main() {
    let app = app::App::default();
    
    // Setting up main menu window
    let mut wind = window::MenuWindow::default()
        .with_size(600, 450)
        .center_screen()
        .with_label("The Snake Game");
    wind.set_color(enums::Color::Black);
    wind.make_resizable(true);

    // Creating logo frame
    let mut fram = frame::Frame::default()
        .with_size(250, 200)
        .above_of(&wind, -100)
        .center_x(&wind);

    // Looking for the executable's parent directory
    let current_exe = std::env::current_dir()
        .unwrap();
    let exe_dir = current_exe.as_path()
        .parent()
        .unwrap();

    // Creating logo title using the parent path and the PNG file name
    let mut snake_logo: image::PngImage =
        image::PngImage::load(&Path::new(&(exe_dir.to_str().unwrap().to_owned() + "/snake_logo.png")))
        .unwrap();
    snake_logo.scale(250, 200, true, true);
    fram.set_image(Some(snake_logo));
    
    wind.end();
    wind.show();
    app.run().unwrap();
}