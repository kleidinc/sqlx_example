**Still in alpha - work in progress :)**
This example will be part of the book ["Master Iced by building a Modern UI with Rust, Iced, SQLX and Postgres"](https://github.com/kleidinc/master_iced_book).

Here we build a small Iced App, which has input, and text fields, to showcase how to use SQLX with Postgres, alongside Iced, and tokio async,
to perform the most popular CRUD operations:

- CREATE TABLE
- INSERT INTO
- INSERT IF NOT EXISTS / else UPDATE
- SELECT
- UPDATE
- DELETE

### Prerequisites

You need a working Postgres database, preferably locally. Check the official Postgres docs on to install postgres on your system.
Put the URL in a `.env` file.

```bash
DATABASE_URL=postgres://<user>:<password>@localhost/icedexample
```

Then you can run the following commands, in your shell, to set up the database and load the sql in the migrations folder:

```bash
sqlx database create
sqlx migrate run
```

If your database already exists, you can run `sqlx database drop` first, and rerun the above commands.
