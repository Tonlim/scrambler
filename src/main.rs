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
    translated_value: Option<String>,
    suggested_translations: Vec<(String, Translation)>,
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
    DummyToMakeTextInputSelectable(String),
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
                suggested_translations: Vec::new(),
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
            Message::DummyToMakeTextInputSelectable(_) => {}
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
            translation =
                row![text_input("", &value).on_input(Message::DummyToMakeTextInputSelectable)]
                    .spacing(10);
        } else {
            translation = row![];
        }

        let mut suggested_translations_view = column![];
        if !self.suggested_translations.is_empty() {
            for value in self.suggested_translations.iter() {
                let accept_button = button("Accept translation").on_press(
                    Message::TranslationAccepted(value.0.clone(), value.1.clone()),
                );
                let reset_button =
                    button("Generate new translation").on_press(Message::TranslationRejected);
                let block_button = button("Block translation and generate a new one")
                    .on_press(Message::TranslationBlocked(value.1.translation.clone()));
                suggested_translations_view = suggested_translations_view.push(
                    row![
                        text(&value.0),
                        text("->"),
                        text_input("", &value.1.translation)
                            .on_input(Message::DummyToMakeTextInputSelectable),
                        accept_button,
                        reset_button,
                        block_button
                    ]
                    .spacing(10),
                );
            }
        }

        let lookup_feature =
            text("For looking up existing words, please search the file in the data directory.");

        let proper_unicode_support = text("Iced does not properly support Unicode.
It will accept Latin and Greek in both input fields and text fields. It will accept Elder Futhark in input fields, but won't render it in text fields.
It crashes on Hebrew.
So, until either Iced implement proper Unicode support or I get around to replacing Iced with a web UI, all text fields that contain letters from the alphabet are input fields. This hard to read and even worse to copy pasted. But it's something at least...
And Hebrew is not supported.");

        let alphabet_input = text_input(
            "Which letter needs to be added to the alphabet?",
            &self.alphabet_input,
        )
        .on_input(Message::AlphabetInputChanged)
        .on_submit(Message::AddToAlphabet)
        .padding(15);

        let remove_alphabet_feature = text("For removing a character from the alphabet, please remove it from the file in the data directory.");

        let alphabet_text = text("The current alphabet is: ");

        let alphabet_value = text_input(
            "",
            &self
                .current_alphabet
                .iter()
                .map(|glyph| &glyph.symbol)
                .join(""),
        )
        .on_input(Message::DummyToMakeTextInputSelectable);

        let translation_column = column![
            input,
            translation,
            suggested_translations_view,
            lookup_feature,
            proper_unicode_support
        ]
        .spacing(20)
        .max_width(1200);

        let alphabet_column = column![
            alphabet_input,
            remove_alphabet_feature,
            alphabet_text,
            alphabet_value
        ]
        .spacing(20)
        .max_width(600);

        let body = row![translation_column, alphabet_column];

        let content = column![title, body].spacing(20).max_width(1800);

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
        self.suggested_translations = Vec::new();

        let mut translations = vec![];
        let mut suggested_translations = vec![];

        for word in self.input_value.split_whitespace() {
            match scrambler::translate_word(&word) {
                Ok(translation) => match scrambler::is_word_known(&word) {
                    true => translations.push(translation),
                    false => suggested_translations.push((word.to_owned(), translation)),
                },
                Err(error) => {
                    error!("{error}");
                }
            }
        }

        if suggested_translations.is_empty() {
            self.translated_value = Some(
                translations
                    .iter()
                    .map(|translation| &translation.translation)
                    .join(" "),
            );
        } else {
            self.suggested_translations = suggested_translations;
        }
    }
}
