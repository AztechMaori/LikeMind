use axum::{Extension, Json};
use axum_extra::extract::CookieJar;

use serde::Deserialize;
use sqlx::{PgPool, postgres::PgQueryResult};
use uuid::Uuid;

#[derive(Deserialize)]

pub struct UserData {
    email:String, 
    password: String, 
}

pub async fn trial(db: &PgPool, email: String)-> Result<Uuid, sqlx::Error>{
    let query = "SELECT id FROM users WHERE email = $1";
    let query:Result<Uuid, sqlx::Error> = sqlx::query_scalar(query).bind(email).fetch_one(db).await;
    return query;
}

pub async fn trial_2(db: &PgPool, id: Uuid)->Result<PgQueryResult, sqlx::Error>{
    let query = "UPDATE users SET refresh_token = $1 WHERE email = $2";

    let query = sqlx::query(query).bind("yahooo").bind("ash@gmail.com".to_string()).execute(db).await;
    return query;
}

pub async fn Gen(Extension(db):Extension<PgPool>){
let x = trial(&db, "ash@gmail.com".to_string()).await.unwrap();
println!("the first query worked:{}", x);
let y = trial_2(&db, x).await.unwrap();
println!("the second query worked, there may be a chance: {:?}", y);

}