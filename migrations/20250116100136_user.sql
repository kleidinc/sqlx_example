-- Add migration script here
CREATE TABLE IF NOT EXISTS "user" (
    user_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    first_name varchar(15),
    last_name varchar(25),
    email_address varchar(50),
    telephone_number varchar(15)
)
