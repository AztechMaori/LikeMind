use axum_extra::{extract::CookieJar,TypedHeader } ;
use jsonwebtoken::{TokenData, Validation, decode, DecodingKey};
use sea_orm::sea_query::token;
use serde::{Serialize, Deserialize};
use uuid::Uuid; 

#[derive(Serialize, Deserialize )]
pub struct AccessToken{
    id: Uuid, 
    exp: usize, 
  }

fn decode_jwt_token(token:String) -> Result<TokenData<AccessToken>, jsonwebtoken::errors::Error>{
    let secret = "Jtso1AzmdRSglvM0OXXxQpcQUnM+k9qcq6dMnCL0mkY=";
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256); 
    
    let token_data = decode::<AccessToken>(&token, &DecodingKey::from_secret(secret.as_ref()), &validation); 
    return token_data; 
}

pub async fn get_cookie(jar:CookieJar){
let access_token = jar.get("auth").map(|cookie| cookie.value().to_owned()); 

match access_token {
    Some(a_token) => {
        println!("this is the token string: {}", a_token);
       let token_data =  decode_jwt_token(a_token);  
        
        match token_data {
            Ok(data) => {
                println!(" this is the id: {}", data.claims.id)
            }
            Err(error) => {
                println!("there was na error: {}", error);  
            }
        }
        
    }
    None => {
        println!("there is no token here"); 
    }
}
}

// let data = decode_jwt_token(cookie); 


