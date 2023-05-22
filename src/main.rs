use iced::widget::button;
use iced::widget::column;
use iced::widget::row;
use iced::widget::text;
use iced::widget::text_input;
use iced::Alignment;
use iced::Sandbox;
use iced::Settings;

fn main() -> iced::Result {
    println!("Hello");
    let res = Counter::run(Settings::default());
    println!("Goodbye");
    res
}

struct Counter {
    value: i32,
    input_value: String,
}

#[derive(Debug, Clone)]
enum Message {
    IncrementPressed,
    DecrementPressed,
    Reset,
    SetInput(String),
    SetButton,
}

impl iced::Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self {
            value: 0,
            input_value: "".to_string(),
        }
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
            Message::SetInput(value) => {
                self.input_value = value;
            }
            Message::SetButton => match self.input_value.parse::<i32>() {
                Ok(value) => {
                    self.value = value;
                }
                Err(error) => {
                    self.input_value = error.to_string();
                }
            },
            Message::Reset => {
                self.value = 0;
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        row![
            column![
                button("Increment").on_press(Message::IncrementPressed),
                text(self.value).size(50),
                button("Decrement").on_press(Message::DecrementPressed)
            ],
            column![
                text_input("Value of counter", &self.input_value)
                    .on_input(Message::SetInput)
                    .on_submit(Message::SetButton),
                button("Set").on_press(Message::SetButton),
                button("Reset").on_press(Message::Reset)
            ]
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
