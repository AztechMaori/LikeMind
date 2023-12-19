
use axum::Extension;
use sea_orm::{DatabaseConnection, ActiveModelTrait};
use sea_orm::ActiveValue::Set; 

use crate::db::users;





pub async fn create_job(Extension(database):Extension<DatabaseConnection>){
    let new_user = users::ActiveModel{
        email:Set("yowhattdup@gmail.com".to_string()),
        user_id:Set(12),
        username:Set("dantdm".to_string()),
        password_hash:Set("here_is_hash".to_string()),
        ..Default::default()
    };

    let response = new_user.insert(&database).await.unwrap();

    dbg!(response);
}


