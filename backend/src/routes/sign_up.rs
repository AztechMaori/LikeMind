use std::{time::{SystemTime, UNIX_EPOCH}};

use jsonwebtoken::{Header, encode,decode,  EncodingKey, TokenData, Validation, DecodingKey, errors::Error};
use axum::{Json, Extension, http::{StatusCode, header::{SET_COOKIE, COOKIE}, status}, response::{Response, Redirect, IntoResponse}, body::Body};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use serde::{Serialize,Deserialize};
use sqlx::{Pool, Postgres, postgres::PgQueryResult};
use crate::db::users;
use sea_orm::{ActiveValue::Set, DatabaseConnection, ActiveModelTrait}; 
use uuid::Uuid;
use argon2::{password_hash::{PasswordHasher, SaltString, rand_core::OsRng},Argon2};

use crate::utils::{encode_access_token,encode_refresh_token};
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

struct User{
  email:String, 
  username: String, 
  hashed_password:String, 
  salt:String, 
  id:Uuid, 
  refresh_token:String,
}


pub async fn create_user(user: User, db:&Pool<Postgres>) -> Result<PgQueryResult, sqlx::Error>{
let query = "INSERT INTO users (email, username, hashed_password, salt, id, refresh_token) VALUES ($1, $2, $3, $4, $5, $6)";

 let query = sqlx::query(query)
            .bind(user.email)
            .bind(user.username)
            .bind(user.hashed_password)
            .bind(user.salt)
            .bind(user.id)
            .bind(user.refresh_token)
            .execute(db)
            .await;
     
     return query;

   
}



pub async fn SignUp( Extension(database):Extension<Pool<Postgres>>, jar:CookieJar, Json(user_data): Json<UserData>) -> Result<CookieJar, StatusCode>{ 
  
  let id = Uuid::new_v4();

  let access_token = encode_access_token(id.clone(), false);

  match access_token {
    Ok(a_token) => {
      println!("the access token has been succesfully created: {}", a_token); 
      let auth_cookie = Cookie::build(("auth", a_token)).http_only(true); 
     


      let refresh_token = encode_refresh_token();

      match refresh_token {
        Ok(r_token) => {
          println!("the refresh token has been succesfully created: {}", r_token); 

       let salt = SaltString::generate(&mut OsRng); 
       let argon = Argon2::default(); 
       let hashed_pw = argon.hash_password(user_data.password.as_bytes(), &salt).expect("password hashing failed").to_string();  
    
       let user = User {
        email: user_data.email, 
        username: user_data.username, 
        hashed_password: hashed_pw, 
        salt: salt.to_string(), 
        id: id,
        refresh_token: r_token
       };

       let success = create_user(user, &database).await;


       match success {
           Ok(data) => {
            println!("the user has been succesfully created: {:?}", data);
            Ok(jar.add(auth_cookie))
           }
           Err(error) => {
            println!("there was an error creating the user: {}", error); 
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




