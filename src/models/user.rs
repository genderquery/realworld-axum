use crate::schema::*;
use diesel::{
    backend::Backend,
    helper_types::{AsSelect, Select},
    prelude::*,
};

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub bio: String,
    pub image: String,
}

type All<DB> = Select<users::table, AsSelect<User, DB>>;

impl User {
    pub fn all<DB: Backend>() -> All<DB> {
        users::table.select(User::as_select())
    }

    pub fn from_username(
        conn: &mut PgConnection,
        username: &str,
    ) -> Result<Option<User>, diesel::result::Error> {
        Self::all()
            .filter(users::username.eq(username))
            .get_result(conn)
            .optional()
    }
}
