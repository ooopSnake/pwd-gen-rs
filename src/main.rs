#![feature(cell_leak)]
#![windows_subsystem = "windows"]

use iced::Application;

mod ui;

fn main() {
    let mut settings: iced::Settings<()> = Default::default();
    settings.window.size = (300, 220);
    settings.window.resizable = false;
    settings.window.icon = iced::window::Icon::from_rgba(
        include_bytes!("../icon.rgba")[..].into(),
        94u32,
        127u32)
        .ok();
    settings.default_font = Some(include_bytes!("../fonts/pingfang.ttf"));
    ui::main_window::Form::run(settings).unwrap();
}
