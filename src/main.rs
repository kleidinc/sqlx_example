use anyhow::Error;
use futures::TryStreamExt;
use iced::widget::{button, column, row, text, text_input, Column};
use iced::{Application, Element, Task, Theme};
use sqlx::postgres::PgPool;
use std::sync::Arc;

fn main() -> iced::Result {
    iced::application("ExampleApp", ExampleApp::update, ExampleApp::view)
        .theme(ExampleApp::theme)
        .run_with(ExampleApp::new)
}

#[derive(Debug, thiserror::Error)]
enum LocalError {
    #[error(transparent)]
    IcedError(#[from] iced::Error),
    #[error(transparent)]
    DbError(#[from] sqlx::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

// The sqlx::FromRow is needed to convert Postgres types to Rust types, and back
// This is essential when you process result from SELECT clauses.
#[derive(Debug, sqlx::FromRow)]
pub struct User {
    user_id: uuid::Uuid,
    first_name: String,
    last_name: String,
    email_address: String,
    telephone_number: String,
}

impl User {
    fn new() -> Self {
        Self {
            user_id: uuid::Uuid::new_v4(),
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

#[derive(Debug)]
pub struct ExampleApp {
    pgpool: Option<Arc<PgPool>>,
    user: User,
    all_users: Option<Vec<User>>,
}

// NOTE: The enum message has to cloneable since the user can
// trigger the messages multiple times
#[derive(Debug, Clone)]
enum Message {
    DbConnectionResult(Result<PgPool, LocalError>),
    OnChangeFirstName(String),
    OnChangeLastName(String),
    OnChangeTelephoneNumber(String),
    OnChangeEmailAddress(String),
    SaveUser,
    SaveUserResult(Result<uuid::Uuid, LocalError>),
    ListAllUsers,
    ListAllUsersResult(Result<Vec<User>, LocalError>),
}

impl ExampleApp {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                pgpool: None,
                user: User::new(),
                all_users: None,
            },
            Task::perform(connect_to_db(), Message::DbConnectionResult),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::DbConnectionResult(result) => {
                if let Ok(result) = result {
                    self.pgpool = Some(Arc::new(result));
                    Task::perform(
                        get_all_users(Arc::clone(self.pgpool.as_ref().unwrap())),
                        Message::ListAllUsersResult,
                    )
                } else {
                    println!("We need a database connection! {:?}", result);
                    Task::none()
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
                Task::perform(
                    save_user(
                        self.user.first_name.clone(),
                        self.user.last_name.clone(),
                        self.user.telephone_number.clone(),
                        self.user.email_address.clone(),
                        // You have to unwrap so you get access to the underlying
                        // Arc, which you can then clone by providing the reference
                        // you can't use unwrap because you are not allowed to move
                        Arc::clone(self.pgpool.as_ref().unwrap()),
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
            Message::ListAllUsers => {
                println!("Request ListAllUsers");
                Task::perform(
                    // We need to first get a reference to the T inside the
                    // Option, and then unwrap it. We use the Arc::clone then
                    // to create a new reference to pgpool
                    get_all_users(Arc::clone(self.pgpool.as_ref().unwrap())),
                    Message::ListAllUsersResult,
                )
            }
            Message::ListAllUsersResult(result) => {
                println!("Listing all users");
                println!("Listing all users : {:?}", result);
                if let Ok(result) = result {
                    println!("----------------------------------------------------------------------------");
                    println!("The incoming users in ListAllUsersResult {:?}", result);
                    self.all_users = Some(result);
                } else {
                    println!("We couldn't process the result of the Vec<User> ");
                }
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        // let form = column![
        //     text("Simple Form"),
        //     text_input("First Name", &self.user.first_name).on_input(Message::OnChangeFirstName),
        //     text_input("Last Name", &self.user.last_name).on_input(Message::OnChangeLastName),
        //     text_input("Telephone Number", &self.user.telephone_number)
        //         .on_input(Message::OnChangeTelephoneNumber),
        //     text_input("Email Address", &self.user.email_address)
        //         .on_input(Message::OnChangeEmailAddress),
        //     button("Save").on_press(Message::SaveUser),
        //     button("Get All Users").on_press(Message::ListAllUsers),
        // ];
        // form.into()

        // We start with an empty vector of Type Element, which is
        // the lowest form of a widget, we should be able to push
        // widget::row's into it.
        // let mut all_users_vec: Vec<Element<Message>> = Vec::new();
        // if self.all_users.is_some() {
        //     for user in self.all_users.as_ref().unwrap().iter() {
        //         println!("The user being pushed in is {:?}", &user);
        //         all_users_vec.push(
        //             row![
        //                 text(&user.first_name),
        //                 text(&user.last_name),
        //                 text(&user.telephone_number),
        //                 text(&user.email_address)
        //             ]
        //             .into(),
        //         );
        //     }
        // }
        // We now need to publish it.
        // then we can use the from_vec function on the column of rows
        // to build an iced element to show on the screen
        // let all_users_component = Column::from_vec(all_users_vec);
        // column![form].into()
        let dashboard = column![
            row!(text("This is an experiment on messages")),
            row!(button("Click me").on_press(Message::ListAllUsers)),
        ];
        dashboard.into()
    }

    fn theme(&self) -> Theme {
        Theme::Dracula
    }
}

async fn connect_to_db() -> Result<PgPool, LocalError> {
    let result = PgPool::connect("postgres://alex:1234@localhost/dbexample").await?;
    Ok(result)
}

async fn save_user(
    first_name: String,
    last_name: String,
    telephone_number: String,
    email_address: String,
    pgpool: Arc<PgPool>,
) -> Result<uuid::Uuid, LocalError> {
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
    .await?;
    Ok(rec.user_id)
}

// TODO: Check if result is working
async fn get_all_users(pgpool: Arc<PgPool>) -> Result<Vec<User>, LocalError> {
    let mut users: Vec<User> = Vec::new();
    let mut incoming = sqlx::query_as::<_, User>(
        r#"
SELECT user_id, first_name, last_name, telephone_number, email_address FROM "user"
        "#,
    )
    .fetch(&*pgpool);

    while let Some(user_incoming) = incoming.try_next().await? {
        let _ = &users.push(user_incoming);
    }
    println!("The users being sent is {:?}", &users);
    Ok(users)
}
