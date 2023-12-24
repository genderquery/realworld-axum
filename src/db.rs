use sqlx::{Pool, Postgres};

#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

pub async fn get_user_by_username(
    pool: &Pool<Postgres>,
    username: &str,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(User, "select * from users where username = $1", username)
        .fetch_optional(pool)
        .await
}

pub async fn get_user_by_email(
    pool: &Pool<Postgres>,
    email: &str,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(User, "select * from users where email = $1", email)
        .fetch_optional(pool)
        .await
}

pub async fn create_user(
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

pub async fn update_user(
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
