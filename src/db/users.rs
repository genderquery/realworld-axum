use sqlx::{Pool, Postgres};

pub async fn get_by_id(pool: &Pool<Postgres>, id: i32) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(User, "select * from users where id = $1", id)
        .fetch_optional(pool)
        .await
}

pub async fn get_by_username(
    pool: &Pool<Postgres>,
    username: &str,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(User, "select * from users where username = $1", username)
        .fetch_optional(pool)
        .await
}

pub async fn get_by_email(pool: &Pool<Postgres>, email: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(User, "select * from users where email = $1", email)
        .fetch_optional(pool)
        .await
}

pub async fn create(
    pool: &Pool<Postgres>,
    username: &str,
    email: &str,
    password_hash: &str,
) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        "insert into users (username, email, password_hash) values ($1, $2, $3) returning *",
        username,
        email,
        password_hash
    )
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &Pool<Postgres>,
    username: &str,
    email: &str,
    password_hash: &str,
    bio: Option<&str>,
    image: Option<&str>,
) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        "update users set username = $1, email = $2, password_hash = $3, bio = $4, image = $5 returning *",
        username,
        email,
        password_hash,
        bio,
        image
    )
    .fetch_one(pool)
    .await
}

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}
