use iced::alignment;
use iced::widget::column;
use iced::widget::container;
use iced::widget::scrollable;
use iced::widget::text;
use iced::widget::text_input;
use iced::Application;
use iced::Color;
use iced::Command;
use iced::Length;
use iced::Settings;

use scrambler::scrambler;

fn main() -> iced::Result {
    ScramblerUi::run(Settings::default())
}

struct ScramblerUi {
    translated_value: String,
    input_value: String,
    messages: Vec<String>,
}

#[derive(Debug, Clone)]
enum Message {
    // A bit ugly to put a string here instead of the strongly typed error. But `Box<dyn Error>` cannot be used across threads, which is a requirement for UI stuff.
    // So, String it us.
    Loaded(Result<scrambler::Storage, String>),
    InputChanged(String),
    TranslateWord,
}

impl iced::Application for ScramblerUi {
    type Message = Message;

    type Executor = iced::executor::Default;

    type Theme = iced::theme::Theme;

    type Flags = ();

    fn new(_: ()) -> (Self, Command<Message>) {
        (
            Self {
                translated_value: "".to_owned(),
                input_value: "".to_owned(),
                messages: Vec::new(),
            },
            // TODO: generic function that unwraps Box<dyn Error> into a string
            Command::perform(scrambler::initialize(), |value| {
                Message::Loaded(value.map_err(|error| error.to_string()))
            }),
        )
    }

    fn title(&self) -> String {
        String::from("Scrambler - Iced")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::Loaded(Ok(_storage)) => {
                // #TODO do something with storage
            }
            Message::Loaded(Err(message)) => {
                self.messages.push(message.to_string());
            }
            Message::InputChanged(value) => {
                self.input_value = value;
            }
            Message::TranslateWord => match scrambler::translate_word(&self.input_value) {
                Ok(translation) => {
                    self.translated_value = translation;
                }
                Err(error) => {
                    self.translated_value = error.to_string();
                }
            },
        }

        Command::none()
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

        let messages = text(&self.messages.join("\n")).style(Color::from_rgb(255.0, 0.0, 0.0));

        let content = column![title, input, translation, messages]
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
