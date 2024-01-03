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



fn decode_jwt_token(token:String) -> Result<TokenData<AccessToken>, jsonwebtoken::errors::Error>{
    let secret = "Jtso1AzmdRSglvM0OXXxQpcQUnM+k9qcq6dMnCL0mkY=";
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256); 
    
    let token_data = decode::<AccessToken>(&token, &DecodingKey::from_secret(secret.as_ref()), &validation); 
    return token_data; 
}

pub async fn time(jar:CookieJar, req: Request){
let access_token = jar.get("auth").map(|token| token.value().to_owned()); 

let token = req.headers().get(COOKIE);

match token {
  Some(data) => {

    let new = data.to_str().unwrap();
    
    println!("this is the cookie: {:?}", new.to_string());
  }
  None => {
    println!("nothing fiund");
  }
}


// match access_token {
//   Some(token) => {
//     println!("the token string is {}", token); 
//     let token_data = decode_jwt_token(token); 

//     match token_data {
//         Ok(data) => {
//             println!("the data is: {}", data.claims.exp);  
//             let time = chrono::Utc::now(); 
//             println!("{:?}", time); 
//         }
//         Err(error) => {
//             println!("there was an error: {}", error); 
//         }
//     }
//   }
//   None => {
//     println!("there was no token"); 
//   }
}
