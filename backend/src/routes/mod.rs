

use axum::
{routing::{get, post}, Router, http::Method, Extension};

use sea_orm::DatabaseConnection;
use tower_http::cors::{CorsLayer, Any};

//hello world 
mod hello_world;
use  hello_world::hello_world;

// JSON
mod json;
use json::{car_details,return_json};

// SeaORM 
mod db_query;
use db_query::create_job;
// use db_query::query_data; 




 



pub fn create_routes(database:DatabaseConnection) -> Router {

   

    
    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_origin(Any);



    return Router::new()
    .route("/", get(|| async { "wassup,bitches!" }))
    .route("/route", get(hello_world))
    .route("/json", get(return_json))
    .route("/car", get(car_details))
    .route("/database", get(create_job))
    .layer(cors)
    .layer(Extension(database))

    

}