


use axum::{http::{Request, StatusCode, header::SET_COOKIE, HeaderValue}, middleware::Next, response::Response, body::Body, Extension};
use axum_extra::extract::CookieJar;
use chrono::Utc;

use jsonwebtoken::{Validation, TokenData, decode, DecodingKey, errors::Error, Header, encode, EncodingKey};
use sea_orm::{DatabaseConnection, EntityTrait, DbErr};
use uuid::Uuid;
use serde:: {Serialize,Deserialize};

use crate::db::users::{self, Model};


#[derive(Serialize, Deserialize )]
pub struct AccessToken{
    id: Uuid, 
    exp: usize, 
  }

  fn encode_access_token(id:Uuid) -> Result<String, Error> {
    // let secret = "Jtso1AzmdRSglvM0OXXxQpcQUnM+k9qcq6dMnCL0mkY=";
    let secret = std::env::var("ACCESS_TOKEN").expect("ACCESS TOKEN SECRET COULDN'T BE LOADED FROM ENV");
    let expiration_date = (chrono::Utc::now() + chrono::Duration::seconds(60)).timestamp() as usize;
  
    let body = AccessToken { id: id, exp: (expiration_date) }; 
    let header = Header::new(jsonwebtoken::Algorithm::HS256); 
  
    let jwt = encode(&header, &body, &EncodingKey::from_secret(secret.as_ref())); 
    return jwt; 
  }
  




  fn decode_refresh_token(token:String) -> Result<TokenData<AccessToken>, jsonwebtoken::errors::Error>{
    // let secret = "Jtso1AzmdRSglvM0OXXxQpcQUnM+k9qcq6dMnCL0mkY=";
 let secret = std::env::var("REFRESH_TOKEN").expect("secret couldnt be loaded from env");
    let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256); 
    validation.leeway = 0;
    let token_data = decode::<AccessToken>(&token, &DecodingKey::from_secret(secret.as_ref()), &validation); 
    return token_data; 
}

fn decode_access_token(token:String) -> Result<TokenData<AccessToken>, jsonwebtoken::errors::Error>{
    // let secret = "Jtso1AzmdRSglvM0OXXxQpcQUnM+k9qcq6dMnCL0mkY=";
 let secret = std::env::var("ACCESS_TOKEN").expect("secret couldnt be loaded from env");
    let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256); 
    validation.validate_exp = false;
    let token_data = decode::<AccessToken>(&token, &DecodingKey::from_secret(secret.as_ref()), &validation); 
    return token_data; 
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
    Some(jwt) => {
        let data = decode_access_token(jwt); 
        
        // check if access token is expired
        match data {
            Ok(user_data) => {
               
               let validity: bool = time_validation(user_data.claims.exp);

               match validity {
                true => {
                    request.extensions_mut().insert(user_data.claims.id);
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
                                // user has a confirmed valid refresh token, create new access token and send it to frotnend 
                                let access_token = encode_access_token(user_data.claims.id); // if refresh token is still valid create a new access token and add it to the request cookie
                                match access_token {
                                    Ok(a_token) => {
                                        let header = format!("auth={}; HttpOnly", a_token);
                                        let header_value = HeaderValue::from_maybe_shared(header).expect("error converting cookie string to headervalue type");
                                        request.headers_mut().insert(SET_COOKIE, header_value);
                                       request.extensions_mut().insert(user_data.claims.id);
                                       Ok(next.run(request).await)
                                    }
                                    Err(error) => {
                                        println!("there was an error creating your access token: {}", error);
                                        Err(StatusCode::INTERNAL_SERVER_ERROR)
                                    }
                                }
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

