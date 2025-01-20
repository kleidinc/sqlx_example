use iced::widget::{button, column, row, text, text_input};
use iced::{Application, Element, Task, Theme};
use sqlx::postgres::PgPool;
use std::sync::Arc;

fm main() -> iced::Result {
    iced::application("ExampleApp", ExampleApp::update, ExampleApp::view)
        .theme(ExampleApp::theme)
        .run_with(ExampleApp::new)

}

#[derive(Debug, Clone)]
struct User {
    first_name: String,
    last_name: String,
    email_address: String,
    telephone_number: String,
}

impl User {
    fn new() -> Self {
        Self {
            first_name: String::new(),
            last_name: String::new(),
            email_address: String::new(),
            telephone_number: String::new(),
    }
    }
    fn user_clear(&mut self) {
        self.first_name.clear();
        self.last_name.clear();
        self.telephone_number.clear();
        self.email_address.clear();
    }
}

#[derive(Debug, Clone)]
struct ExampleApp {
    pgpool: Option<Arc<PgPool>>,
    user: User,
}

#[derive(Debug, Clone)]
enum Message {
    DbConnectionResult(Result<PgPool,Error>)
    OnChangeFirstName(String),
    OnChangeLastName(String),
    OnChangeTelephoneNumber(String),
    OnChangeEmailAddress(String),
    SaveUser,
    SaveUserResult(Result<uuid::Uuid, Error>),
}

#[derive(Debug, Clone)]
enum Error {
    DbError,
    IcedError,
    OtherError,
    SaveUserError,
}

impl ExampleApp {
    fn new() -> (Self, Task<Message>) {
        (
            Self{
                pgpool: None,
                user: User::new(),
        },
            Task::perform(connect_to_db(), Message::DbConnectionResult),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::DbConnectionResult(result) => {
                if let Ok(result) {
                    println!("We have liftoff : connected to the db");
                    self.pgpool = Some(Arc::new(result));
                    Task::none()
                } else {
                    panic!("We need a connection");
            },
            Message::OnChangeFirstName(first_name) => {
                self.user.first_name = first_name;
                Task::none()
            }
            Message::OnChangeLastName(last_name) => {
                self.user.last_name = last_name;
                Task::none()
            }
            Message::OnChangeEmailAddress(email_address) => {
                self.user.email_address = email_address;
                Task::none()
            }
            Message::OnChangeTelephoneNumber(telephone_number) => {
                self.user.telephone_number = telephone_number;
                Task::none()
            }
            Message::SaveUser => {
                Task::perform(save_user(self.user.first_name.clone(), self.user.last_name.clone(), 
                    self.user.telephone_number.clone(), self.user.email_address.clone(), Arc::clone(self.pgpool), Message::SaveUserResult)
            }
            Message::SaveUserResult(result) => {
                if let Ok(result) {
                    println!("The user had been save with user_id: {}", result);
                    // clear the user data
                    self.user.user_clear();
                    Task::none()
                }
            }
    }
        }
    }

    fn view(&self) -> Element<Message> {
        todo!()
    }

    fn theme(&self) -> Theme {
        Theme::Dracula
    }
}

async fn connect_to_db() -> Result<PgPool, Error> {
    let pgpool = PgPool::connect("postgres://alex:1234@localhost/icedform").await;
    if let Ok(pgpool) = pgpool {
       Ok(pgpool)
    } else {
        Err(Error::DbError)
    }
}

async fn save_user(
    first_name: String, 
    last_name: String, 
    telephone_number: String, 
    email_address: String, 
    pgpool: Arc<PgPool>) -> Result<uuid::Uuid, Error> {
    todo!();
}
