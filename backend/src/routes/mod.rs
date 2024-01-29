use axum::{
    http::{
        header::{ACCESS_CONTROL_ALLOW_CREDENTIALS, AUTHORIZATION, CONTENT_TYPE, COOKIE},
        HeaderValue, Method,
    },
    middleware,
    routing::{get, post},
    Extension, Router,
};
use sqlx::{Pool, Postgres};
use tower_http::cors::CorsLayer;

//hello world
mod sign_up;
use sign_up::sign_up;

//JWT

mod authguard;
use authguard::auth_guard;

mod check;
use check::check;

mod login;
use login::login;

mod logout;
use logout::logout;

pub fn create_routes(database: Pool<Postgres>) -> Router {
    let cors = CorsLayer::new()
        .allow_headers([
            AUTHORIZATION,
            COOKIE,
            ACCESS_CONTROL_ALLOW_CREDENTIALS,
            CONTENT_TYPE,
        ])
        .allow_methods([Method::GET, Method::POST])
        .allow_origin("http://localhost:4321".parse::<HeaderValue>().unwrap())
        .allow_credentials(true);

    return Router::new()
        .route("/check", get(check))
        .route_layer(middleware::from_fn(auth_guard))
        .route("/login", post(login))
        .route("/signup", post(sign_up))
        .route("/logout", post(logout))
        .layer(Extension(database))
        .layer(cors);
}

