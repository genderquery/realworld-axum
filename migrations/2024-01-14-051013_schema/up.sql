create table users (
    id              serial primary key,
    email           text not null,
    username        text not null,
    password_hash   text not null,
    bio             text not null,
    image           text not null
);

create unique index users_email_key on users (lower(email));
create unique index users_username_key on users (lower(username));

create table follows (
    follower        integer not null references users on delete cascade,
    followee        integer not null references users on delete cascade,
    primary key (follower, followee),
    check (follower <> followee)
);

create table articles (
    id              serial primary key,
    slug            text not null,
    title           text not null,
    description     text not null,
    body            text not null,
    created_at      timestamp with time zone not null default now(),
    updated_at      timestamp with time zone not null default now(),
    author_id       integer not null references users on delete cascade
);

create unique index articles_slug_key on articles (lower(slug));

create table tags (
    id              serial primary key,
    tag             text not null
);

create unique index tags_tag_key on tags (lower(tag));

create table article_tags (
    article_id      integer not null references articles on delete cascade,
    tag_id          integer not null references tags on delete cascade,
    primary key (article_id, tag_id)
);

create table favorites (
    user_id         integer not null references users on delete cascade,
    article_id      integer not null references articles on delete cascade,
    primary key (user_id, article_id)
);

create table comments (
    id              serial primary key,
    article_id      integer not null references articles on delete cascade,
    created_at      timestamp with time zone not null default now(),
    updated_at      timestamp with time zone not null default now(),
    body            text not null,
    author_id       integer not null references users on delete cascade
);
