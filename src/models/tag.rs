use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tag {
    pub id: i32,
    pub tag: String,
}
