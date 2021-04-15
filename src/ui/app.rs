use iced::Element;

#[derive(Default)]
pub struct App {
    pick_state: iced::pick_list::State<&'static str>,
    suggest_state: iced::text_input::State,
    slide_state: iced::slider::State,
    btn_copy_state: iced::button::State,
    btn_roll_state: iced::button::State,
    current_selected: Option<&'static str>,
    current_suggestions: Option<String>,
    pwd_len: Option<u32>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    PickChanged(&'static str),
    SuggestionChanged(String),
    PwdLenChanged(u32),
    Roll,
    Copy2Clipboard,
}

const OPTIONS: [&'static str; 3] = ["随机生成", "纯数字", "字母与数字"];

impl App {
    fn current_selected(&self) -> Option<&'static str> {
        self.current_selected.or_else(|| Some(OPTIONS[0]))
    }
}

impl iced::Application for App {
    type Executor = iced::executor::Default;
    type Message = Msg;
    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Self::Message>) {
        (Self::default(), iced::Command::none())
    }

    fn title(&self) -> String {
        "密码助理".into()
    }

    fn update(&mut self, message: Self::Message, clipboard: &mut iced::Clipboard)
              -> iced::Command<Self::Message> {
        println!("{:?}", message);
        match message {
            Msg::PickChanged(v) => {
                self.current_selected = Some(v)
            }
            Msg::SuggestionChanged(s) => {
                self.current_suggestions = Some(s)
            }
            Msg::PwdLenChanged(pwd_len) => {
                self.pwd_len = Some(pwd_len)
            }
            Msg::Roll => {}
            Msg::Copy2Clipboard => {
                if let Some(s) = &self.current_suggestions {
                    clipboard.write(s.clone())
                }
            }
        }
        iced::Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
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
                let cur_opt = self.current_selected();
                iced::PickList::new(&mut self.pick_state,
                                    &OPTIONS[..],
                                    cur_opt,
                                    |it| {
                                        Msg::PickChanged(it)
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
                Msg::SuggestionChanged)
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
                                          |v| Msg::PwdLenChanged(v))
                            .width(iced::Length::Fill)
                    })
                    .push(spacer(5))
                    .push(iced::Text::new(format!("{}", self.pwd_len.unwrap_or(8))))
            );

        let line4 = iced::Row::new()
            .align_items(iced::Align::Center)
            .width(iced::Length::Fill)
            .push({
                let btn = iced::Button::new(&mut self.btn_copy_state
                                            , iced::Text::new("Copy")
                                                .width(iced::Length::Units(80))
                                                .horizontal_alignment(
                                                    iced::HorizontalAlignment::Center))
                    .on_press(Msg::Copy2Clipboard);
                iced::Container::new(btn)
                    .width(iced::Length::Fill)
                    .center_x()
                    .center_y()
            })
            .push({
                let btn = iced::Button::new(&mut self.btn_roll_state
                                            , iced::Text::new("Roll")
                                                .width(iced::Length::Units(80))
                                                .horizontal_alignment(
                                                    iced::HorizontalAlignment::Center))
                    .on_press(Msg::Roll);
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

