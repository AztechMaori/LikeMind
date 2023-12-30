use axum::{Json, Extension, http::{StatusCode, header::{SET_COOKIE, COOKIE}, status}, response::{Response, Redirect, IntoResponse}, body::Body};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use serde::{Serialize,Deserialize};
use crate::db::users;
use sea_orm::{ActiveValue::Set, DatabaseConnection, ActiveModelTrait}; 
use uuid::Uuid;
use argon2::{password_hash::{PasswordHasher, SaltString, rand_core::OsRng},Argon2};


#[derive(Deserialize)]
pub struct UserData{
    username: String,
    email: String, 
    password: String,
}





pub async fn SignUp( Extension(database):Extension<DatabaseConnection>, jar:CookieJar, Json(user_data): Json<UserData>) -> Result<Response<String>, StatusCode>{ 
  
  //create id. 
    let id = Uuid::new_v4();
    
  // hash password. 
   let salt = SaltString::generate(&mut OsRng); 
   let argon = Argon2::default(); 
   let hashed_pw = argon.hash_password(user_data.password.as_bytes(), &salt).expect("this shit aint work").to_string();  

    
    let new_user = users::ActiveModel{
        email: Set(user_data.email),
        username: Set(user_data.username), 
        hashed_password: Set(hashed_pw), 
        salt: Set(salt.to_string()), 
        id: Set(id.clone()),
        ..Default::default()
        }; 
    
     let user_created = new_user.insert(&database).await; 

      match user_created {
        Ok(_) => {
            println!("user was created succesfully"); 
           let res = Response::builder().header(SET_COOKIE, "tryhard=YOU; HttpOnly").body("hello".to_string()).unwrap(); 
         Ok(res)
            
        }
        Err(error) => {
            println!("{}", error); 
           Err(StatusCode::NOT_FOUND)

        }
      } 

    
  
} 




