use chrono::DateTime;
use chrono::Utc;
use iced::alignment;
use iced::widget::column;
use iced::widget::container;
use iced::widget::row;
use iced::widget::scrollable;
use iced::widget::text;
use iced::widget::text_input;
use iced::Application;
use iced::Color;
use iced::Command;
use iced::Length;
use iced::Settings;
use log::error;

use ::scrambler::scrambler::Translation;
use scrambler::scrambler;
use unicode_segmentation::UnicodeSegmentation;

fn main() -> iced::Result {
    env_logger::init();
    ScramblerUi::run(Settings::default())
}

struct ScramblerUi {
    translated_value: Option<Translation>,
    input_value: String,
    alphabet_input: String,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    TranslateWord,
    AlphabetInputChanged(String),
    AddToAlphabet,
}

impl iced::Application for ScramblerUi {
    type Message = Message;

    type Executor = iced::executor::Default;

    type Theme = iced::theme::Theme;

    type Flags = ();

    fn new(_: ()) -> (Self, Command<Message>) {
        (
            Self {
                translated_value: None,
                input_value: "".to_owned(),
                alphabet_input: "".to_owned(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Scrambler - Iced")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::InputChanged(value) => {
                self.input_value = value;
            }
            Message::TranslateWord => match scrambler::translate_word(&self.input_value) {
                Ok(translation) => {
                    self.translated_value = Some(translation);
                }
                Err(error) => {
                    error!("{}", error.to_string());
                    self.translated_value = None;
                }
            },
            Message::AlphabetInputChanged(value) => {
                let char_count = value.graphemes(true).count();
                // We only accept a single non-whitespace character.
                // And of course an empty field. Otherwise, people can't delete the character.
                if char_count == 0 || (char_count == 1 && !value.trim().is_empty()) {
                    self.alphabet_input = value;
                }
            }
            Message::AddToAlphabet => {
                if let Err(error) = scrambler::add_to_alphabet(&self.alphabet_input) {
                    error!("{}", error.to_string());
                }
                self.alphabet_input = "".to_owned();
            }
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

        let translation;
        if let Some(value) = &self.translated_value {
            let timestamp: DateTime<Utc> = value.time_added.into();
            let timestamp = timestamp.to_rfc3339();
            translation = row![text(&value.translation), text(timestamp)].spacing(10);
        } else {
            translation = row![];
        }

        let lookup_feature =
            text("For looking up existing words, please search the file in the data directory.");

        let alphabet_input = text_input(
            "Which letter needs to be added to the alphabet?",
            &self.alphabet_input,
        )
        .on_input(Message::AlphabetInputChanged)
        .on_submit(Message::AddToAlphabet)
        .padding(15);

        let remove_alphabet_feature = text("For removing a character from the alphabet, please remove it from the file in the data directory.");

        let translation_column = column![input, translation, lookup_feature]
            .spacing(20)
            .max_width(800);

        let alphabet_column = column![alphabet_input, remove_alphabet_feature]
            .spacing(20)
            .max_width(800);

        let body = row![translation_column, alphabet_column];

        let content = column![title, body].spacing(20).max_width(1600);

        scrollable(
            container(content)
                .width(Length::Fill)
                .padding(40)
                .center_x(),
        )
        .into()
    }
}
