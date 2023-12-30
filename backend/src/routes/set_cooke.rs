use axum_extra::{extract::{cookie::Cookie, CookieJar}, headers::SetCookie, TypedHeader};



pub async fn setcookie (  jar:CookieJar) -> CookieJar{
// let cookie = Cookie::build("ash=asdudda").http_only(true);
let cookie = Cookie::build(("nelvimn","SMELVIN")).http_only(false);
jar.add(cookie)
}