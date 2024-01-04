


use axum::{http::{Request, StatusCode}, middleware::Next, response::Response, body::Body, Extension};
use axum_extra::extract::CookieJar;
use chrono::Utc;


use sea_orm::{DatabaseConnection, EntityTrait, DbErr};
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

fn validate_query(query:Result<Option<Model>,DbErr>)-> Result<String, StatusCode>{
    match query {
        Ok(data) => {
          match data {
            Some(user) => {
                let refresh_token = user.refresh_token;
                match refresh_token {
                    Some(token) => {
                        Ok(token)
                    }
                    None => {
                        println!("you have no refresh token");
                        Err(StatusCode::UNAUTHORIZED)
                        // redirect back to login for new refresh token

                    }
                }
            }
            None => {
                println!("User Model not found, create an account please!");
                Err(StatusCode::UNAUTHORIZED)
                // redirect back to signup to create an account;
            }
          }
        }
        Err(error) => {
            println!("there was an error retrieving user model from the database: {}", error);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    } 
    

}
pub async fn auth_guard(Extension(database): Extension<DatabaseConnection> ,jar: CookieJar, mut request:Request<Body>, next: Next)->Result<Response, StatusCode>{
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
                    let query = users::Entity::find_by_id(user_data.claims.id).one(&database).await;
                    let refresh_token = validate_query(query); // check if refresh token exists 
                    match refresh_token {
                        Ok(r_token) => {
                          let decoded_refresh = decode_refresh_token(r_token); // decode refresh token to check whether it is expired (possibly make this a manual check instead of decoding the entire token)
                          match decoded_refresh {
                            Ok(_) => {
                               let passed: Passed = Passed { userid: user_data.claims.id, send: true };
                               request.extensions_mut().insert(passed);
                               Ok(next.run(request).await)
                               
                            }
                            Err(error) => {
                                println!("there was an error parsing your refresh token: {}", error);
                                Err(StatusCode::UNAUTHORIZED)
                            }
                          }
                        }
                        Err(error) => {
                        Err(error)
                        }
                    }

                    
                }
               }
             

           
              
            }
            Err(error) => {
                println!("there was an error parsing your auth token: {}", error);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
    None => {
        println!("no auth token found, login required");
        Err(StatusCode::UNAUTHORIZED)
    }
}



}

