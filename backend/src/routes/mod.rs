

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

mod test;
use test::trial; 


mod set_cooke;
use set_cooke::setcookie; 

mod validation;
use validation::time; 

mod authguard;
use authguard::auth_guard;

// mod yes;
// use yes::yessir;

// Access to fetch at 'http://localhost:3000/set' from origin 'http://localhost:4321' has been blocked by CORS policy: The 'Access-Control-Allow-Origin' header has a value 'http://localhost:4231' that is not equal to the supplied origin. Have the server send the header with a valid value, or, if an opaque response serves your needs, set the request's mode to 'no-cors' to fetch the resource with CORS disabled.


pub fn create_routes(database:DatabaseConnection) -> Router {


    
    let cors = CorsLayer::new()
    .allow_headers([AUTHORIZATION, COOKIE,ACCESS_CONTROL_ALLOW_CREDENTIALS, CONTENT_TYPE])
    .allow_methods([Method::GET, Method::POST])
    .allow_origin("http://localhost:4321".parse::<HeaderValue>().unwrap())
    .allow_credentials(true); 
    

    return Router::new()
    .route("/test", get(trial))
    .route_layer(middleware::from_fn(auth_guard))
    // .route("/yes", get(yessir))
    .route("/route", post(SignUp)) 
    .route("/set", get(setcookie))
    .route("/time", get(time))
    .layer(Extension(database))
    .layer(cors)

    

}