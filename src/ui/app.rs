use std::fmt::Formatter;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PasswordLevel(usize, &'static str);

impl Into<String> for PasswordLevel {
    fn into(self) -> String {
        self.1.into()
    }
}

impl std::fmt::Display for PasswordLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.1)
    }
}

impl PasswordLevel {
    fn get_pwd_generator(&self, expect_len: usize) -> passwords::PasswordGenerator {
        passwords::PasswordGenerator {
            length: expect_len,
            numbers: true,
            lowercase_letters: self.0 != 1,
            uppercase_letters: self.0 != 1,
            symbols: self.0 == 0,
            spaces: false,
            exclude_similar_characters: false,
            strict: false,
        }
    }
}

const OPTIONS: [PasswordLevel; 3] = [
    PasswordLevel(0, "随机生成"),
    PasswordLevel(1, "纯数字"),
    PasswordLevel(2, "字母与数字")
];


#[derive(Default)]
pub struct App {
    pick_state: iced::pick_list::State<PasswordLevel>,
    suggest_state: iced::text_input::State,
    slide_state: iced::slider::State,
    btn_copy_state: iced::button::State,
    btn_roll_state: iced::button::State,
    current_selected: Option<PasswordLevel>,
    current_suggestions: Option<String>,
    pwd_len: Option<u32>,
}

#[derive(Debug, Clone)]
pub enum AppMsg {
    PickChanged(PasswordLevel),
    SuggestionChanged(String),
    PwdLenChanged(u32),
    Roll,
    Copy2Clipboard,
}

impl App {
    fn generate_pwd(&self) -> String {
        let pwd_len = self.pwd_len.unwrap_or(12u32) as usize;
        let pwd_generator = self.current_selected.as_ref()
            .or_else(|| Some(&OPTIONS[0]))
            .map(|it| it.get_pwd_generator(pwd_len))
            .unwrap();
        pwd_generator.generate_one().unwrap_or_default()
    }
}

impl iced::Application for App {
    type Executor = iced::executor::Default;
    type Message = AppMsg;
    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Self::Message>) {
        let mut o = Self::default();
        o.current_suggestions = Some(o.generate_pwd());
        (o, iced::Command::none())
    }

    fn title(&self) -> String {
        "密码助理".into()
    }

    fn update(&mut self, message: Self::Message, clipboard: &mut iced::Clipboard)
              -> iced::Command<Self::Message> {
        println!("{:?}", message);
        match message {
            AppMsg::PickChanged(v) => {
                self.current_selected = Some(v);
                self.current_suggestions = Some(self.generate_pwd())
            }
            AppMsg::SuggestionChanged(s) => {
                self.current_suggestions = Some(s)
                // todo detect complex
            }
            AppMsg::PwdLenChanged(pwd_len) => {
                self.pwd_len = Some(pwd_len);
                self.current_suggestions = Some(self.generate_pwd())
            }
            AppMsg::Roll => {
                self.current_suggestions = Some(self.generate_pwd())
            }
            AppMsg::Copy2Clipboard => {
                if let Some(s) = &self.current_suggestions {
                    clipboard.write(s.clone())
                }
            }
        }
        iced::Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let spacer = |n| iced::Space::new(
            iced::Length::Units(n),
            iced::Length::Units(0));

        let line1 = iced::Row::new()
            .width(iced::Length::Fill)
            .align_items(iced::Align::Center)
            .spacing(5)
            .push(iced::Text::new("类型"))
            .push(spacer(10))
            .push({
                let cur_opt =
                    self.current_selected
                        .or_else(|| Some(OPTIONS[0].clone()));
                iced::PickList::new(&mut self.pick_state,
                                    &OPTIONS[..],
                                    cur_opt,
                                    |it| {
                                        AppMsg::PickChanged(it)
                                    })
                    .width(iced::Length::Fill)
            })
            .push(spacer(10));

        let suggestions = self.current_suggestions.clone().unwrap_or_default();
        let line2 = iced::Row::new()
            .width(iced::Length::Fill)
            .align_items(iced::Align::Center)
            .spacing(5)
            .push(iced::Text::new("建议"))
            .push(spacer(10))
            .push(iced::TextInput::new(
                &mut self.suggest_state,
                "",
                &suggestions,
                AppMsg::SuggestionChanged)
                .padding(3)
                .width(iced::Length::Fill)
                .size(26))
            .push(spacer(10));

        let line3 = iced::Row::new()
            .width(iced::Length::Fill)
            .align_items(iced::Align::Center)
            .spacing(5)
            .push(iced::Text::new("长度"))
            .push(spacer(10))
            .push(
                iced::Row::new()
                    .align_items(iced::Align::Center)
                    .push({
                        iced::Slider::new(&mut self.slide_state,
                                          8u32..=16u32,
                                          self.pwd_len.unwrap_or(12u32),
                                          |v| AppMsg::PwdLenChanged(v))
                            .width(iced::Length::Fill)
                    })
                    .push(spacer(5))
                    .push(iced::Text::new(
                        format!("{}", self.pwd_len.unwrap_or(12u32))))
            );

        let line4 = iced::Row::new()
            .padding(10)
            .align_items(iced::Align::Center)
            .width(iced::Length::Fill)
            .push({
                let btn = iced::Button::new(&mut self.btn_copy_state
                                            , iced::Text::new("拷贝密码")
                                                .width(iced::Length::Units(80))
                                                .horizontal_alignment(
                                                    iced::HorizontalAlignment::Center))
                    .on_press(AppMsg::Copy2Clipboard);
                iced::Container::new(btn)
                    .width(iced::Length::Fill)
                    .center_x()
                    .center_y()
            })
            .push({
                let btn = iced::Button::new(&mut self.btn_roll_state
                                            , iced::Text::new("再来一次")
                                                .width(iced::Length::Units(80))
                                                .horizontal_alignment(
                                                    iced::HorizontalAlignment::Center))
                    .on_press(AppMsg::Roll);
                iced::Container::new(btn)
                    .width(iced::Length::Fill)
                    .center_x()
                    .center_y()
            });

        let cols = iced::Column::new()
            .spacing(5)
            .align_items(iced::Align::Center)
            .push(line1)
            .push(line2)
            .push(line3)
            .push(line4);

        iced::Container::new(cols)
            .padding(10)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

