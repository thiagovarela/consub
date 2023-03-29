use accounts::Account;
use aide::{
    axum::{routing::get_with, ApiRouter, IntoApiResponse},
    transform::TransformOperation,
};
use axum::http::StatusCode;
use axum::{debug_handler, extract::State, Json};
use axum_extra::extract::Query;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use shared::{AppError, AppState};
use sqlx::PgPool;

use crate::{
    categories::Category,
    post_images::PostImage,
    posts::{Post, PostQuery},
};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PublicPost {
    #[serde(flatten)]
    pub post: Post,
    pub images: Vec<PostImage>,
    pub category: Option<Category>,
}

#[debug_handler]
pub async fn list_posts(
    State(pool): State<PgPool>, account: Account, Query(query): Query<PostQuery>,
) -> Result<impl IntoApiResponse, AppError> {
    let posts = {
        let mut conn = pool.acquire().await?;
        crate::posts::public_list_posts(&mut conn, account.id, query).await?
    };

    let post_images = {
        let mut conn = pool.acquire().await?;
        crate::post_images::list_post_images_by_post_ids(
            &mut conn,
            account.id,
            posts.iter().map(|p| p.id).collect(),
        )
        .await?
    };

    let post_categories = {
        let mut conn = pool.acquire().await?;
        crate::categories::get_category_by_ids(
            &mut conn,
            account.id,
            posts
                .iter()
                .filter_map(|p| p.category_id)
                .collect(),
        )
        .await?
    };

    let posts: Vec<PublicPost> = posts
        .into_iter()
        .map(|post| {
            let images = post_images
                .iter()
                .filter(|image| image.post_id == post.id)
                .cloned()
                .collect();
            if let Some(category_id) = post.category_id {
                let category = post_categories
                    .iter()
                    .find(|category| category.id == category_id)
                    .cloned();
                PublicPost {
                    post,
                    images,
                    category,
                }
            } else {
                PublicPost {
                    post,
                    images,
                    category: None,
                }
            }
        })
        .collect();

    Ok((StatusCode::OK, Json(posts)))
}

pub fn list_posts_docs(op: TransformOperation) -> TransformOperation {
    op.id("list_posts")
        .description("List posts")
        .response::<200, Json<Vec<PublicPost>>>()
        .tag("blogs")
}

pub fn public_routes(app_state: AppState) -> aide::axum::ApiRouter {
    ApiRouter::new()
        .api_route("/posts", get_with(list_posts, list_posts_docs))
        .with_state(app_state)
}
