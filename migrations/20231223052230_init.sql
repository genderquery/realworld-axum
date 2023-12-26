create table users (
    id              serial primary key,
    username        text not null unique,
    email           text not null unique,
    password_hash   text not null,
    bio             text,
    image           text
);

create table follows (
    user_id         integer not null references users on delete cascade,
    follows_user    integer not null references users on delete cascade,
    primary key(user_id, follows_user)
);
