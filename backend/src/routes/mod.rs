

use axum::
{routing::{get, post}, Router, http::{Method, header::{AUTHORIZATION, COOKIE, ACCESS_CONTROL_ALLOW_CREDENTIALS, CONTENT_TYPE}, HeaderValue}, Extension, middleware};

use sea_orm::DatabaseConnection;
use tower_http::cors::{CorsLayer, Any, AllowCredentials, AllowOrigin, any};

//hello world 
mod sign_up;
use sign_up::SignUp; 

//JWT

// SeaORM 
 
// mod db_get;
// use db_get::get_user;



mod validation;
use validation::time; 

mod authguard;
use authguard::auth_guard;

mod check;
use check::check;


pub fn create_routes(database:DatabaseConnection) -> Router {


    
    let cors = CorsLayer::new()
    .allow_headers([AUTHORIZATION, COOKIE,ACCESS_CONTROL_ALLOW_CREDENTIALS, CONTENT_TYPE])
    .allow_methods([Method::GET, Method::POST])
    .allow_origin("http://localhost:4321".parse::<HeaderValue>().unwrap())
    .allow_credentials(true); 
    

    return Router::new()
    .route("/check", get(check))
    .route_layer(middleware::from_fn(auth_guard))
    .route("/route", post(SignUp)) 
    .route("/time", get(time))
    .layer(Extension(database))
    .layer(cors)

    

}