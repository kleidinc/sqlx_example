This is part of the book ["Master Iced by building a Modern UI with Rust, Iced, SQLX and Postgres"](https://github.com/kleidinc/master_iced_book).

Here we build a small Iced App, which has one input field, and one text field, to showcase how to use SQLX and Postgres, to do CRUD operations.

### Prerequisites

You need a working postgres database, preferably locally.
Put the url in a `.env` file.

```bash
DATABASE_URL=postgres://<user>:<password>@localhost/icedexample
```

Then you can run the following commands, in your shell, to set up the database and load the sql in the migrations folder:

```bash
sqlx database create
sqlx migrate run
```

If your database already exists, you can run `sqlx database drop` first, and rerun the above commands.
