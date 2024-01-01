use std::{time::{SystemTime, UNIX_EPOCH}};

use jsonwebtoken::{Header, encode,decode,  EncodingKey, TokenData, Validation, DecodingKey, errors::Error};
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

#[derive(Deserialize, Serialize)]
pub struct AccessToken{
  id: Uuid, 
  exp: usize, 
}

#[derive(Deserialize, Serialize)]

pub struct RefreshToken {
  exp: usize,
}

fn encode_refresh_token() -> Result<String, Error>{
let secret = "2upNVKJdnT0N2rSTF338ZcaiYsxxtEzmsHl4+RQwpqI="; 
let expiration_date = SystemTime::now().checked_add(std::time::Duration::from_secs(172800)).expect("overflow").duration_since(UNIX_EPOCH).expect("time has been incorrectly set").as_secs() as usize; 
let body = RefreshToken{exp: expiration_date};  
let header = Header::new(jsonwebtoken::Algorithm::HS256);  

let refresh_token = encode(&header, &body, &EncodingKey::from_secret(secret.as_ref())); 
return refresh_token; 
}

fn encode_access_token(id:Uuid) -> Result<String, Error> {
  let secret = "Jtso1AzmdRSglvM0OXXxQpcQUnM+k9qcq6dMnCL0mkY=";

  let expiration_date = SystemTime::now()
  .checked_add(std::time::Duration::from_secs(20))
  .expect("SystemTime overflow")
  .duration_since(UNIX_EPOCH)
  .expect("Time went backwards")
  .as_secs() as usize;

  let body = AccessToken { id: id, exp: expiration_date }; 
  let header = Header::new(jsonwebtoken::Algorithm::HS256); 

  let jwt = encode(&header, &body, &EncodingKey::from_secret(secret.as_ref())); 
  return jwt; 
}




pub async fn SignUp( Extension(database):Extension<DatabaseConnection>, jar:CookieJar, Json(user_data): Json<UserData>) -> Result<CookieJar, StatusCode>{ 
  
  let id = Uuid::new_v4();

  let access_token = encode_access_token(id.clone());

  match access_token {
    Ok(a_token) => {
      println!("the access token has been succesfully created: {}", a_token); 
      let cookie = Cookie::build(("auth", a_token)).http_only(true); 

      let refresh_token = encode_refresh_token();

      match refresh_token {
        Ok(r_token) => {
          println!("the refresh token has been succesfully created: {}", r_token); 

       let salt = SaltString::generate(&mut OsRng); 
       let argon = Argon2::default(); 
       let hashed_pw = argon.hash_password(user_data.password.as_bytes(), &salt).expect("password hashing failed").to_string();  
    
        
        let new_user = users::ActiveModel{
            email: Set(user_data.email),
            username: Set(user_data.username), 
            hashed_password: Set(hashed_pw), 
            salt: Set(salt.to_string()), 
            id: Set(id.clone()),
            refresh_token: Set(Some(r_token)),
            ..Default::default()
            }; 
        
         let user_created = new_user.insert(&database).await; 

         match user_created {
             Ok(_) => {
              println!("user has been succesfully created"); 
              Ok(jar.add(cookie))
            
             }
             Err(error) => {
              println!("there was an error, creating the user: {}", error); 
              Err(StatusCode::INTERNAL_SERVER_ERROR)
             }
         }
        }
        Err(error) => {
          println!("error creating refresh token: {}", error);
          Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
      }     
    }
    Err(error) => {
      println!("there was an error creating access token:{}", error);  
      Err(StatusCode::INTERNAL_SERVER_ERROR) 
    }
  }
} 




