

use axum::
{routing::{get, post}, Router, http::{Method, header::{AUTHORIZATION, COOKIE, ACCESS_CONTROL_ALLOW_CREDENTIALS, CONTENT_TYPE}, HeaderValue}, Extension, middleware};

use sea_orm::DatabaseConnection;
use sqlx::{Pool, Postgres};
use tower_http::cors::{CorsLayer, Any, AllowCredentials, AllowOrigin, any};

//hello world 
mod sign_up;
use sign_up::SignUp; 

//JWT





mod authguard;
use authguard::auth_guard;

mod check;
use check::check;

mod x;
use x::Gen;

mod test;
use test::sql_test;


pub fn create_routes(database:Pool<Postgres>) -> Router {


    
    let cors = CorsLayer::new()
    .allow_headers([AUTHORIZATION, COOKIE,ACCESS_CONTROL_ALLOW_CREDENTIALS, CONTENT_TYPE])
    .allow_methods([Method::GET, Method::POST])
    .allow_origin("http://localhost:4321".parse::<HeaderValue>().unwrap())
    .allow_credentials(true); 
    

    return Router::new()
    .route("/check", get(check))
    .route_layer(middleware::from_fn(auth_guard))
    .route("/test", get(sql_test))
    .route("/login", post(Gen))
    .route("/route", post(SignUp)) 
   
    
    .layer(Extension(database))
    .layer(cors)

    

}