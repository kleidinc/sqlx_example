-- Add migration script here
CREATE TABLE IF NOT EXISTS "user" (
    user_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    first_name numeric(15),
    last_name numeric(25),
    telephone numeric(15)
)
