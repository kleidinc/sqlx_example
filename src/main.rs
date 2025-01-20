use iced::widget::{button, column, row, text, text_input};
use iced::{Application, Element, Task, Theme};
use sqlx::postgres::PgPool;
use std::sync::Arc;

fn main() -> iced::Result {
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
    DbConnectionResult(Result<PgPool, Error>),
    OnChangeFirstName(String),
    OnChangeLastName(String),
    OnChangeTelephoneNumber(String),
    OnChangeEmailAddress(String),
    SaveUser,
    SaveUserResult(Result<uuid::Uuid, Error>),
}

// TODO: fix this error handling
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
            Self {
                pgpool: None,
                user: User::new(),
            },
            Task::perform(connect_to_db(), Message::DbConnectionResult),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::DbConnectionResult(result) => {
                if let Ok(result) = result {
                    println!("We have liftoff : connected to the db");
                    self.pgpool = Some(Arc::new(result));
                    Task::none()
                } else {
                    panic!("We need a connection");
                }
            }
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
                let pgpool_ref = self.pgpool.as_ref().unwrap();
                let pgpool = Arc::clone(pgpool_ref);
                Task::perform(
                    save_user(
                        self.user.first_name.clone(),
                        self.user.last_name.clone(),
                        self.user.telephone_number.clone(),
                        self.user.email_address.clone(),
                        // You have to unwrap so you get access to the underlying
                        // Arc, which you can then clone by providing the reference
                        // you can't use unwrap because you are not allowed to move
                        pgpool,
                    ),
                    Message::SaveUserResult,
                )
            }
            Message::SaveUserResult(result) => {
                if let Ok(result) = result {
                    println!("The user had been save with user_id: {}", result);
                    // clear the user data
                    self.user.user_clear();
                    Task::none()
                } else {
                    println!("Not saved!{:?}", result);
                    Task::none()
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let form = column![
            text("Simple Form"),
            text_input("First Name", &self.user.first_name).on_input(Message::OnChangeFirstName),
            text_input("Last Name", &self.user.last_name).on_input(Message::OnChangeLastName),
            text_input("Telephone Number", &self.user.telephone_number)
                .on_input(Message::OnChangeTelephoneNumber),
            text_input("Email Address", &self.user.email_address)
                .on_input(Message::OnChangeEmailAddress),
            button("Save").on_press(Message::SaveUser),
        ];
        form.into()
    }

    fn theme(&self) -> Theme {
        Theme::Dracula
    }
}

async fn connect_to_db() -> Result<PgPool, Error> {
    let pgpool = PgPool::connect("postgres://alex:1234@localhost/dbexample").await;
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
    pgpool: Arc<PgPool>,
) -> Result<uuid::Uuid, Error> {
    println!("running save function");
    let rec = sqlx::query!(
        r#"
INSERT INTO "user"(first_name, last_name, email_address, telephone_number)
VALUES ($1, $2, $3, $4)
RETURNING user_id
        "#,
        first_name,
        last_name,
        email_address,
        telephone_number,
    )
    .fetch_one(&*pgpool)
    .await;
    if let Ok(rec) = rec {
        println!("We saved the user with id {:?}", rec.user_id);
        Ok(rec.user_id)
    } else {
        Err(Error::DbError)
    }
}

// async fn get_user_by_id(id: uuid::Uuid, pgpool: Arc<PgPool>) -> Result<User, Error> {
//     todo!()
// }
