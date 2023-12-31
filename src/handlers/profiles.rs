use axum_macros::debug_handler;
use serde::Serialize;

#[debug_handler]
pub async fn get_profile() {
    todo!()
}

#[debug_handler]
pub async fn follow() {
    todo!()
}

#[debug_handler]
pub async fn unfollow() {
    todo!()
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileResponse {
    profile: Profile,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    username: String,
    bio: String,
    image: String,
    following: bool,
}
