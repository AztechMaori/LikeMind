use jsonwebtoken::{TokenData, Validation, decode, DecodingKey, errors::Error, Header, encode, EncodingKey};
use uuid::Uuid;
use serde::{Serialize,Deserialize};




#[derive(Deserialize, Serialize)]
pub struct AccessToken{
  pub id: Uuid, 
  pub exp: usize, 
  pub regen:bool, 
}

#[derive(Deserialize, Serialize)]

pub struct RefreshToken {
  pub exp: usize,
}

//decoding tokens
pub fn decode_access_token(token:String) -> Result<TokenData<AccessToken>, jsonwebtoken::errors::Error>{
    // let secret = "Jtso1AzmdRSglvM0OXXxQpcQUnM+k9qcq6dMnCL0mkY=";
 let secret = std::env::var("ACCESS_TOKEN").expect("secret couldnt be loaded from env");
    let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256); 
    validation.validate_exp = false;
    let token_data = decode::<AccessToken>(&token, &DecodingKey::from_secret(secret.as_ref()), &validation); 
    return token_data; 
}

pub fn decode_refresh_token(token:String) -> Result<TokenData<AccessToken>, jsonwebtoken::errors::Error>{
    // let secret = "Jtso1AzmdRSglvM0OXXxQpcQUnM+k9qcq6dMnCL0mkY=";
 let secret = std::env::var("REFRESH_TOKEN").expect("secret couldnt be loaded from env");
    let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256); 
    validation.leeway = 0;
    let token_data = decode::<AccessToken>(&token, &DecodingKey::from_secret(secret.as_ref()), &validation); 
    return token_data; 
}

//encoding tokens
pub fn encode_access_token(id:Uuid, regen:bool) -> Result<String, Error> {
    // let secret = "Jtso1AzmdRSglvM0OXXxQpcQUnM+k9qcq6dMnCL0mkY=";
    let secret = std::env::var("ACCESS_TOKEN").expect("ACCESS TOKEN SECRET COULDN'T BE LOADED FROM ENV");
    let expiration_date = (chrono::Utc::now() + chrono::Duration::seconds(60)).timestamp() as usize;
  
    let body = AccessToken { id: id, exp: (expiration_date), regen:regen}; 
    let header = Header::new(jsonwebtoken::Algorithm::HS256); 
  
    let jwt = encode(&header, &body, &EncodingKey::from_secret(secret.as_ref())); 
    return jwt; 
  }
  
 
  
  
  