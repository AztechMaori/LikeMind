use axum::{Extension, Json};
use axum_extra::extract::CookieJar;
use serde::Deserialize;
use sqlx::{Postgres, postgres::PgQueryResult, Pool, FromRow};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct jell{
  data:String, 
}

#[derive(FromRow)]
pub struct Details {
    hashed_password:String, 
    salt: String, 
    id:Uuid
}





pub async fn get_details(db: &Pool<Postgres>, email:String)-> Result<Details, sqlx::Error> {
  let query = "SELECT hashed_password, salt, id FROM users WHERE email = $1";
  let details = sqlx::query_as::<_,Details>(query).bind(email).fetch_one(db).await;
  return details
}

pub async fn update_refresh_token( db: &Pool<Postgres>, id:Uuid, r_token: String) -> Result<PgQueryResult, sqlx::Error>{
   
  let query = "UPDATE users SET refresh_token = $1 WHERE id = $2";

  let query = sqlx::query(query).bind(r_token).bind(id).execute(db).await;
  return query;
  }

  pub async fn trial_2(db: &Pool<Postgres>, id:Uuid)-> Result<String, sqlx::Error>{
    let query = "SELECT hashed_password FROM users where id = $1";
    let email = "ash@gmail.com".to_string();
  let query:Result<String, sqlx::Error> = sqlx::query_scalar(query).bind(id).fetch_one(db).await;
  return query;
  }


pub async fn sql_test(Extension(database): Extension<Pool<Postgres>>,jar:CookieJar, Json(data):Json<jell>){

let details = get_details(&database, "ash@gmail.com".to_string()).await.unwrap();
println!("the first query worked. this porves the login tiself it the problem{}", details.id);

let x = trial_2(&database, details.id).await.unwrap();

println!("the secnd query worked: {}", x);
}



  



