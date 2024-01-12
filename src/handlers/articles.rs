use axum::{
    debug_handler,
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::error::AppError;

use super::profiles::Profile;

#[debug_handler]
pub async fn get_articles(
    State(pool): State<PgPool>,
) -> Result<Json<MultipleArticlesResponse>, AppError> {
    todo!()
}

#[debug_handler]
pub async fn create_article(
    State(pool): State<PgPool>,
    Json(payload): Json<NewArticleRequest>,
) -> Result<Json<SingleArticleResponse>, AppError> {
    todo!()
}

#[debug_handler]
pub async fn get_feed(
    State(pool): State<PgPool>,
) -> Result<Json<MultipleArticlesResponse>, AppError> {
    todo!()
}

#[debug_handler]
pub async fn get_article(
    State(pool): State<PgPool>,
    Path(slug): Path<String>,
) -> Result<Json<SingleArticleResponse>, AppError> {
    todo!()
}

#[debug_handler]
pub async fn update_article(
    State(pool): State<PgPool>,
    Path(slug): Path<String>,
    Json(payload): Json<UpdateArticleRequest>,
) -> Result<Json<SingleArticleResponse>, AppError> {
    todo!()
}

#[debug_handler]
pub async fn delete_article(Path(slug): Path<String>) -> Result<(), AppError> {
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
