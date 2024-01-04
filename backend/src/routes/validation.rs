use axum::{extract::Request, http::{header::COOKIE, HeaderMap}};
use axum_extra::extract::CookieJar;
use jsonwebtoken::{Validation, decode, TokenData, DecodingKey};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize )]
pub struct AccessToken{
    id: Uuid, 
    exp: usize, 
  }

use crate::utils::decode_access_token;


pub async fn time( jar: CookieJar){

let token = jar.get("auth").map(|x| x.value().to_owned());

match token {
  Some(data) => {
    println!("the token exists, here it is: {}", data);

  }
  None => {
    println!("THERE WAS NO TOKEN");
  }
}

}
