#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    clippy::all,
    unreachable_patterns,
    irrefutable_let_patterns
)]

use iced::widget::{button, column, text, text_input};
use iced::{Application, Element, Task};
use sqlx::postgres::PgPool;
use sqlx::FromRow;
use std::sync::Arc;

fn main() -> iced::Result {
    iced::application("Example App", IcedExample::update, IcedExample::view)
        .run_with(IcedExample::new)
}
#[derive(Debug, Clone)]
struct IcedExample {
    pgpool: Option<Arc<PgPool>>,
    first_name: String,
    last_name: String,
    telephone: String,
}

#[derive(Debug, Clone)]
enum Error {}

#[derive(Debug, Clone)]
enum Message {
    SetupPoolResult(Result<PgPool, Error>),
    OnChangeFirstName(String),
    OnChangeLastName(String),
    OnChangeTelephone(String),
    Save,
    SaveResult(Result<uuid::Uuid, Error>),
}

impl IcedExample {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                pgpool: None,
                first_name: String::new(),
                last_name: String::new(),
                telephone: String::new(),
            },
            // We do this once when we launch the app
            Task::perform(launch_postgres_pool(), Message::SetupPoolResult),
        )
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SetupPoolResult(result) => {
                // Irrefutable let pattern ... change this
                if let Ok(result) = result {
                    self.pgpool = Some(Arc::new(result));
                    Task::none()
                } else {
                    println!("No connection to db, its needed to continue");
                    panic!();
                }
            }
            Message::OnChangeLastName(last_name) => {
                self.last_name = last_name;
                Task::none()
            }
            Message::OnChangeFirstName(first_name) => {
                self.first_name = first_name;
                Task::none()
            }
            Message::OnChangeTelephone(telephone) => {
                self.telephone = telephone;
                Task::none()
            }
            Message::SaveResult(result) => {
                todo!();
            }
            _ => Task::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        // we just will have a form on the left
        // and the results on the right ...
        // we will just use column with some rows
        todo!()
    }
}

async fn launch_postgres_pool() -> Result<PgPool, Error> {
    // TODO: seteup the postgres pgpool
    todo!()
}

async fn save(
    first_name: String,
    last_name: String,
    telephone: String,
    pgpool: Arc<PgPool>,
) -> Result<uuid::Uuid, Error> {
    // TODO: save the data to the db
    todo!()
}
