use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(belongs_to(User, foreign_key = author_id))]
#[diesel(table_name = crate::schema::articles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Article {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub author_id: i32,
}
