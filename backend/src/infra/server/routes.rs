use crate::infra::{
    ApiDoc, ServerState,
    http_api::{self},
    init_auth_layer, init_session_layer,
};
use axum::Router;
use http::{HeaderValue, Method};
use sqlx::{Pool, Postgres};
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::{Config, SwaggerUi};

pub async fn setup_router(pg_pool: &Pool<Postgres>) -> Router<ServerState> {
    //
    let auth_layer = init_auth_layer(pg_pool).await;
    let session_layer = init_session_layer(&pg_pool).await;
    let cors_layer = CorsLayer::new()
        // TODO: Set this to your actual frontend origin (Dioxus dev server, etc.).
        .allow_origin(HeaderValue::from_static("http://localhost:9010"))
        .allow_methods([Method::POST, Method::PUT, Method::OPTIONS])
        .allow_headers([http::header::CONTENT_TYPE, http::header::AUTHORIZATION]);

    // Swagger UI at /docs and OpenAPI JSON at /api-doc/openapi.json
    let swagger_ui = SwaggerUi::new("/docs")
        .url("/api-doc/openapi.json", ApiDoc::openapi())
        .config(Config::default().doc_expansion("none"));

    Router::new()
        .route("/auth/login", axum::routing::post(http_api::login))
        .route("/auth/logout", axum::routing::post(http_api::logout))
        .route("/auth/is_admin", axum::routing::get(http_api::has_admin_permissions))
        .route("/auth/password", axum::routing::put(http_api::update_user_password))
        .route("/user/profile", axum::routing::put(http_api::update_user_primary_info))
        .layer(auth_layer)
        .layer(session_layer)
        .layer(cors_layer)
        .merge(swagger_ui)
}
