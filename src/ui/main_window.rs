use std::fmt::Formatter;

use super::kit::Kit;

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
struct FormStates {
    pick_state: iced::pick_list::State<PasswordLevel>,
    suggest_state: iced::text_input::State,
    slide_state: iced::slider::State,
    btn_copy_state: iced::button::State,
    btn_roll_state: iced::button::State,
}

#[derive(Default)]
pub struct Form {
    states: std::cell::UnsafeCell<FormStates>,
    current_selected: Option<PasswordLevel>,
    current_suggestions: Option<String>,
    pwd_score: Option<String>,
    pwd_len: Option<u32>,
}

#[derive(Debug, Clone)]
pub enum FormMessage {
    PickChanged(PasswordLevel),
    SuggestionChanged(String),
    PwdLenChanged(u32),
    Roll,
    Copy2Clipboard,
}

impl Form {
    fn get_mut_states(&self) -> &mut FormStates {
        unsafe {
            &mut *self.states.get()
        }
    }

    fn generate_pwd(&self) -> String {
        let pwd_len = self.pwd_len.unwrap_or(12u32) as usize;
        let pwd_generator = self.current_selected.as_ref()
            .or_else(|| Some(&OPTIONS[0]))
            .map(|it| it.get_pwd_generator(pwd_len))
            .unwrap();
        pwd_generator.generate_one().unwrap_or_default()
    }

    fn update_suggestion_password(&mut self) {
        self.current_suggestions = Some(self.generate_pwd());
    }

    fn update_password_score(&mut self) {
        let cur_pwd = self.current_suggestions.clone().unwrap_or_default();
        if cur_pwd.is_empty() {
            return;
        }
        let ap = passwords::analyzer::analyze(&cur_pwd);
        let is_comm_pwd = ap.is_common();
        let score = passwords::scorer::score(&ap);
        self.pwd_score = Some(format!("强度:{}, 常见:{}",
                                      score as u32,
                                      if is_comm_pwd { "是" } else { "否" }))
    }

    #[inline]
    fn make_line1(&self) -> iced::Row<FormMessage> {
        let cur_opt =
            self.current_selected.clone()
                .or_else(|| Some(OPTIONS[0].clone()));

        let pick_list = iced::PickList::new(
            &mut self.get_mut_states().pick_state,
            &OPTIONS[..],
            cur_opt,
            |it| {
                FormMessage::PickChanged(it)
            })
            .width(iced::Length::Fill);

        iced::Row::new()
            .width(iced::Length::Fill)
            .align_items(iced::Align::Center)
            .spacing(5)
            .push(iced::Text::new("类型"))
            .push(Kit::make_h_spacer(10))
            .push(pick_list)
            .push(Kit::make_h_spacer(10))
            .height(iced::Length::Units(30))
    }

    #[inline]
    fn make_line2(&self) -> iced::Row<FormMessage> {
        let suggestions = self.current_suggestions.clone().unwrap_or_default();
        iced::Row::new()
            .width(iced::Length::Fill)
            .align_items(iced::Align::Center)
            .spacing(5)
            .push(iced::Text::new("建议"))
            .push(Kit::make_h_spacer(10))
            .push(iced::TextInput::new(
                &mut self.get_mut_states().suggest_state,
                "",
                &suggestions,
                FormMessage::SuggestionChanged)
                .padding(3)
                .width(iced::Length::Fill)
                .size(26))
            .push(Kit::make_h_spacer(10))
            .height(iced::Length::Units(30))
    }

    #[inline]
    fn make_line3(&self) -> iced::Row<FormMessage> {
        iced::Row::new()
            .width(iced::Length::Fill)
            .align_items(iced::Align::Center)
            .spacing(5)
            .push(iced::Text::new("长度"))
            .push(Kit::make_h_spacer(10))
            .push(
                iced::Row::new()
                    .align_items(iced::Align::Center)
                    .push({
                        iced::Slider::new(&mut self.get_mut_states().slide_state,
                                          8u32..=16u32,
                                          self.pwd_len.unwrap_or(12u32),
                                          |v| FormMessage::PwdLenChanged(v))
                            .width(iced::Length::Fill)
                    })
                    .push(Kit::make_h_spacer(5))
                    .push(iced::Text::new(
                        format!("{}", self.pwd_len.unwrap_or(12u32))))
            )
            .height(iced::Length::Units(30))
    }

    #[inline]
    fn make_line4(&self) -> iced::Row<FormMessage> {
        iced::Row::new()
            .width(iced::Length::Fill)
            .align_items(iced::Align::Center)
            .spacing(5)
            .push(iced::Text::new("评分"))
            .push(Kit::make_h_spacer(10))
            .push(iced::Text::new(
                self.pwd_score.clone().unwrap_or_default()))
            .height(iced::Length::Units(30))
    }

    #[inline]
    fn make_line5(&self) -> iced::Row<FormMessage> {
        iced::Row::new()
            .align_items(iced::Align::Center)
            .width(iced::Length::Fill)
            .push({
                let btn =
                    iced::Button::new(&mut self.get_mut_states().btn_copy_state
                                      , iced::Text::new("拷贝密码")
                                          .width(iced::Length::Units(80))
                                          .horizontal_alignment(
                                              iced::HorizontalAlignment::Center))
                        .on_press(FormMessage::Copy2Clipboard);
                iced::Container::new(btn)
                    .width(iced::Length::Fill)
                    .center_x()
                    .center_y()
            })
            .push({
                let btn =
                    iced::Button::new(&mut self.get_mut_states().btn_roll_state
                                      , iced::Text::new("再来一次")
                                          .width(iced::Length::Units(80))
                                          .horizontal_alignment(
                                              iced::HorizontalAlignment::Center))
                        .on_press(FormMessage::Roll);
                iced::Container::new(btn)
                    .width(iced::Length::Fill)
                    .center_x()
                    .center_y()
            })
            .height(iced::Length::Units(50))
    }
}

impl iced::Application for Form {
    type Executor = iced::executor::Default;
    type Message = self::FormMessage;
    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Self::Message>) {
        let mut o = Self::default();
        o.update_suggestion_password();
        o.update_password_score();
        (o, iced::Command::none())
    }

    fn title(&self) -> String {
        "密码助理".into()
    }

    fn update(&mut self, message: Self::Message, clipboard: &mut iced::Clipboard)
              -> iced::Command<Self::Message> {
        println!("{:?}", message);
        match message {
            FormMessage::PickChanged(v) => {
                self.current_selected = Some(v);
                self.update_suggestion_password();
                self.update_password_score()
            }
            FormMessage::SuggestionChanged(s) => {
                self.current_suggestions = Some(s);
                self.update_password_score()
            }
            FormMessage::PwdLenChanged(pwd_len) if self.pwd_len.unwrap_or_default() != pwd_len => {
                self.pwd_len = Some(pwd_len);
                self.update_suggestion_password();
                self.update_password_score()
            }
            FormMessage::Roll => {
                self.update_suggestion_password();
                self.update_password_score()
            }
            FormMessage::Copy2Clipboard => {
                if let Some(s) = &self.current_suggestions {
                    clipboard.write(s.clone())
                }
            }
            _ => {}
        }
        iced::Command::none()
    }

    fn view(&mut self) -> iced::Element<Self::Message> {
        let cols = iced::Column::new()
            .spacing(5)
            .align_items(iced::Align::Center)
            .push(self.make_line1())
            .push(self.make_line2())
            .push(self.make_line3())
            .push(self.make_line4())
            .push(self.make_line5());
        iced::Container::new(cols)
            .padding(10)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

