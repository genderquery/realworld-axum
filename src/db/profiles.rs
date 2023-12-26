use sqlx::{postgres::PgQueryResult, Pool, Postgres};

pub async fn get_profile(
    pool: &Pool<Postgres>,
    user_id: i32,
) -> Result<Option<Profile>, sqlx::Error> {
    sqlx::query_as!(
        Profile,
        r#"select
            username,
            email,
            bio,
            image,
            false as "following!"
        from users
        where id = $1"#,
        user_id,
    )
    .fetch_optional(pool)
    .await
}

pub async fn get_profile_as_user(
    pool: &Pool<Postgres>,
    user_id: i32,
    as_user: i32,
) -> Result<Option<Profile>, sqlx::Error> {
    sqlx::query_as!(
        Profile,
        r#"select
            username,
            email,
            bio,
            image,
            ( select
                count(*) > 0
                from follows
                where user_id = $2 and
                follows_user = $1
            ) as "following!"
        from users
        where id = $1"#,
        user_id,
        as_user,
    )
    .fetch_optional(pool)
    .await
}

pub async fn follow_user(
    pool: &Pool<Postgres>,
    user_id: i32,
    as_user: i32,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        "insert into follows (user_id, follows_user) values ($1, $2)",
        as_user,
        user_id,
    )
    .execute(pool)
    .await
}

pub async fn unfollow_user(
    pool: &Pool<Postgres>,
    user_id: i32,
    as_user: i32,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        "delete from follows where user_id = $1 and follows_user = $2",
        as_user,
        user_id,
    )
    .execute(pool)
    .await
}

#[derive(Debug)]
pub struct Profile {
    pub username: String,
    pub email: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}
