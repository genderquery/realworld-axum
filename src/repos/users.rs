use async_trait::async_trait;
use mockall::automock;

use super::error::RepoError;

#[async_trait]
#[automock]
pub trait UsersRepo {
    async fn get_user_by_id(&self, id: i64) -> Result<User, RepoError>;
    async fn get_user_by_username(&self, username: String) -> Result<User, RepoError>;
    async fn get_user_by_email(&self, email: String) -> Result<User, RepoError>;
    async fn create_user(&self, user: CreateUser) -> Result<User, RepoError>;
    async fn update_user(&self, user: UpdateUser) -> Result<User, RepoError>;
}

pub struct User {
    id: i64,
    username: String,
    email: String,
    password_hash: String,
    bio: String,
    image: String,
}

pub struct CreateUser {
    username: String,
    email: String,
    password_hash: String,
}

pub struct UpdateUser {
    username: String,
    email: String,
    password_hash: String,
    bio: String,
    image: String,
}
