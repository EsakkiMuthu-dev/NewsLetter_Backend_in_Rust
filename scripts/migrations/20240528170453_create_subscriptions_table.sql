-- Add migration script here
-- create subscripitons table
CREATE TABLE subscriptions(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    email TEXT UNIQUE NOT NULL,
    name TEXT UNIQUE NOT NULL,
    subscribed_at timestamptz NOT NULL
);