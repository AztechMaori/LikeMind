use axum::{Extension, response::Response, Json, http::{StatusCode,header::SET_COOKIE}};
use uuid::Uuid;

use super::jwt::encode_access_token;


#[derive(Clone)]
pub struct Passed {
    pub userid: Uuid, 
    pub send:bool,
}
  

// pub fn returns<T>(Extension(payload):Extension<Passed>, body: T)->Result<Response<T>, StatusCode> {

//     if payload.send == false{
//      let response = Response::builder().body(body).expect("there was an error composing the response");
//       Ok(response)
//     }else {
//         let access_token = encode_access_token(payload.userid, true);
       
//        match access_token {
//         Ok(a_token) => {
//          let response = Response::builder().header(SET_COOKIE, a_token).body(body);

//          match response {
//             Ok(res) => {
//                 println!("request has been succesfully made, the cookie will be sent to frontend");
//                 Ok(res)
//             }
//             Err(error) => {
//                print!("there was an error composing the response: {}", error);
//                Err(StatusCode::INTERNAL_SERVER_ERROR)
//             }
//          }
         
//         }
//         Err(error) => {
//          println!("there was an error encoding your access token: {}", error); 
//          Err(StatusCode::INTERNAL_SERVER_ERROR)
//         }
//        }
//     }
// }
