use chrono::DateTime;
use chrono::Utc;
use iced::alignment;
use iced::widget::button;
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
use itertools::Itertools;
use log::error;
use unicode_segmentation::UnicodeSegmentation;

use ::scrambler::scrambler::Glyph;
use ::scrambler::scrambler::Translation;
use scrambler::scrambler;

fn main() -> iced::Result {
    env_logger::init();
    ScramblerUi::run(Settings::default())
}

struct ScramblerUi {
    translated_value: Option<Translation>,
    suggested_translation: Option<Translation>,
    input_value: String,
    alphabet_input: String,
    current_alphabet: Vec<Glyph>,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    TranslateWord,
    TranslationAccepted(String, Translation),
    TranslationRejected,
    TranslationBlocked(String),
    AlphabetInputChanged(String),
    AddToAlphabet,
    AlphabetLoaded(Vec<Glyph>),
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
                suggested_translation: None,
                input_value: "".to_owned(),
                alphabet_input: "".to_owned(),
                current_alphabet: Vec::new(),
            },
            Command::perform(
                scrambler::storage::load_alphabet_async(),
                Message::AlphabetLoaded,
            ),
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
            Message::TranslateWord => self.translate_input(),
            Message::TranslationAccepted(original, translation) => {
                if let Err(error) = scrambler::save_translation(&original, translation) {
                    error!("{error}");
                }
                self.translate_input();
            }
            Message::TranslationRejected => self.translate_input(),
            Message::TranslationBlocked(word) => {
                if let Err(error) = scrambler::add_to_block_list(&word) {
                    error!("{error}");
                }
                self.translate_input();
            }
            Message::AlphabetInputChanged(value) => {
                let char_count = value.graphemes(true).count();
                // We only accept a single non-whitespace character.
                // And of course an empty field. Otherwise, people can't delete the character.
                if char_count == 0 || (char_count == 1 && !value.trim().is_empty()) {
                    self.alphabet_input = value;
                }
            }
            Message::AddToAlphabet => {
                if !self.alphabet_input.is_empty() {
                    if let Err(error) = scrambler::add_to_alphabet(&self.alphabet_input) {
                        error!("{}", error.to_string());
                    }
                    self.alphabet_input = "".to_owned();

                    self.current_alphabet = scrambler::storage::load_alphabet();
                }
            }
            Message::AlphabetLoaded(alphabet_result) => self.current_alphabet = alphabet_result,
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

        let suggested_translation;
        if let Some(value) = &self.suggested_translation {
            let accept_button = button("Accept translation").on_press(
                Message::TranslationAccepted(self.input_value.clone(), value.clone()),
            );
            let reset_button =
                button("Generate new translation").on_press(Message::TranslationRejected);
            let block_button = button("Block translation and generate a new one")
                .on_press(Message::TranslationBlocked(value.translation.clone()));
            suggested_translation = row![
                text(&value.translation),
                accept_button,
                reset_button,
                block_button
            ];
        } else {
            suggested_translation = row![];
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

        let alphabet_text = text(
            "The current alphabet is: ".to_owned()
                + &self
                    .current_alphabet
                    .iter()
                    .map(|glyph| &glyph.symbol)
                    .join(""),
        );

        let translation_column = column![input, translation, suggested_translation, lookup_feature]
            .spacing(20)
            .max_width(800);

        let alphabet_column = column![alphabet_input, remove_alphabet_feature, alphabet_text]
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

impl ScramblerUi {
    fn translate_input(&mut self) {
        self.translated_value = None;
        self.suggested_translation = None;

        match scrambler::translate_word(&self.input_value) {
            Ok(translation) => match scrambler::is_word_known(&self.input_value) {
                true => self.translated_value = Some(translation),
                false => self.suggested_translation = Some(translation),
            },
            Err(error) => {
                error!("{error}");
            }
        }
    }
}
