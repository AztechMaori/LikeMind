

use axum::
{routing::get, Router, http::Method};

use sea_orm::{DatabaseConnection, DbErr};
use tower_http::cors::{CorsLayer, Any};

//hello world 
mod hello_world;
use  hello_world::hello_world;

// JSON
mod json;
use json::{car_details,return_json};

// SeaORM 
mod db_query;
// use db_query::query_data; 

#[derive(Clone)]
pub struct AppState {
    pub database: DatabaseConnection, 
}

fn validate(database: Result<DatabaseConnection, DbErr>) -> Result<AppState, String> {
        
    
    match database {
        Ok(database) => {
            println!("Connection to the database was successful!");
            let db = AppState { database };
            Ok(db)
        }
        Err(error) => {
            println!("Unsuccessful connection to the database: {}", error);
            Err(format!("Error connecting to the database: {}", error))
        }
    }
}




pub fn create_routes(database:Result<DatabaseConnection, DbErr>) -> Router {

   

    
    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_origin(Any);

    return Router::new()
    .route("/", get(|| async { "wassup,bitches!" }))
    .route("/route", get(hello_world))
    .route("/json", get(return_json))
    .route("/car", get(car_details))
    .with_state(validate(database))
    .layer(cors);

    
}