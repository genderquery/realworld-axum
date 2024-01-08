use async_trait::async_trait;
use mockall::automock;

use super::error::RepoError;

#[async_trait]
#[automock]
pub trait ProfilesRepo {
    async fn get_profile_by_username(&self, username: String) -> Result<Profile, RepoError>;
    async fn get_profile_by_username_as_user(
        &self,
        username: String,
        id: i64,
    ) -> Result<Profile, RepoError>;
    async fn follow_user(&self, username: String, as_user: i64);
    async fn unfollow_user(&self, username: String, as_user: i64);
}

pub struct Profile {
    username: String,
    email: String,
    password_hash: String,
    bio: String,
    image: String,
    following: bool,
}
