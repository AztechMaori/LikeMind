use axum::Json;
use serde::{Serialize,Deserialize};

#[derive(Serialize)]
pub struct Response {
    authenticated:bool,
}

#[derive(Deserialize)]
pub struct UserData{
    username: String,
    email: String, 
    password: String,
}



pub async fn SignUp(Json(user_data): Json<UserData>)  {
    println!("{},{},{}", user_data.username, user_data.email, user_data.password); 

}

