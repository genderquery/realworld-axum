create table users (
    id              serial primary key,
    username        text not null unique,
    email           text not null unique,
    password_hash   text not null,
    bio             text,
    image           text
);
