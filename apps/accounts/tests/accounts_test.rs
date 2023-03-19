use accounts::routes;
use shared::testing::{test_app, test_opendal_uploader};
use shared::AppState;

#[sqlx::test(migrations = "../../migrations")]
async fn test_create_a_valid_account(pool: sqlx::PgPool) {
    let opendal = test_opendal_uploader();
    let address = test_app(routes(AppState {
        db_pool: pool,
        opendal,
    }))
    .await;

    let client = reqwest::Client::new();

    let response = client
        .post(format!("{address}/"))
        .json(&serde_json::json!({
            "email": "fancy@website.com",
            "password": "consub123",
            "name": "Fancy Website",
            "subdomain": "fancy"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 201);
}

#[sqlx::test(migrations = "../../migrations", fixtures("account"))]
async fn test_fail_to_create_existing_subdomain(pool: sqlx::PgPool) {
    let opendal = test_opendal_uploader();
    let address = test_app(routes(AppState {
        db_pool: pool,
        opendal,
    }))
    .await;

    let client = reqwest::Client::new();

    let response = client
        .post(format!("{address}/"))
        .json(&serde_json::json!({
            "email": "fancy@website.com",
            "password": "consub123",
            "name": "Fancy Website",
            "subdomain": "consub"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 409);
}

#[sqlx::test(migrations = "../../migrations", fixtures("account"))]
async fn test_can_retrieve_access_token(pool: sqlx::PgPool) {
    let opendal = test_opendal_uploader();
    let address = test_app(routes(AppState {
        db_pool: pool,
        opendal,
    }))
    .await;

    let client = reqwest::Client::new();

    let response = client
        .post(format!("{address}/users/access-tokens/passwords"))
        .header("X-API-KEY", "e6af50e6-0ef3-4908-80c0-a83622d96d03")
        .json(&serde_json::json!({
            "email": "thiagovarela@consub.io",
            "password": "123456",
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 201);
}

#[sqlx::test(migrations = "../../migrations", fixtures("account"))]
async fn test_can_retrieve_access_token_with_subdomain(pool: sqlx::PgPool) {
    let opendal = test_opendal_uploader();
    let address = test_app(routes(AppState {
        db_pool: pool,
        opendal,
    }))
    .await;

    let client = reqwest::Client::new();

    let response = client
        .post(format!("{address}/users/access-tokens/passwords"))
        .header("X-FORWARDED-HOST", "consub.example.com")
        .json(&serde_json::json!({
            "email": "thiagovarela@consub.io",
            "password": "123456",
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 201);
}

#[sqlx::test(migrations = "../../migrations", fixtures("account"))]
async fn test_login_with_wrong_password(pool: sqlx::PgPool) {
    let opendal = test_opendal_uploader();
    let address = test_app(routes(AppState {
        db_pool: pool,
        opendal,
    }))
    .await;

    let client = reqwest::Client::new();

    let response = client
        .post(format!("{address}/users/access-tokens/passwords"))
        .header("X-FORWARDED-HOST", "consub.example.com")
        .json(&serde_json::json!({
            "email": "thiagovarela@consub.io",
            "password": "consub1234",
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 401);
}
