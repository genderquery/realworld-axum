use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(belongs_to(Article, foreign_key = article_id))]
#[diesel(belongs_to(User, foreign_key = author_id))]
#[diesel(table_name = crate::schema::comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Comment {
    pub id: i32,
    pub article_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub body: String,
    pub author_id: i32,
}
