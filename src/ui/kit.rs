pub struct Kit;

impl Kit {
    pub fn make_h_spacer(width: u16) -> iced::Space {
        iced::Space::new(
            iced::Length::Units(width),
            iced::Length::Units(0))
    }
}