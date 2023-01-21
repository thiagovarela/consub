use accounts::authenticate_user_with_password;
use blogs::routes;
use shared::{testing::test_app, AppState};

async fn test_token(pool: &sqlx::PgPool) -> String {
    let access_token = authenticate_user_with_password(
        pool,
        uuid::uuid!("263b6188-aac6-45e9-9a2c-4728fdfd7ea1"),
        "thiagovarela@consub.io".into(),
        "123456".into(),
    )
    .await
    .unwrap();
    access_token.token
}


#[sqlx::test(migrations = "../../migrations", fixtures("account"))]
async fn test_create_a_category(pool: sqlx::PgPool) {
    let token = test_token(&pool).await;

    let address = test_app(routes(AppState { db_pool: pool })).await;

    let client = reqwest::Client::new();

    let url = format!("{}/admin/categories", address);
    let response = client
        .post(url)
        .header("X-API-KEY", "e6af50e6-0ef3-4908-80c0-a83622d96d03")
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({
            "name": "Test Category",
            "locale": "en-US",
        }))
        .send()
        .await
        .unwrap();    

    assert_eq!(response.status(), 201);
}

#[sqlx::test(migrations = "../../migrations", fixtures("account", "categories"))]
async fn test_update_a_category(pool: sqlx::PgPool) {
    let token = test_token(&pool).await;

    let address = test_app(routes(AppState { db_pool: pool })).await;

    let client = reqwest::Client::new();

    let url = format!("{}/admin/categories/{}", address, "e1c312c3-6d9b-42bc-ba15-a550971cdfa4");
    let response = client
        .patch(url)
        .header("X-API-KEY", "e6af50e6-0ef3-4908-80c0-a83622d96d03")
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({
            "name": "Testing",            
        }))
        .send()
        .await
        .unwrap();    
    
    let vl = response.json::<serde_json::Value>().await.unwrap();

    assert_eq!(vl["name"], "Testing");
}

#[sqlx::test(migrations = "../../migrations", fixtures("account", "categories"))]
async fn test_delete_category(pool: sqlx::PgPool) {
    let token = test_token(&pool).await;

    let address = test_app(routes(AppState { db_pool: pool })).await;

    let client = reqwest::Client::new();

    let url = format!("{}/admin/categories/{}", address, "e1c312c3-6d9b-42bc-ba15-a550971cdfa4");
    let response = client
        .delete(url)
        .header("X-API-KEY", "e6af50e6-0ef3-4908-80c0-a83622d96d03")
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({
            "name": "Testing",            
        }))
        .send()
        .await
        .unwrap();    
    
        assert_eq!(response.status(), 204);
}

