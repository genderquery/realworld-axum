use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(primary_key(user_id, article_id))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Article))]
#[diesel(table_name = crate::schema::favorites)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Favorite {
    pub user_id: i32,
    pub article_id: i32,
}
