

use axum::
{routing::{get, post}, Router, http::Method, Extension};

use sea_orm::DatabaseConnection;
use tower_http::cors::{CorsLayer, Any};

//hello world 
mod sign_up;
use sign_up::SignUp; 

//JWT

// SeaORM 
mod db_post;
use db_post::create_job;

mod db_get;
use db_get::get_user;

mod test;
use test::trial; 


 



pub fn create_routes(database:DatabaseConnection) -> Router {

   

    
    let cors = CorsLayer::new()
    .allow_headers(Any)
    .allow_methods([Method::GET, Method::POST])
    .allow_origin(Any);
    



    return Router::new()
    .route("/", get(trial))
    .route("/route", post(SignUp))
    .route("/database", get(create_job))
    .route("/user/:id", get(get_user))
    .layer(Extension(database))
    .layer(cors)

    

}