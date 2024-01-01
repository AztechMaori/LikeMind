use axum_extra::{extract::CookieJar,TypedHeader } ; 

pub async fn get_cookie(cookie:CookieJar){
println!("{:?}", cookie);

}