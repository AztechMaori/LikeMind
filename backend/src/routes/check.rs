use axum::{extract::Request, http::{header::SET_COOKIE, HeaderValue, StatusCode}, middleware::Next, Extension, response::Response};
use axum_extra::extract::{CookieJar, cookie::Cookie};


use crate::utils::{decode_access_token, encode_access_token};

use super::authguard::Passed;


pub async fn check(Extension(stuff): Extension<Passed>){

}