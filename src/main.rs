use iced::Application;

mod ui;

fn main() {
    let mut settings = iced::Settings::default();
    settings.window.size = (300, 180);
    settings.window.resizable = false;
    settings.default_font = Some(include_bytes!("../fonts/pingfang.ttf"));
    ui::App::run(settings).unwrap();
}
