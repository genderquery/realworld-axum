use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(primary_key(article_id, tag_id))]
#[diesel(belongs_to(Article))]
#[diesel(belongs_to(Tag))]
#[diesel(table_name = crate::schema::article_tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ArticleTag {
    pub article_id: i32,
    pub tag_id: i32,
}
