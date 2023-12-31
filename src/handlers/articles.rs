use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};

use super::profiles::Profile;

#[debug_handler]
pub async fn get_articles() {
    todo!()
}

#[debug_handler]
pub async fn create_article() {
    todo!()
}

#[debug_handler]
pub async fn get_feed() {
    todo!()
}

#[debug_handler]
pub async fn get_article() {
    todo!()
}

#[debug_handler]
pub async fn update_article() {
    todo!()
}

#[debug_handler]
pub async fn delete_article() {
    todo!()
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewArticleRequest {
    article: NewArticle,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewArticle {
    title: String,
    description: String,
    body: String,
    tag_list: Option<Vec<String>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateArticleRequest {
    article: UpdateArticle,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateArticle {
    title: Option<String>,
    description: Option<String>,
    body: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MultipleArticlesResponse {
    articles: Vec<Article>,
    articles_count: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleArticleResponse {
    article: Article,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    slug: String,
    title: String,
    description: String,
    body: String,
    tag_list: Vec<String>,
    created_at: String,
    updated_at: String,
    favorited: bool,
    favorites_count: u64,
    author: Profile,
}
