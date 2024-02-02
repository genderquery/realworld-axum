use super::profiles::Profile;
use crate::{error::AppError, jwt::Claims, models, schema::*, AppState, PgPool};
use axum::{
    debug_handler,
    extract::{Path, Query, State},
    Json,
};
use chrono::NaiveDateTime;
use diesel::{connection::DefaultLoadingMode, dsl::count_star, prelude::*};
use serde::{Deserialize, Serialize};

#[debug_handler(state = AppState)]
pub async fn get_articles(
    State(pool): State<PgPool>,
    maybe_claims: Option<Claims>,
    Query(query): Query<ArticlesQuery>,
) -> Result<Json<MultipleArticlesResponse>, AppError> {
    let conn = &mut pool.get()?;

    let mut articles_query = articles::table
        .inner_join(users::table)
        .left_join(article_tags::table.left_join(tags::table))
        .left_join(favorites::table)
        .select((
            models::Article::as_select(),
            models::User::as_select(),
            tags::tag.nullable(),
        ))
        .offset(query.offset.unwrap_or(0))
        .limit(query.limit.unwrap_or(20))
        .order(articles::created_at.desc())
        .into_boxed();

    if let Some(tag) = query.tag {
        articles_query = articles_query.filter(tags::tag.eq(tag));
    }

    if let Some(author) = query.author {
        articles_query = articles_query.filter(users::username.eq(author));
    }

    if let Some(favorited_by) = query.favorited {
        let user_id = users::table
            .select(users::id)
            .filter(users::username.eq(favorited_by))
            .first::<i32>(conn)?;
        articles_query = articles_query.filter(favorites::user_id.eq(user_id));
    }

    let articles = articles_query
        .load_iter::<(models::Article, models::User, Option<String>), DefaultLoadingMode>(conn)?
        .map(|result| {
            let (article, author, _) = result?;
            let tag_list = article_tags::table
                .inner_join(tags::table)
                .select(tags::tag)
                .filter(article_tags::article_id.eq(article.id))
                .load::<String>(conn)?;
            let favorited = if let Some(claim) = maybe_claims {
                favorites::table
                    .select(count_star())
                    .filter(favorites::user_id.eq(claim.user_id))
                    .filter(favorites::article_id.eq(article.id))
                    .first::<i64>(conn)
                    .optional()?
                    .is_some()
            } else {
                false
            };
            let following = if let Some(claim) = maybe_claims {
                follows::table
                    .select(count_star())
                    .filter(follows::follower.eq(claim.user_id))
                    .filter(follows::followee.eq(author.id))
                    .first::<i64>(conn)
                    .optional()?
                    .is_some()
            } else {
                false
            };
            let favorites_count = favorites::table.select(count_star()).first(conn)?;
            Ok(Article {
                slug: article.slug,
                title: article.title,
                description: article.description,
                body: article.body,
                tag_list,
                created_at: article.created_at,
                updated_at: article.updated_at,
                favorited,
                favorites_count,
                author: Profile {
                    username: author.username,
                    bio: author.bio,
                    image: author.image,
                    following,
                },
            })
        })
        .collect::<Result<Vec<_>, diesel::result::Error>>()?;

    let articles_count = articles.len() as i64;

    Ok(Json(MultipleArticlesResponse {
        articles,
        articles_count,
    }))
}

#[debug_handler(state = AppState)]
pub async fn create_article(
    State(pool): State<PgPool>,
    claim: Claims,
    Json(payload): Json<NewArticleRequest>,
) -> Result<Json<SingleArticleResponse>, AppError> {
    todo!()
}

#[debug_handler(state = AppState)]
pub async fn get_feed(
    State(pool): State<PgPool>,
    claim: Claims,
    Query(query): Query<ArticlesFeedQuery>,
) -> Result<Json<MultipleArticlesResponse>, AppError> {
    todo!()
}

#[debug_handler(state = AppState)]
pub async fn get_article(
    State(pool): State<PgPool>,
    maybe_claims: Option<Claims>,
    Path(slug): Path<String>,
) -> Result<Json<SingleArticleResponse>, AppError> {
    todo!()
}

#[debug_handler(state = AppState)]
pub async fn update_article(
    State(pool): State<PgPool>,
    claim: Claims,
    Path(slug): Path<String>,
    Json(payload): Json<UpdateArticleRequest>,
) -> Result<Json<SingleArticleResponse>, AppError> {
    todo!()
}

#[debug_handler(state = AppState)]
pub async fn delete_article(
    State(pool): State<PgPool>,
    claim: Claims,
    Path(slug): Path<String>,
) -> Result<(), AppError> {
    todo!()
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticlesQuery {
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticlesFeedQuery {
    offset: Option<u64>,
    limit: Option<u64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewArticleRequest {
    pub article: NewArticle,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewArticle {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Option<Vec<String>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateArticleRequest {
    pub article: UpdateArticle,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateArticle {
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MultipleArticlesResponse {
    pub articles: Vec<Article>,
    pub articles_count: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleArticleResponse {
    pub article: Article,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub favorited: bool,
    pub favorites_count: i64,
    pub author: Profile,
}
