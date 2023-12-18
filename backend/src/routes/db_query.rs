use axum::extract::State;
use sea_orm::{DatabaseConnection, ActiveModelTrait};
use sea_orm::ActiveValue::Set; 

use crate::db::jobs;

use super::AppState;




pub async fn create_job(State(database):State<DatabaseConnection>){
    let new_job = jobs::ActiveModel{
        job_id:Set(4), 
        job_title:Set("engineer".to_owned()),
        salary:Set(40000),
        user_id:Set(12),
        ..Default::default()
    };

    let response = new_job.insert(&database).await.unwrap();

    dbg!(response);
}


