use axum::{Extension, Json, http::StatusCode};
use axum_extra::extract::{CookieJar, cookie::Cookie};

use serde::{Deserialize, de};
use sqlx::{PgPool, postgres::PgQueryResult, prelude::FromRow};
use uuid::Uuid;

use crate::utils::{validate_password, encode_access_token, encode_refresh_token};


#[derive(Deserialize)]

pub struct UserData {
    email:String, 
    password: String, 
}
#[derive(FromRow)]

pub struct Details {
    hashed_password:String, 
    id: Uuid, 
    salt: String, 
}

pub async fn get_details_from_email(db: &PgPool, email: String) -> Result<Details, sqlx::Error>{
    let query = "SELECT hashed_password, id, salt FROM users WHERE email = $1";
    let details: Result<Details, sqlx::Error> = sqlx::query_as(query).bind(email).fetch_one(db).await;
    return details 
}

pub async fn update_refresh_token(db: &PgPool, id: Uuid, r_token: String)-> Result<PgQueryResult, sqlx::Error>{
   let query = "UPDATE users SET refresh_token = $1 WHERE id = $2";
   let result = sqlx::query(query).bind(r_token).bind(id).execute(db).await;
   return result;
}

pub async fn login(Extension(db):Extension<PgPool>, jar:CookieJar, Json(u_data):Json<UserData>)-> Result<CookieJar, StatusCode>{
let details = get_details_from_email(&db, u_data.email).await;

match details {
    Ok(details) => {
        let valid = validate_password(u_data.password, details.hashed_password, details.salt);
        if valid == true {
         let a_token = encode_access_token(details.id, false).expect("failure to create access token");
         let r_token = encode_refresh_token().expect("failure to create refresh token");
         let success = update_refresh_token(&db, details.id, r_token).await;
         match success {
            Ok(s) => {
                println!("refresh token succesfully renewed!: {:?}",s);
                let auth_cookie = Cookie::build(("auth", a_token)).http_only(false);
                Ok(jar.add(auth_cookie))
            }
            Err(error) => {
                println!("there was an error renewing your refresh token, please try again soon!: {}", error);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
         }
        }
        else {
            println!("you have entered invalid credentials");
            Err(StatusCode::UNAUTHORIZED)
        }
    }
    Err(error) => {
        println!("there was an error retrieving your credentials from the database: {}", error);
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}


}