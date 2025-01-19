#![allow(dead_code, unused_imports, unused_variables, clippy::all)]
use iced::{Application, Element, Task, Theme};

// Add the theme
fn main() -> iced::Result {
    iced::application("Example App", ExampleApp::update, ExampleApp::view)
        .theme(ExampleApp::theme)
        .run_with(ExampleApp::new)
}

#[derive(Debug, Clone)]
struct ExampleApp {}

#[derive(Debug, Clone)]
enum Message {}

impl ExampleApp {
    fn new() -> (Self, Task<Message>) {
        todo!()
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {}
    }

    fn view(&self) -> Element<Message> {
        todo!()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
