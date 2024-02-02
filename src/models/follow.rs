use diesel::prelude::*;

use super::user::User;

#[derive(Debug, Queryable, Selectable, Identifiable, Associations)]
#[diesel(primary_key(follower, followee))]
#[diesel(belongs_to(User, foreign_key = follower, foreign_key = followee))]
#[diesel(table_name = crate::schema::follows)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Follow {
    pub follower: i32,
    pub followee: i32,
}
