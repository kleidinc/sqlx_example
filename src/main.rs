#![allow(dead_code, unused_imports, unused_variables, clippy::all)]

use iced::widget::{button, column, container, row, text, text_input};
use iced::{Application, Element, Task, Theme};

// Add the theme
fn main() -> iced::Result {
    iced::application("Example App", ExampleApp::update, ExampleApp::view)
        .theme(ExampleApp::theme)
        .run_with(ExampleApp::new)
}

#[derive(Debug, Clone)]
struct ExampleApp {
    first_name: String,
    last_name: String,
}

#[derive(Debug, Clone)]
enum Message {
    OnChangeFirstName,
    OnChangeLastName,
    Save,
    SaveResult(Result<uuid::Uuid, Error>),
}

#[derive(Debug, Clone)]
enum Error {
    DbError,
    OtherError,
}

impl ExampleApp {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                first_name: String::new(),
                last_name: String::new(),
            },
            Task::none(),
        )
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        todo!()
    }

    fn view(&self) -> Element<Message> {
        todo!()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
