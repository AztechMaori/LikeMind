use axum::{Extension, Json, extract::Path};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Serialize;

use crate::db::users;

#[derive(Serialize)]

pub struct Response{
    user_id:i32,
    username:String,
    password_hash:String,
}

#[derive(Serialize)]
pub struct ErrResponse {
    message: String,
    status_code: i32,
}

// pub async fn get_user(Path(id):Path<i32>, Extension(database):Extension<DatabaseConnection>) -> Result<Json<Response>, Json<ErrResponse>> {
//     let user = users::Entity::find_by_id(id).one(&database).await;

//     match user {
//         Ok(user) =>{
//             match user {
//                 Some(user) => {
//                     Ok(Json(Response{username:user.username,user_id:user.id,password_hash:user.password_hash}))
//                 }
//                 None => {
//                     Err(Json(ErrResponse { message: "sorry we couldn't find what you were looking for".to_string(), status_code: 404 }))
//                 }
//             }
//         }
//         Err(error) => {
//             Err(Json(ErrResponse { message: error.to_string(), status_code:404 }))
//         }
//     }
// }