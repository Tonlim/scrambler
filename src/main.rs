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

use scrambler::scrambler;

fn main() -> iced::Result {
    ScramblerUi::run(Settings::default())
}

struct ScramblerUi {
    translated_value: String,
    input_value: String,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    TranslateWord,
}

impl iced::Sandbox for ScramblerUi {
    type Message = Message;

    fn new() -> Self {
        // #TODO: move this to  place where we can do something with the error
        // show it in a text field for example
        // current idea: somehow trigger a "Startup" message and do it in the update. (Add a text widget for the error)
        scrambler::storage::initialize_directory().unwrap();
        Self {
            translated_value: "".to_owned(),
            input_value: "".to_owned(),
        }
    }

    fn title(&self) -> String {
        String::from("Scrambler - Iced")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::InputChanged(value) => {
                self.input_value = value;
            }
            Message::TranslateWord => match scrambler::translate_word(&self.input_value) {
                Ok(translation) => {
                    self.translated_value = translation;
                }
                Err(error) => {
                    self.translated_value = error;
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
