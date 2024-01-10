


use axum::{http::{Request, StatusCode}, middleware::Next, response::Response, body::Body, Extension};
use axum_extra::extract::CookieJar;
use chrono::Utc;


use sea_orm::{DatabaseConnection, EntityTrait, DbErr};
use sqlx::{Pool, Postgres, prelude::FromRow};
use uuid::Uuid;


use crate::db::users::{self, Model};
use crate::utils::{decode_access_token,decode_refresh_token};


#[derive(Clone)]
pub struct Passed {
    pub userid: Uuid, 
    pub send:bool,
}
  

fn time_validation(expiry: usize) -> bool {
    let current_time = Utc::now().timestamp();
   if current_time >= expiry.try_into().expect("conversion failed") {
    return false
   } else {
       return true
   }
}

pub async fn get_refresh_token(db:&Pool<Postgres>, id:Uuid )-> Result<String,sqlx::Error>{
    let query = "SELECT refresh_token FROM users WHERE id = $1";
    // let row = sqlx::query_as::<_,RToken>(query).bind(id).fetch_one(&db).await;
    let row: Result<String, sqlx::Error> = sqlx::query_scalar(query).bind(id).fetch_one(db).await;
    return row;
}




pub async fn auth_guard(Extension(database): Extension<Pool<Postgres>>,jar: CookieJar, mut request:Request<Body>, next: Next)->Result<Response, StatusCode>{
let token = jar.get("auth").map(|token| token.value().to_owned());


//check for access token existence
match token {
    Some(a_jwt) => {
        let data = decode_access_token(a_jwt.clone()); 
        
        // check if access token is expired
        match data {
            Ok(user_data) => {
               
               let validity: bool = time_validation(user_data.claims.exp);

               match validity {
                true => {
                    let passed: Passed = Passed { userid: user_data.claims.id, send:false };
                    request.extensions_mut().insert(passed);
                    Ok(next.run(request).await)
                }
                // if access token is expired query db for refresh token
                false => {
                   let data = get_refresh_token(&database, user_data.claims.id).await;
                    match data {
                        Ok(r_token) => {
                          let decoded_refresh = decode_refresh_token(r_token); // decode refresh token to check whether it is expired (possibly make this a manual check instead of decoding the entire token)
                          match decoded_refresh {
                            Ok(_) => {
                               let passed: Passed = Passed { userid: user_data.claims.id, send: true };
                               request.extensions_mut().insert(passed);
                               Ok(next.run(request).await)
                               
                            }
                            Err(error) => {
                                println!("there was an error parsing your refresh token,: {}", error);
                                Err(StatusCode::UNAUTHORIZED)
                            }
                          }
                        }
                        Err(error) => {
                            println!("there was an error retrieiving your refresh token: {}, please login again", error); 
                            Err(StatusCode::UNAUTHORIZED)
                        }
        
                    }

                    
                }
               }
             

           
              
            }
            Err(error) => {
                println!("there was an error decoding your auth token: {}", error);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
    None => {
        println!("no auth token found, login required"); // redirect to login page 
        Err(StatusCode::UNAUTHORIZED)
    }
}



}

