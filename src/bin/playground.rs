use conduit::models::follow::Follow;
use conduit::models::user::User;
use diesel::debug_query;
use diesel::pg::Pg;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use conduit::models::*;
use conduit::schema::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    let conn = &mut establish_connection();

    let (users_follower, users_followee) =
        diesel::alias!(users as users_follower, users as users_followee);

    let query = follows::table
        .inner_join(users_follower.on(follows::follower.eq(users_follower.field(users::id))))
        .inner_join(users_followee.on(follows::followee.eq(users_followee.field(users::id))))
        .select((
            users_follower.field(users::username),
            users_followee.field(users::username),
        ));

    let result = query.load::<(String, String)>(conn).unwrap();

    println!("{}", debug_query::<Pg, _>(&query));
}
