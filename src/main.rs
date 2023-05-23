use iced::alignment;
use iced::widget::column;
use iced::widget::container;
use iced::widget::scrollable;
use iced::widget::text;
use iced::widget::text_input;
use iced::Color;
use iced::Length;
use iced::Sandbox;
use iced::Settings;

fn main() -> iced::Result {
    Counter::run(Settings::default())
}

struct Counter {
    translated_value: String,
    input_value: String,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    TranslateWord,
}

impl iced::Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self {
            translated_value: "".to_owned(),
            input_value: "".to_owned(),
        }
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::InputChanged(value) => {
                self.input_value = value;
            }
            Message::TranslateWord => match self.input_value.split_whitespace().count() {
                0 => {
                    self.translated_value = "".to_owned();
                }
                1 => {
                    self.translated_value =
                        "Translation of \"".to_owned() + &self.input_value + "\".";
                }
                _ => {
                    self.translated_value = "Error! I can only translate single words. The input \""
                        .to_owned()
                        + &self.input_value
                        + "\" is not a single word."
                }
            },
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let title = text("Scrambler")
            .width(Length::Fill)
            .size(100)
            .style(Color::from([0.5, 0.5, 0.5]))
            .horizontal_alignment(alignment::Horizontal::Center);

        let input = text_input("What needs to be translated?", &self.input_value)
            .on_input(Message::InputChanged)
            .on_submit(Message::TranslateWord)
            .padding(15)
            .size(30);

        let translation = column![text(&self.translated_value)].spacing(10);

        let content = column![title, input, translation]
            .spacing(20)
            .max_width(800);

        scrollable(
            container(content)
                .width(Length::Fill)
                .padding(40)
                .center_x(),
        )
        .into()
    }
}
